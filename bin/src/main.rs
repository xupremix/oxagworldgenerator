use fltk;
use fltk::enums::ColorDepth;
use fltk::group::PackType;
use fltk::image::RgbImage;
use fltk::prelude::GroupExt;
use fltk::prelude::ImageExt;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::{app, enums::Color, frame::Frame, group::Pack, window::Window};
use lib_oxidizing_agents;
use lib_oxidizing_agents::world_generator::utilities::generate_random_seed;
use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;
use robotics_lib;
use robotics_lib::world::worldgenerator::Generator;

use lib_oxidizing_agents::utils::errors::OxAgError;
use lib_oxidizing_agents::world_generator::tile_type_spawn_levels::{
    OxAgTileTypeSpawnLevelPresets, OxAgTileTypeSpawnLevels,
};
use robotics_lib::world::tile::TileType::*;

fn main() {
    let size: usize = 1024;
    let seed = 20; // generate_random_seed();
    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_size(size)
        .set_tile_type_spawn_levels_from_preset(OxAgTileTypeSpawnLevelPresets::DEFAULT)
        .build();

    let tmp = generator.gen().0;

    let app = app::App::default();
    let mut wind = Window::default().with_size(size as i32, size as i32);
    let mut frame = Frame::default_fill();
    wind.make_resizable(true);
    wind.end();
    wind.show();
    frame.draw(move |f| {
        let mut fb: Vec<u8> = vec![0u8; (f.w() * f.h() * 4) as usize];
        for (iter, pixel) in fb.chunks_exact_mut(4).enumerate() {
            let x = iter % f.w() as usize;
            let y = iter / f.w() as usize;
            let color = match tmp[x][y].tile_type {
                DeepWater => Color::from_hex_str("#042B90"),
                ShallowWater => Color::from_hex_str("#08A5F3"),
                Sand => Color::from_hex_str("#F3CE08"),
                Grass => Color::from_hex_str("#57FF43"),
                Hill => Color::from_hex_str("#DC970D"),
                Mountain => Color::from_hex_str("#6F482A"),
                Snow => Color::from_hex_str("#FFFFFF"),
                _ => Color::from_hex_str("#000000"),
            };
            let color = color.unwrap().to_rgb();
            pixel.copy_from_slice(&[color.0, color.1, color.2, 255]);
        }
        let mut image = RgbImage::new(&fb, f.w(), f.h(), ColorDepth::Rgba8)
            .unwrap()
            .to_srgb_image()
            .unwrap();
        image.draw(f.x(), f.y(), f.width(), f.height());
    });

    app.run().unwrap();
}

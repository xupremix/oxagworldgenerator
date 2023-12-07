use std::time::{Duration, Instant};

use fltk::enums::ColorDepth;
use fltk::image::RgbImage;
use fltk::prelude::GroupExt;
use fltk::prelude::ImageExt;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::{app, enums::Color, frame::Frame, window::Window};
use robotics_lib::world::tile::Content::{Coin, Fire, Fish, Garbage, Rock, Tree};
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::worldgenerator::Generator;

use lib_oxidizing_agents::world_generator::presets::content_presets::OxAgContentPresets;
use lib_oxidizing_agents::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;

fn main() {
    let size: usize = 256;
    let seed = 75176; // generate_random_seed();
    let start = Instant::now();
    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_with_info(true)
        .build();

    let tmp = generator.gen().0;
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);

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
            let mut color = match tmp[x][y].tile_type {
                DeepWater => Color::from_hex_str("#042B90"),
                ShallowWater => Color::from_hex_str("#08A5F3"),
                Sand => Color::from_hex_str("#F3CE08"),
                Grass => Color::from_hex_str("#57FF43"),
                Hill => Color::from_hex_str("#DC970D"),
                Mountain => Color::from_hex_str("#6F482A"),
                Snow => Color::from_hex_str("#FFFFFF"),
                _ => Color::from_hex_str("#000000"),
            };
            if tmp[x][y].content.to_default() == Fire {
                color = Color::from_hex_str("#E22403");
            } else if tmp[x][y].content.to_default() == Tree(0) {
                color = Color::from_hex_str("#1D6004");
            } else if tmp[x][y].content.to_default() == Garbage(0) {
                color = Color::from_hex_str("#641FAF");
            } else if tmp[x][y].content.to_default() == Rock(0) {
                color = Color::from_hex_str("#2F2323");
            } else if tmp[x][y].content.to_default() == Fish(0) {
                color = Color::from_hex_str("#8F8EA1");
            } else if tmp[x][y].content.to_default() == Coin(0) {
                color = Color::from_hex_str("#000000");
            }
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

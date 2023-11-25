use std::fs::File;
use std::io::Write;
use std::process::Command;

use robotics_lib;

use lib_oxidizing_agents;
use lib_oxidizing_agents::utils::{gen_seed, OxAgError};
use lib_oxidizing_agents::worldgenerator::OxAgWorldGeneratorBuilder;

use fltk;
use fltk::enums::ColorDepth;
use fltk::group::PackType;
use fltk::image::RgbImage;
use fltk::prelude::GroupExt;
use fltk::prelude::ImageExt;
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::{app, enums::Color, frame::Frame, group::Pack, window::Window};

fn main() {
    let size = 512;
    let seed = gen_seed();
    let generator = OxAgWorldGeneratorBuilder::new().set_size(size).build();

    let tmp = generator.gen_map();

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
            let color = match tmp[x][y] {
                (-2.0..=-0.6) => Color::from_hex_str("#042B90"),
                (-0.6..=-0.4) => Color::from_hex_str("#08A5F3"),
                (-0.4..=-0.2) => Color::from_hex_str("#F3CE08"),
                (-0.2..=0.25) => Color::from_hex_str("#57FF43"),
                (0.25..=0.7) => Color::from_hex_str("#DC970D"),
                (0.7..=1.2) => Color::from_hex_str("#6F482A"),
                (1.2..=2.0) => Color::from_hex_str("#FFFFFF"),
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

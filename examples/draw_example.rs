use eframe::egui::{Rect, Vec2};
use eframe::emath::Pos2;
use eframe::{egui, emath};
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::world_generator::Generator;

const TILE_SIZE: f32 = 10.0;
const WORLD_SIZE: usize = 128;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([TILE_SIZE * WORLD_SIZE as f32, TILE_SIZE * WORLD_SIZE as f32]),
        ..Default::default()
    };
    eframe::run_native(
        "World Preview",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
    .unwrap()
}

struct MyApp {
    map: Vec<Vec<Tile>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            map: OxAgWorldGeneratorBuilder::new()
                .set_size(WORLD_SIZE)
                .set_content_options_from_preset(OxAgContentPresets::Default)
                .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
                .build()
                .unwrap()
                .gen()
                .0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for (row, rows) in self.map.iter().enumerate() {
            for (col, cell) in rows.iter().enumerate() {
                egui::CentralPanel::default().show(ctx, |ui| match cell.tile_type {
                    TileType::DeepWater => {
                        egui::Image::new(egui::include_image!("assets/DeepWater.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::ShallowWater => {
                        egui::Image::new(egui::include_image!("assets/ShallowWater.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Sand => {
                        egui::Image::new(egui::include_image!("assets/Sand.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Grass => {
                        egui::Image::new(egui::include_image!("assets/Grass.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Street => {
                        egui::Image::new(egui::include_image!("assets/Street.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Hill => {
                        egui::Image::new(egui::include_image!("assets/Hill.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Mountain => {
                        egui::Image::new(egui::include_image!("assets/Mountain.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Snow => {
                        egui::Image::new(egui::include_image!("assets/Snow.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Lava => {
                        egui::Image::new(egui::include_image!("assets/Lava.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Teleport(_) => {
                        egui::Image::new(egui::include_image!("assets/Teleport.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                    TileType::Wall => {
                        egui::Image::new(egui::include_image!("assets/Wall.png"))
                            .fit_to_exact_size(Vec2::new(TILE_SIZE, TILE_SIZE))
                            .paint_at(
                                &ui,
                                Rect::from_two_pos(
                                    Pos2::new(row as f32 * TILE_SIZE, col as f32 * TILE_SIZE),
                                    Pos2::new(
                                        row as f32 * TILE_SIZE + TILE_SIZE,
                                        col as f32 * TILE_SIZE + TILE_SIZE,
                                    ),
                                ),
                            );
                    }
                });
            }
        }
    }
}

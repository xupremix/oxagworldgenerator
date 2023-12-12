use eframe::egui;
use eframe::egui::{Rect, Ui, Vec2};
use eframe::emath::Pos2;
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::tile_type_options::OxAgTileTypeOptions;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::world_generator::Generator;

const TILE_SIZE: f32 = 7.0;
const WORLD_SIZE: usize = 120;

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([TILE_SIZE * WORLD_SIZE as f32, TILE_SIZE * WORLD_SIZE as f32]),
        ..Default::default()
    };
    eframe::run_native(
        "World Preview",
        options,
        Box::new(|cc| {
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
                .load("examples/save.json")
                .unwrap()
                .gen()
                .0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (row, rows) in self.map.iter().enumerate() {
                for (col, cell) in rows.iter().enumerate() {
                    handle_tile_type(ui, &cell.tile_type, row, col);
                    handle_tile_content(ui, &cell.content, row, col);
                }
            }
        });
    }
}

fn handle_tile_content(ui: &Ui, content: &Content, row: usize, col: usize) {
    match content {
        Content::Rock(_) => {
            egui::Image::new(egui::include_image!("assets/content/Rock.png"))
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
        Content::Tree(_) => {
            egui::Image::new(egui::include_image!("assets/content/Tree.png"))
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
        Content::Garbage(_) => {
            egui::Image::new(egui::include_image!("assets/content/Garbage.png"))
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
        Content::Fire => {
            egui::Image::new(egui::include_image!("assets/content/Fire.png"))
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
        Content::Coin(_) => {
            egui::Image::new(egui::include_image!("assets/content/Coin.png"))
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
        Content::Bin(_) => {
            egui::Image::new(egui::include_image!("assets/content/Bin.png"))
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
        Content::Crate(_) => {
            egui::Image::new(egui::include_image!("assets/content/Crate.png"))
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
        Content::Bank(_) => {
            egui::Image::new(egui::include_image!("assets/content/Bank.png"))
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
        Content::Water(_) => {}
        Content::Market(_) => {
            egui::Image::new(egui::include_image!("assets/content/Market.png"))
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
        Content::Fish(_) => {
            egui::Image::new(egui::include_image!("assets/content/Fish.png"))
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
        Content::Building => {
            egui::Image::new(egui::include_image!("assets/content/Building.png"))
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
        Content::Bush(_) => {
            egui::Image::new(egui::include_image!("assets/content/Bush.png"))
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
        Content::JollyBlock(_) => {
            egui::Image::new(egui::include_image!("assets/content/JollyBlock.png"))
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
        Content::Scarecrow => {
            egui::Image::new(egui::include_image!("assets/content/Scarecrow.png"))
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
        Content::None => {}
    }
}

fn handle_tile_type(ui: &Ui, tile_type: &TileType, row: usize, col: usize) {
    match tile_type {
        TileType::DeepWater => {
            egui::Image::new(egui::include_image!("assets/tiles/DeepWater.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/ShallowWater.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Sand.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Grass.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Street.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Hill.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Mountain.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Snow.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Lava.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Teleport.png"))
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
            egui::Image::new(egui::include_image!("assets/tiles/Wall.png"))
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
    }
}

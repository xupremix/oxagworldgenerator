use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Add, Fbm, Perlin};

const DIM: usize = 2;
const SEED: u32 = 42;
const X: usize = 256;
const Y: usize = 256;
const SEAMLESS: bool = true;
const XY_BOUND1: f64 = -1.0;
const XY_BOUND2: f64 = 1.0;
const FREQUENCY: f64 = 2.5;
const LACUNARITY: f64 = 2.0;
const PERSISTANCE: f64 = 0.6;
const OCTAVES: usize = 12;

// pub(crate) fn single_perlin_example() {
//     let fbm = Fbm::<Perlin>::new(SEED)
//         .set_frequency(FREQUENCY)
//         .set_lacunarity(LACUNARITY)
//         .set_octaves(OCTAVES)
//         .set_persistence(PERSISTANCE);
//
//     let mapbuilder = PlaneMapBuilder::<&Fbm<Perlin>, DIM>::new(&fbm)
//         .set_size(X, Y)
//         .set_x_bounds(XY_BOUND1, XY_BOUND2)
//         .set_y_bounds(XY_BOUND1, XY_BOUND2)
//         .set_is_seamless(SEAMLESS);
//
//     mapbuilder.build().write_to_file("out1.png");
// }
//
// pub(crate) fn add_2_perlin_example() {
//     let fbm = Fbm::<Perlin>::new(SEED)
//         .set_frequency(FREQUENCY)
//         .set_lacunarity(LACUNARITY)
//         .set_octaves(OCTAVES)
//         .set_persistence(PERSISTANCE);
//     let fbm2 = Fbm::<Perlin>::new(SEED * 2)
//         .set_frequency(FREQUENCY)
//         .set_lacunarity(LACUNARITY)
//         .set_octaves(OCTAVES)
//         .set_persistence(PERSISTANCE);
//
//     let fmb_final = Add::<f64, Fbm<Perlin>, Fbm<Perlin>, 2>::new(fbm, fbm2);
//
//     let mapbuilder = PlaneMapBuilder::new(fmb_final)
//         .set_size(X, Y)
//         .set_x_bounds(XY_BOUND1, XY_BOUND2)
//         .set_y_bounds(XY_BOUND1, XY_BOUND2)
//         .set_is_seamless(SEAMLESS);
//
//     let map = mapbuilder.build();
//
//     map.write_to_file("out.png");
// }

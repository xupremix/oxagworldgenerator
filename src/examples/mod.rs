use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Add, Fbm, Perlin};

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

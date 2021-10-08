use image::{DynamicImage, GrayImage, Luma};
use imageproc::drawing::draw_filled_circle_mut;
use rand::Rng;
//use rayon::prelude::*;

pub fn create_vignette(width: u32, height: u32, radius: u32) -> DynamicImage {
    let (center_x, center_y) = ((width / 2) as i32, (height / 2) as i32);
    // create canvas with given opacity
    let mut canvas = GrayImage::new(width, height);
    canvas
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Luma([0u8]));

    // partial radius should be gradient
    let inner_radius = (radius as f32 * 0.33).round() as u32;
    let outer_radius = radius - inner_radius;
    // draw clear center
    draw_filled_circle_mut(
        &mut canvas,
        (center_x, center_y),
        inner_radius as i32,
        Luma([255u8]),
    );
    // draw gradient
    let alpha_step = 255.0 / outer_radius as f32;
    (1..=outer_radius)
        //.step_by(radius_step)
        .into_iter()
        .for_each(|i| {
            let alpha = (alpha_step * i as f32).round() as u8;
            draw_filled_circle_mut(
                &mut canvas,
                (center_x, center_y),
                (radius - i) as i32,
                Luma([alpha.clamp(0u8, 255u8)]),
            );
        });
    // add noise to gradient

    // let mut rng = rand::thread_rng();
    // canvas.iter_mut().for_each(|p| {
    //     if *p != 0 && *p != 255 {
    //         //let mut rng = rand::thread_rng();
    //         let random: i32 = *p as i32 + rng.gen_range(-10..10);
    //         if random < 0 {
    //             *p = 0
    //         } else {
    //             *p = random.clamp(0, 255) as u8
    //         }
    //     }
    // });

    // canvas.par_iter_mut().for_each_init(
    //     || rand::thread_rng(),
    //     |rng, p| {
    //         if *p != 0 && *p != 255 {
    //             //let mut rng = rand::thread_rng();
    //             let random: i32 = *p as i32 + rng.gen_range(-10..10);
    //             if random < 0 {
    //                 *p = 0
    //             } else {
    //                 *p = random.clamp(0, 255) as u8
    //             }
    //         }
    //     },
    // );

    DynamicImage::ImageLuma8(canvas)
}

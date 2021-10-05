use fltk::image::RgbImage;
use image::{
    imageops::overlay, DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Pixel, Rgb,
};
use imageproc::drawing::draw_filled_circle_mut;
use rand::Rng;
use std::option::Option;

pub fn process_image(
    image: &DynamicImage,
    radius: u32,
    opacity: u8,
    preview_mode: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius, opacity, preview_mode);
    let mut base_image = image.clone();
    //overlay(&mut base_image, &vignette, 0, 0);
    //base_image.to_rgb8()
    darken(base_image, vignette).to_rgb8()
}

pub fn darken(base_image: DynamicImage, top_image: DynamicImage) -> DynamicImage {
    let mut base = base_image.into_rgb8();
    let (w, h) = base.dimensions();
    let top = top_image.into_rgb8();
    let strength = 1.0;
    base.pixels_mut()
        .into_iter()
        .zip(top.pixels().into_iter())
        .for_each(|(b, t)| b.apply(|c| if c > t[0] { c - t[0] } else { c }));
    let result = base;
    DynamicImage::ImageRgb8(result)
}

pub fn create_vignette(
    width: u32,
    height: u32,
    radius: u32,
    opacity: u8,
    preview_mode: bool,
) -> DynamicImage {
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
    let mut rng = rand::thread_rng();
    canvas.pixels_mut().into_iter().for_each(|px| {
        if px[0] != 0u8 && px[0] != 255u8 {
            let random: i32 = px[0] as i32 + rng.gen_range(-1..1);
            if random < 0 {
                px[0] = 0
            } else {
                px[0] = random.clamp(0, 255) as u8
            }
        }
    });
    //canvas = blur(&canvas, 0.1);
    DynamicImage::ImageLuma8(canvas)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

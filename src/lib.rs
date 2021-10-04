use image::{
    imageops::{blur, overlay},
    DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage,
};
use imageproc::drawing::draw_filled_circle_mut;

pub fn preview(mut image: DynamicImage, radius: i32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius, false);
    overlay(&mut image, &vignette, 0, 0);
    image.to_rgb8()
}

pub fn create_vignette(width: u32, height: u32, radius: i32, blur_gradient: bool) -> DynamicImage {
    let strength = 155; // add to slider 1 .. 255

    let (center_x, center_y) = ((width / 2) as i32, (height / 2) as i32);
    // create canvas with given opacity
    let mut canvas = RgbaImage::new(width, height);
    canvas
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Rgba([0u8, 0u8, 0u8, strength]));
    // half radius should be gradient
    let half_radius = radius / 2;
    draw_filled_circle_mut(
        &mut canvas,
        (center_x, center_y),
        half_radius,
        Rgba([0u8, 0u8, 0u8, 0u8]),
    );
    // create gradient for second half of radius
    let alpha_step = if half_radius >= strength as i32 {
        1
    } else {
        (strength as f32 / half_radius as f32).ceil() as u32
    };
    // draw gradient
    (1..=half_radius).rev().into_iter().for_each(|i| {
        draw_filled_circle_mut(
            &mut canvas,
            (center_x, center_y),
            half_radius - 1 + i,
            Rgba([
                0u8,
                0u8,
                0u8,
                (alpha_step * i as u32).clamp(0, strength as u32) as u8,
            ]),
        );
    });
    // optional blur to make gradient smooter
    if blur_gradient {
        canvas = blur(&canvas, 10.0);
    }
    DynamicImage::ImageRgba8(canvas)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

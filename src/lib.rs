use image::{
    imageops::{blur, overlay},
    DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage,
};
use imageproc::drawing::draw_filled_circle_mut;

pub fn preview(image: &DynamicImage, radius: u32, opacity: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius, opacity, true);
    let mut base_image = image.clone();
    overlay(&mut base_image, &vignette, 0, 0);
    base_image.to_rgb8()
}

pub fn create_vignette(
    width: u32,
    height: u32,
    radius: u32,
    opacity: u8,
    preview: bool,
) -> DynamicImage {
    let (center_x, center_y) = ((width / 2) as i32, (height / 2) as i32);
    // create canvas with given opacity
    let mut canvas = RgbaImage::new(width, height);
    canvas
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Rgba([0u8, 0u8, 0u8, opacity]));
    // partial radius should be gradient
    let inner_radius = (radius as f32 * 0.33).round() as u32;
    let outer_radius = radius - inner_radius;
    // draw clear center
    draw_filled_circle_mut(
        &mut canvas,
        (center_x, center_y),
        inner_radius as i32,
        Rgba([0u8, 0u8, 0u8, 0u8]),
    );
    // create gradient for second half of radius
    let radius_step: usize = if outer_radius as usize > opacity as usize {
        outer_radius as usize / opacity as usize
    } else {
        1
    };
    let alpha_step = opacity as f32 / outer_radius as f32;
    // println!(
    //     "innerradius:{},outerradius:{},radius_step:{},alpha_step:{},opacity:{}",
    //     inner_radius, outer_radius, radius_step, alpha_step, opacity
    // );

    // draw gradient
    (1..=outer_radius)
        .rev()
        .step_by(radius_step)
        .into_iter()
        .for_each(|i| {
            let alpha = (alpha_step * i as f32).round() as u8;
            println!("alpha:{}, radius:{}, i:{}", alpha, inner_radius + i, i);
            draw_filled_circle_mut(
                &mut canvas,
                (center_x, center_y),
                (inner_radius + i) as i32,
                Rgba([0u8, 0u8, 0u8, alpha.clamp(0u8, 255u8)]),
            );
        });
    // blur to make gradient smooter
    if preview {
        canvas = blur(&canvas, 1.0);
    } else {
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

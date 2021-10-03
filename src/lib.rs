use image::{
    imageops::{blur, overlay},
    DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage,
};
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_circle_mut};

pub fn preview(mut image: DynamicImage, radius: i32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let (center_x, center_y) = ((width / 2) as i32, (height / 2) as i32);
    let mut canvas = RgbaImage::new(width, height);
    canvas
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Rgba([0u8, 0u8, 0u8, 255u8]));
    let inner = radius / 2;
    draw_filled_circle_mut(
        &mut canvas,
        (center_x, center_y),
        inner,
        Rgba([0u8, 0u8, 0u8, 0u8]),
    );
    let alpha_step = if inner >= 255 {
        1
    } else {
        (255.0 / inner as f32).ceil() as u32
    };

    println!("radius:{}", radius);
    println!("alpha:{}", alpha_step);
    (1..=inner).rev().into_iter().for_each(|i| {
        draw_filled_circle_mut(
            &mut canvas,
            (center_x, center_y),
            inner + i,
            Rgba([0u8, 0u8, 0u8, (alpha_step * i as u32).clamp(0, 255) as u8]),
        );
    });
    overlay(&mut image, &blur(&canvas, 2.0), 0, 0);
    image.to_rgb8()
}

pub fn create_vignette(width: u32, height: u32, radius: i32) -> DynamicImage {
    let outer_box = ((width * width + height * height) as f64).sqrt().ceil() as u32;
    let mut vignette = RgbaImage::new(outer_box, outer_box);
    let center = (outer_box / 2) as i32;
    // println!(
    //     "Image: {}x{}, Outer Box:{}, Center:{}",
    //     width, height, outer_box, center
    // );
    vignette
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Rgba([0u8, 0u8, 0u8, 100u8]));
    draw_filled_circle_mut(
        &mut vignette,
        (center, center),
        radius,
        Rgba([255u8, 255u8, 255u8, 0u8]),
    );
    //blur(&vignette, 30.0);
    DynamicImage::ImageRgba8(vignette)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

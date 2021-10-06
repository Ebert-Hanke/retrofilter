use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
mod vignette;
use vignette::create_vignette;

pub fn process_image(
    image: &DynamicImage,
    radius: u32,
    // fix this in gui
    _opacity: u8,
    _preview_mode: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius);
    let mut base_image = image.clone();
    //overlay(&mut base_image, &vignette, 0, 0);
    //base_image.to_rgb8()
    darken(base_image, vignette).to_rgb8()
}

pub fn darken(base_image: DynamicImage, top_image: DynamicImage) -> DynamicImage {
    let mut base = base_image.to_rgb8();
    //  let mut base_raw = base_image.into_bytes();
    //  let color_buffer: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(&mut base_raw);
    //  let base: Hsv = color_buffer.into_color();
    // let (w, h) = base.dimensions();
    let top = top_image.into_luma8();
    let (w, h) = base.dimensions();

    todo!()
    // let strength = 1.0;
    // base.pixels_mut()
    //     .into_iter()
    //     .zip(top.pixels().into_iter())
    //     .for_each(|(b, t)| {
    //         let base: Hsv = Srgb::from(b);
    //         let top: Hsv = Srgb::from(t);
    //         let blend = base.blend(b, multiply);
    //         let mut blend = Rgb([0, 0, 0]);
    //         (0..3).into_iter().for_each(|i| {
    //             if b[i] > t[0] {
    //                 blend[i] = ((b[i] as u32 + t[0] as u32) / 2).clamp(0, 255) as u8;
    //             } else {
    //                 blend[i] = b[i];
    //             }
    //         });
    //         *b = blend
    //     });
    // let result = base;
    // DynamicImage::ImageRgb8(result)
}

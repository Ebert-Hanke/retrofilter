use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb};
mod vignette;
use palette::{Hsl, Hue, IntoColor, Pixel, Saturate, Shade, Srgb};
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
    let mut base_image = image.clone().to_rgb8();
    darken(&mut base_image, vignette);
    base_image
}

pub fn darken(base_image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, top_image: DynamicImage) {
    base_image.pixels_mut().into_iter().for_each(|px| {
        // decode from raw to palette Srgb
        let pp = Srgb::from_raw(&px.0);
        // convert to Hsl
        let hsl: Hsl = pp.into_format().into_color();
        // modify
        let shift = hsl.shift_hue(80.0);
        // convert back to Srgb
        let rgb: Srgb = shift.into_color();
        // encode back to raw
        let raw: [u8; 3] = Srgb::into_raw(rgb.into_format());
        px.0 = raw;
    });
}

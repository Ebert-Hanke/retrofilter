use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
mod vignette;
use palette::{Blend, Pixel, Srgb, WithAlpha};
use rayon::prelude::*;
use vignette::create_vignette;

pub fn process_image(
    image: &DynamicImage,
    radius: u32,
    opacity: f32,
    // fix this in gui
    _preview_mode: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius, true);
    let mut base_image = image.clone().to_rgb8();
    darken(&mut base_image, &vignette, opacity);
    base_image
}

pub fn darken(
    base_image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    top_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    alpha: f32,
) {
    let base: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(base_image);
    let top: &[Srgb<u8>] = Pixel::from_raw_slice(top_image);
    base.par_iter_mut().zip(top).for_each(|(color1, color2)| {
        let color1_alpha = color1.into_format().into_linear().opaque();
        let color2_alpha = color2.into_format().into_linear().with_alpha(alpha);

        let blended = color2_alpha.multiply(color1_alpha);

        *color1 = blended.color.into_encoding().into_format();
    });
}

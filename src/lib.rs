use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
mod vignette;
use palette::{Blend, LinSrgba, Pixel, Srgb, WithAlpha};
use rayon::prelude::*;
use vignette::create_vignette;

pub fn process_image(
    image: &DynamicImage,
    radius: u32,
    alpha: f32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let vignette = create_vignette(width, height, radius, true);
    let mut base_image = image.clone().to_rgb8();
    palette_blend(&mut base_image, &vignette, alpha, |c1, c2| c1.multiply(c2));
    base_image
}

pub fn palette_blend<F>(
    base_image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    top_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    alpha: f32,
    blend_fn: F,
) where
    F: Fn(LinSrgba, LinSrgba) -> LinSrgba + std::marker::Sync,
{
    let base: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(base_image);
    let top: &[Srgb<u8>] = Pixel::from_raw_slice(top_image);
    base.par_iter_mut().zip(top).for_each(|(color1, color2)| {
        let color1_alpha: LinSrgba = color1.into_format().into_linear().opaque();
        let color2_alpha: LinSrgba = color2.into_format().into_linear().with_alpha(alpha);
        let blended = blend_fn(color1_alpha, color2_alpha);
        //        let blended = color2_alpha.multiply(color1_alpha);

        *color1 = blended.color.into_encoding().into_format();
    });
}

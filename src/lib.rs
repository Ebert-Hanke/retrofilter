use image::{imageops::blur, ImageBuffer, Rgb};
mod vignette;
use palette::{Blend, LinSrgba, Pixel, Srgb, WithAlpha};
use rand::prelude::*;
use rayon::prelude::*;
pub use vignette::create_vignette;

pub fn palette_blend<F>(
    base_image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    top_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    alpha: f32,
    blend_fn: F,
) where
    F: Fn(LinSrgba, LinSrgba) -> LinSrgba + std::marker::Sync,
{
    base_image
        .pixels_mut()
        .into_iter()
        .zip(top_image.pixels().into_iter())
        .for_each(|(base, top)| {
            let color1: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(&mut base.0);
            let color2: &[Srgb<u8>] = Pixel::from_raw_slice(&top.0);
            let color1_alpha: LinSrgba = color1[0].into_format().into_linear().opaque();
            let color2_alpha: LinSrgba = color2[0].into_format().into_linear().with_alpha(alpha);
            let blended = blend_fn(color1_alpha, color2_alpha);

            color1[0] = blended.color.into_encoding().into_format();
        });
}

pub fn film_grain(width: u32, height: u32, noise_amount: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = ImageBuffer::from_fn(width, height, |_x, _y| Rgb([255u8, 255u8, 255u8]));
    let mut rng = rand::thread_rng();
    buffer.pixels_mut().for_each(|px| {
        if rng.gen_range(0..100) < noise_amount {
            let random = rng.gen_range(0..50);
            (0..3).into_iter().for_each(|i| {
                px[i] -= random;
            });
        }
    });
    buffer = blur(&buffer, 0.3);
    buffer
}

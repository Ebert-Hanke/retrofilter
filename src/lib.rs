use image::{imageops::blur, ImageBuffer, Rgb};
mod vignette;
use palette::{LinSrgba, Pixel, Srgb, WithAlpha};
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
    let base: &mut [Srgb<u8>] = Pixel::from_raw_slice_mut(base_image);
    let top: &[Srgb<u8>] = Pixel::from_raw_slice(top_image);

    base.par_iter_mut().zip(top).for_each(|(c1, c2)| {
        let color1_alpha: LinSrgba = c1.into_format().into_linear().opaque();
        let color2_alpha: LinSrgba = c2.into_format().into_linear().with_alpha(alpha);
        let blended = blend_fn(color1_alpha, color2_alpha);

        *c1 = blended.color.into_encoding().into_format();
    });
}

pub fn film_grain(width: u32, height: u32, noise_amount: u8) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = ImageBuffer::from_fn(width, height, |_x, _y| Rgb([255u8, 255u8, 255u8]));
    let mut rng = rand::thread_rng();
    buffer.pixels_mut().for_each(|px| {
        if rng.gen_range(0..100) < noise_amount {
            let random = rng.gen_range(0..100);
            (0..3).into_iter().for_each(|i| {
                px[i] -= random;
            });
        }
    });
    buffer = blur(&buffer, 0.3);
    buffer
}

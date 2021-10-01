use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_filled_circle_mut;

fn create_vignette(radius: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let x = radius * 2;
    let mut vignette = RgbImage::new(x, x);
    vignette
        .pixels_mut()
        .into_iter()
        .for_each(|px| *px = Rgb([0, 0, 0]));
    draw_filled_circle_mut(
        &mut vignette,
        (radius as i32, radius as i32),
        radius as i32,
        Rgb([255u8, 255u8, 255u8]),
    );
    vignette
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

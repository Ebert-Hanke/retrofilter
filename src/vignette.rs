use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_circle_mut;

pub fn create_vignette(
    width: u32,
    height: u32,
    radius: u32,
    noise: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (center_x, center_y) = ((width / 2) as i32, (height / 2) as i32);
    let mut buffer = ImageBuffer::from_fn(width, height, |_x, _y| Rgb([0u8, 0u8, 0u8]));

    // partial radius should be gradient
    let inner_radius = (radius as f32 * 0.33).round() as u32;
    let outer_radius = radius - inner_radius;

    // draw clear center
    draw_filled_circle_mut(
        &mut buffer,
        (center_x, center_y),
        inner_radius as i32,
        Rgb([255u8, 255u8, 255u8]),
    );

    // draw gradient
    let alpha_step = 255.0 / outer_radius as f32;
    (1..=outer_radius).into_iter().for_each(|i| {
        let alpha = ((alpha_step * i as f32).round() as u8).clamp(0u8, 255u8);
        draw_filled_circle_mut(
            &mut buffer,
            (center_x, center_y),
            (radius - i) as i32,
            Rgb([alpha, alpha, alpha]),
        );
    });

    if noise {
        gradient_noise(&mut buffer);
    }

    buffer
}

fn gradient_noise(buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    use rand::prelude::*;
    use rayon::prelude::*;

    // optionally add noise to gradient in order to break up banding artefacts
    buffer.par_iter_mut().for_each_init(
        || rand::thread_rng(),
        |rng, p| {
            if *p != 0 && *p != 255 {
                //let mut rng = rand::thread_rng();
                let random: i32 = *p as i32 + rng.gen_range(-10..10);
                if random < 0 {
                    *p = 0
                } else {
                    *p = random.clamp(0, 255) as u8
                }
            }
        },
    );
}

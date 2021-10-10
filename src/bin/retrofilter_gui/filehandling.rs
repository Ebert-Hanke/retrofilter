use image::{codecs::jpeg::JpegEncoder, io::Reader, ColorType, DynamicImage, ImageBuffer, Rgb};
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn image_open(image_path: PathBuf) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let img = Reader::open(image_path)?;
    let image_data = img.decode()?;
    Ok(image_data)
}

pub fn image_save(
    image_data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    quality: u8,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    //let mut save_path = Path::new(&path);
    let save_path = path.with_extension("jpg");
    let file = File::create(&save_path)?;
    let buffer = &mut BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(buffer, quality);
    encoder.encode(
        &image_data,
        image_data.dimensions().0,
        image_data.dimensions().1,
        ColorType::Rgb8,
    )?;
    Ok(())
}

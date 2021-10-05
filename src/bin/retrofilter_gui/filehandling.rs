use image::{codecs::jpeg::JpegEncoder, ColorType, ImageBuffer, Rgb};
use std::{fs::File, io::BufWriter, path::PathBuf};

// pub fn image_open(filename: &str) -> DynamicImage {
//     let img_ref = Reader::open(filename);
//     let img = match img_ref {
//         Ok(image) => image,
//         Err(error) => panic!("The requested file could not be found. {:?}", error),
//     };
//     let image_data = img.decode();
//     match image_data {
//         Ok(image) => image,
//         Err(error) => panic!(
//             "There was an error with decoding the image file. {:?}",
//             error
//         ),
//     }
// }

pub fn image_save(image_data: ImageBuffer<Rgb<u8>, Vec<u8>>, quality: u8, path: PathBuf) {
    //let mut save_path = Path::new(&path);
    let save_path = path.with_extension("jpg");
    let file = File::create(&save_path).unwrap();
    let buffer = &mut BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(buffer, quality);
    encoder.encode(
        &image_data,
        image_data.dimensions().0,
        image_data.dimensions().1,
        ColorType::Rgb8,
    );
}

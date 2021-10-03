use fltk::{
    app, button,
    dialog::file_chooser,
    draw,
    enums::{Color, ColorDepth},
    frame,
    image::{self as fl_image, SharedImage},
    prelude::*,
    valuator, window,
};
use image::{io::Reader as ImageReader, GenericImageView};
use retro_filter_lib::{create_vignette, preview};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = ImageReader::open("/Users/halo/desktop/image.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let thumbnail = image.thumbnail(300, 300);
    let app = app::App::default();
    let mut win = window::Window::default().with_size(900, 600);
    let mut slider = valuator::Slider::new(800, 10, 30, 500, "slider");
    let mut frm = frame::Frame::default().with_size(300, 300);
    slider.set_range(300.0, 1.0);
    slider.set_step(1.0, 1);
    win.set_color(Color::White);
    win.end();
    win.show();
    win.make_resizable(true);
    slider.set_callback({
        move |s| {
            let preview = preview(thumbnail.clone(), s.value() as i32);
            let (w, h) = preview.dimensions();
            let fltk_img =
                fl_image::RgbImage::new(&preview, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
            frm.set_image(Some(fltk_img));
            app::redraw();
        }
    });
    app.run()?;
    Ok(())
}

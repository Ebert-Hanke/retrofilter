use fltk::{enums::*, prelude::*, *};
use retro_filter_lib::create_vignette;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = create_vignette(300);
    let (w, h) = img.dimensions();
    let app = app::App::default();
    let mut win = window::Window::default().with_size(w as i32, h as i32);
    let mut frm = frame::Frame::default().size_of(&win);
    win.end();
    win.show();
    win.make_resizable(true);
    frm.draw(move |_| {
        draw::draw_image(&img, 0, 0, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    });
    app.run()?;
    Ok(())
}

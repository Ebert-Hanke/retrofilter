use fltk::{
    app, button,
    dialog::file_chooser,
    enums::{Color, ColorDepth, FrameType},
    frame,
    frame::Frame,
    image::{self as fl_image},
    prelude::*,
    valuator,
    valuator::Slider,
    window,
};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};
use image::{io::Reader as ImageReader, DynamicImage};
use retro_filter_lib::preview;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // dummy image
    let image = ImageReader::open("/Users/halo/desktop/image.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let thumbnail = image.thumbnail(400, 400);
    // setup fltk gui
    // theme
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::from_colormap(color_themes::DARK_THEME);
    theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();
    // define window
    let mut win = window::Window::default()
        .with_size(900, 600)
        .with_label("Retro Filter");
    win.set_color(Color::BackGround);
    win.make_resizable(true);
    // define components
    let mut frm = frame::Frame::new(10, 10, 400, 400, None);
    frm.set_frame(FrameType::BorderBox);
    frm.set_color(Color::Dark1);
    let mut v_slider_radius = valuator::Slider::new(430, 10, 20, 400, "Radius");
    v_slider_radius.set_range(200.0, 1.0);
    v_slider_radius.set_step(1.0, 1);
    v_slider_radius.set_value(100.0);
    let mut v_slider_opacity = valuator::Slider::new(480, 10, 20, 400, "Opacity");
    v_slider_opacity.set_range(255.0, 1.0);
    v_slider_opacity.set_step(1.0, 1);
    v_slider_opacity.set_value(100.0);
    // end setup and display window
    win.end();
    win.show();
    // functionality
    draw_image(&thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
    v_slider_radius.set_callback({
        let v_slider_radius = v_slider_radius.clone();
        let v_slider_opacity = v_slider_opacity.clone();
        let thumbnail = thumbnail.clone();
        let mut frm = frm.clone();
        move |_| {
            draw_image(&thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
            app::redraw();
        }
    });
    v_slider_opacity.set_callback({
        let v_slider_radius = v_slider_radius.clone();
        let v_slider_opacity = v_slider_opacity.clone();
        let thumbnail = thumbnail.clone();
        let mut frm = frm.clone();
        move |_| {
            draw_image(&thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
            app::redraw();
        }
    });
    app.run()?;
    Ok(())
}

fn draw_image(
    thumbnail: &DynamicImage,
    frame: &mut Frame,
    v_slider_radius: &Slider,
    v_slider_opacity: &Slider,
) {
    let preview = preview(
        thumbnail,
        v_slider_radius.value() as i32,
        v_slider_opacity.value() as u8,
    );
    let (w, h) = preview.dimensions();
    let fltk_img = fl_image::RgbImage::new(&preview, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    frame.set_image(Some(fltk_img));
}

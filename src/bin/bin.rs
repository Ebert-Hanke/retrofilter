use fltk::{
    app, button,
    dialog::{self, FileDialogOptions},
    enums::{Color, ColorDepth, FrameType},
    frame,
    frame::Frame,
    image as fl_image,
    prelude::*,
    valuator,
    valuator::Slider,
    window,
};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};
use image::{io::Reader as ImageReader, DynamicImage};
use retro_filter_lib::preview;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenFile,
    SaveFile,
    ChangeRadius,
    ChangeOpacity,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initial data
    let mut image_data: Option<DynamicImage> = None;
    let mut thumbnail = None;

    // setup fltk gui
    // theme
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    //let theme = ColorTheme::from_colormap(color_themes::GRAY_THEME);
    //theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();
    // define window
    let mut win = window::Window::default()
        .with_size(900, 600)
        .center_screen()
        .with_label("Retro Filter");
    win.set_color(Color::BackGround);
    win.make_resizable(true);
    // define components
    let mut input_chooser = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseFile);
    input_chooser.set_option(dialog::FileDialogOptions::Preview);
    input_chooser.set_option(FileDialogOptions::UseFilterExt);
    input_chooser.set_filter("*.{jpg,jpeg,JPG,png,tif,bmp}");
    let mut save_chooser = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseSaveFile);
    save_chooser.set_option(dialog::FileDialogOptions::SaveAsConfirm);
    let mut btn_open_file = button::Button::new(10, 430, 100, 20, "Open File");
    let mut btn_save_file = button::Button::new(130, 430, 100, 20, "Save File");
    let mut frm = frame::Frame::new(10, 10, 400, 400, None);
    frm.set_frame(FrameType::BorderBox);
    frm.set_color(Color::Dark1);
    let mut v_slider_radius = valuator::Slider::new(430, 10, 20, 400, "Radius");
    v_slider_radius.set_range(300.0, 50.0);
    v_slider_radius.set_step(1.0, 1);
    v_slider_radius.set_value(150.0);
    let mut v_slider_opacity = valuator::Slider::new(480, 10, 20, 400, "Opacity");
    v_slider_opacity.set_range(255.0, 50.0);
    v_slider_opacity.set_step(1.0, 1);
    v_slider_opacity.set_value(127.0);
    // end setup and display window
    win.end();
    win.show();

    // event handling
    let (s, r) = app::channel::<Message>();
    btn_open_file.emit(s, Message::OpenFile);
    btn_save_file.emit(s, Message::SaveFile);
    v_slider_radius.emit(s, Message::ChangeRadius);
    v_slider_opacity.emit(s, Message::ChangeOpacity);

    // event loop for messages
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                // functionality
                Message::OpenFile => {
                    input_chooser.show();
                    let input_path = input_chooser.filename();
                    if input_path.is_file() {
                        image_data = Some(ImageReader::open(input_path).unwrap().decode().unwrap());
                        if let Some(image) = &image_data {
                            thumbnail = Some(image.thumbnail(400, 400));
                        }
                        // draw initial view
                        if let Some(thumbnail) = &thumbnail {
                            draw_image(thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
                            app::redraw();
                        }
                    }
                }
                Message::SaveFile => {
                    save_chooser.show();
                    let save_path = save_chooser.filename();
                }
                Message::ChangeRadius => {
                    if let Some(thumbnail) = &thumbnail {
                        draw_image(thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
                        app::redraw();
                    }
                }
                Message::ChangeOpacity => {
                    if let Some(thumbnail) = &thumbnail {
                        draw_image(thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
                        app::redraw();
                    }
                }
            }
        }
    }

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
        v_slider_radius.value() as u32,
        v_slider_opacity.value() as u8,
    );
    let (w, h) = preview.dimensions();
    let fltk_img = fl_image::RgbImage::new(&preview, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    frame.set_image(Some(fltk_img));
}

mod filehandling;
use filehandling::image_save;
use fltk::{
    app, button,
    dialog::{self, FileDialogOptions},
    enums::{Color, ColorDepth, FrameType},
    frame,
    frame::Frame,
    image as fl_image,
    misc::Progress,
    prelude::*,
    valuator,
    valuator::NiceSlider,
    window,
};
use fltk_theme::{ThemeType, WidgetTheme};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, Rgb};
use retro_filter::process_image;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenFile,
    SaveFile,
    ProcessFile,
    ChangeRadius,
    ChangeOpacity,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // settings
    let preview_size: u32 = 400;
    // initial data
    let mut image_data: Option<DynamicImage> = None;
    let mut thumbnail = None;
    let mut processed_image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> = None;
    // setup fltk gui
    // theme
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    //let theme = ColorTheme::from_colormap(color_themes::GRAY_THEME);
    //theme.apply();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();
    // define window
    let mut win = window::Window::default()
        .with_size(600, 600)
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
    let mut btn_process_file = button::LightButton::new(130, 430, 100, 20, "Process");
    btn_process_file.deactivate();
    btn_process_file.turn_on(false);
    let mut btn_save_file = button::Button::new(250, 430, 100, 20, "Save File");
    btn_save_file.deactivate();
    let mut frm = frame::Frame::new(10, 10, preview_size as i32, preview_size as i32, None);
    frm.set_frame(FrameType::BorderBox);
    frm.set_color(Color::Dark1);
    let mut v_slider_radius = valuator::NiceSlider::new(450, 10, 20, 400, "Radius");
    v_slider_radius.set_range(350.0, 50.0);
    v_slider_radius.set_step(1.0, 1);
    v_slider_radius.set_value(150.0);
    let mut v_slider_opacity = valuator::NiceSlider::new(520, 10, 20, 400, "Opacity");
    v_slider_opacity.set_range(0.0, 1.0);
    v_slider_opacity.set_step(0.1, 1);
    v_slider_opacity.set_value(0.5);
    let mut progress_bar = Progress::new(10, 500, 300, 20, "Progress");
    progress_bar.set_minimum(0.0);
    progress_bar.set_maximum(100.0);
    // end setup and display window
    win.end();
    win.show();

    // event handling
    let (s, r) = app::channel::<Message>();
    btn_open_file.emit(s, Message::OpenFile);
    btn_process_file.emit(s, Message::ProcessFile);
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
                            thumbnail = Some(image.thumbnail(preview_size, preview_size));
                        }
                        // draw initial view
                        if let Some(thumbnail) = &thumbnail {
                            draw_image(thumbnail, &mut frm, &v_slider_radius, &v_slider_opacity);
                            btn_process_file.activate();
                            app::redraw();
                        }
                    }
                }
                Message::ProcessFile => {
                    if let Some(image_data) = &image_data {
                        let scaled_radius =
                            v_slider_radius.value() * get_preview_scale(image_data, &preview_size);
                        processed_image = Some(process_image(
                            image_data,
                            scaled_radius.round() as u32,
                            v_slider_opacity.value() as f32,
                            false,
                        ));
                        btn_process_file.turn_on(true);
                        btn_save_file.activate();
                        btn_process_file.turn_on(false);
                        app::redraw();
                    }
                }
                Message::SaveFile => {
                    save_chooser.show();
                    let save_path = save_chooser.filename();
                    match &mut processed_image {
                        Some(image) => {
                            image_save(image.clone(), 80, save_path);
                            processed_image = None;
                            btn_save_file.deactivate();
                            app::redraw();
                        }
                        None => (),
                    }
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
    v_slider_radius: &NiceSlider,
    v_slider_opacity: &NiceSlider,
) {
    let preview = process_image(
        thumbnail,
        v_slider_radius.value() as u32,
        v_slider_opacity.value() as f32,
        true,
    );
    let (w, h) = preview.dimensions();
    let fltk_img = fl_image::RgbImage::new(&preview, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    frame.set_image(Some(fltk_img));
}

fn get_preview_scale(image_data: &DynamicImage, preview_size: &u32) -> f64 {
    let (w, h) = image_data.dimensions();
    let longer_axis = if w > h { w } else { h };
    longer_axis as f64 / *preview_size as f64
}

fn set_progress(progress: f64, progress_bar: &mut Progress) {
    progress_bar.set_value(progress_bar.value() + progress);
}

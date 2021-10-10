mod filehandling;
use filehandling::{image_open, image_save};
use fltk::{
    app,
    button::{self, CheckButton},
    dialog::{self, FileDialogOptions},
    enums::{Align, Color, ColorDepth, FrameType},
    frame,
    frame::Frame,
    group::Group,
    image as fl_image,
    prelude::*,
    valuator,
    valuator::NiceSlider,
    window,
};
use fltk_theme::{ThemeType, WidgetTheme};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use retro_filter::process_image;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenFile,
    SaveFile,
    ProcessFile,
    VignetteChangeRadius,
    VignetteChangeAlpha,
    VignetteToggle,
}

struct InputState {
    vignette: Option<(f64, f64)>,
    filmgrain: Option<(f64, f64)>,
}
impl InputState {
    fn new() -> InputState {
        InputState {
            vignette: None,
            filmgrain: None,
        }
    }
    fn set_vignette(&mut self, slider_radius: &NiceSlider, slider_alpha: &NiceSlider) {
        self.vignette = Some((slider_radius.value(), slider_alpha.value()));
    }
    fn set_filmgrain(&mut self, slider_strength: &NiceSlider, slider_alpha: &NiceSlider) {
        self.filmgrain = Some((slider_strength.value(), slider_alpha.value()));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // settings
    let preview_size: u32 = 400;
    // initial data
    let mut image_data: Option<DynamicImage> = None;
    let mut thumbnail = None;
    let mut processed_image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>> = None;
    let mut input_state = InputState::new();
    // setup fltk gui
    // theme
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let (s, r) = app::channel::<Message>();

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
    let mut btn_process_file = button::LightButton::new(130, 430, 100, 20, "Process");
    btn_process_file.deactivate();
    btn_process_file.turn_on(false);
    let mut btn_save_file = button::Button::new(250, 430, 100, 20, "Save File");
    btn_save_file.deactivate();
    let mut slider_jpg_quality = valuator::HorValueSlider::new(250, 460, 100, 20, "JPG Quality");
    slider_jpg_quality.set_range(1.0, 100.0);
    slider_jpg_quality.set_step(1.0, 1);
    slider_jpg_quality.set_value(75.0);
    let mut preview_frame =
        frame::Frame::new(10, 10, preview_size as i32, preview_size as i32, None);
    preview_frame.set_frame(FrameType::BorderBox);
    preview_frame.set_color(Color::Dark3);

    // vignette controls
    let mut vignette_controls = Group::new(420, 10, 120, 400, "Vignette");
    vignette_controls.set_align(Align::BottomRight);
    vignette_controls.set_frame(FrameType::BorderBox);
    let mut slider_vignette_radius = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(vignette_controls.x() + 20, vignette_controls.y() + 10)
        .with_label("Radius");
    slider_vignette_radius.set_range(350.0, 50.0);
    slider_vignette_radius.set_step(1.0, 1);
    slider_vignette_radius.set_value(250.0);
    let mut slider_vignette_alpha = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(vignette_controls.x() + 80, vignette_controls.y() + 10)
        .with_label("Alpha");
    slider_vignette_alpha.set_range(1.0, 0.0);
    slider_vignette_alpha.set_step(0.1, 1);
    slider_vignette_alpha.set_value(0.2);
    slider_vignette_radius.emit(s, Message::VignetteChangeRadius);
    slider_vignette_alpha.emit(s, Message::VignetteChangeAlpha);
    vignette_controls.end();
    vignette_controls.deactivate();
    let mut vignette_active = CheckButton::default()
        .with_size(10, 10)
        .below_of(&vignette_controls, 10);
    vignette_active.emit(s, Message::VignetteToggle);

    // filmgrain controls
    let mut filmgrain_controls = Group::new(550, 10, 120, 400, "Filmgrain");
    filmgrain_controls.set_align(Align::BottomRight);
    filmgrain_controls.set_frame(FrameType::BorderBox);
    let mut slider_filmgrain_strength = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(filmgrain_controls.x() + 20, filmgrain_controls.y() + 10)
        .with_label("Strength");
    slider_filmgrain_strength.set_range(100.0, 0.0);
    slider_filmgrain_strength.set_step(1.0, 1);
    slider_filmgrain_strength.set_value(50.0);
    let mut slider_filmgrain_alpha = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(filmgrain_controls.x() + 80, filmgrain_controls.y() + 10)
        .with_label("Alpha");
    slider_filmgrain_alpha.set_range(1.0, 0.0);
    slider_filmgrain_alpha.set_step(0.1, 1);
    slider_filmgrain_alpha.set_value(0.2);
    slider_filmgrain_strength.emit(s, Message::VignetteChangeRadius);
    slider_filmgrain_alpha.emit(s, Message::VignetteChangeAlpha);
    filmgrain_controls.end();
    let mut filmgrain_active = CheckButton::default().with_size(10, 10);
    filmgrain_active.below_of(&filmgrain_controls, 10);

    // let mut progress_bar = Progress::new(10, 500, 300, 20, "Progress");
    // progress_bar.set_minimum(0.0);
    // progress_bar.set_maximum(100.0);

    // end setup and display window
    win.end();
    win.show();

    // event handling
    btn_open_file.emit(s, Message::OpenFile);
    btn_process_file.emit(s, Message::ProcessFile);
    btn_save_file.emit(s, Message::SaveFile);
    // event loop for messages
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                // functionality
                Message::OpenFile => {
                    input_chooser.show();
                    let input_path = input_chooser.filename();
                    if input_path.is_file() {
                        image_data = Some(image_open(input_path)?);
                        if let Some(image) = &image_data {
                            thumbnail = Some(image.thumbnail(preview_size, preview_size));
                        }
                        // draw initial view
                        if let Some(thumbnail) = &thumbnail {
                            draw_image(thumbnail, &mut preview_frame, &input_state)?;
                            btn_process_file.activate();
                            app::redraw();
                        }
                    }
                }
                Message::ProcessFile => {
                    if let Some(image_data) = &image_data {
                        let scaled_radius = slider_vignette_radius.value()
                            * get_preview_scale(image_data, &preview_size);
                        processed_image = Some(process_image(
                            image_data,
                            scaled_radius.round() as u32,
                            slider_vignette_alpha.value() as f32,
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
                    if save_path.file_name().is_some() {
                        match &mut processed_image {
                            Some(image) => {
                                image_save(
                                    image.clone(),
                                    slider_jpg_quality.value() as u8,
                                    save_path,
                                )?;
                                processed_image = None;
                                btn_save_file.deactivate();
                                app::redraw();
                            }
                            None => (),
                        }
                    }
                }
                Message::VignetteToggle => {
                    if vignette_controls.active() {
                        vignette_controls.deactivate();
                        vignette_active.set_checked(false);
                    } else {
                        vignette_controls.activate();
                        vignette_active.set_checked(true);
                    };
                    app::redraw();
                }
                Message::VignetteChangeRadius => {
                    if let Some(thumbnail) = &thumbnail {
                        input_state.set_vignette(&slider_vignette_radius, &slider_filmgrain_alpha);
                        draw_image(thumbnail, &mut preview_frame, &input_state)?;
                        app::redraw();
                    }
                }
                Message::VignetteChangeAlpha => {
                    if let Some(thumbnail) = &thumbnail {
                        draw_image(thumbnail, &mut preview_frame, &input_state)?;
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
    input_state: &InputState,
) -> Result<(), FltkError> {
    if let Some(vignette) = input_state.vignette {
        let preview = process_image(thumbnail, vignette.0 as u32, vignette.1 as f32);
        let (w, h) = preview.dimensions();
        let fltk_img = fl_image::RgbImage::new(&preview, w as i32, h as i32, ColorDepth::Rgb8)?;
        frame.set_image(Some(fltk_img));
    }
    Ok(())
}

fn get_preview_scale(image_data: &DynamicImage, preview_size: &u32) -> f64 {
    let (w, h) = image_data.dimensions();
    let longer_axis = if w > h { w } else { h };
    longer_axis as f64 / *preview_size as f64
}

// fn set_progress(progress: f64, progress_bar: &mut Progress) {
//     progress_bar.set_value(progress_bar.value() + progress);
// }

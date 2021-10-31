mod filehandling;
use std::path::PathBuf;

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
use palette::Blend;
use retro_filter::{bleach_bypass, create_vignette, film_grain, palette_blend};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenFile,
    SaveFile,
    ProcessFile,
    VignetteChange,
    VignetteToggle,
    FilmgrainChange,
    FilmgrainToggle,
    BleachbypassChange,
    BleachbypassToggle,
}

struct DataState {
    image_data: Option<DynamicImage>,
    preview_size: u32,
    image_thumbnail: Option<DynamicImage>,
    image_processed: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}
impl DataState {
    fn new() -> Self {
        Self {
            image_data: None,
            preview_size: 400,
            image_thumbnail: None,
            image_processed: None,
        }
    }
    fn set_image(&mut self, input_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.image_data = Some(image_open(input_path)?);
        if let Some(image) = &self.image_data {
            self.image_thumbnail = Some(image.thumbnail(self.preview_size, self.preview_size));
        }
        Ok(())
    }
    fn process_thumbnail(&mut self, input_state: &InputState) {
        if let Some(image_thumbnail) = &self.image_thumbnail {
            self.image_thumbnail = Some(DynamicImage::ImageRgb8(process_image(
                image_thumbnail,
                input_state,
                self.preview_size,
                true,
            )));
        }
    }
    fn reset_thumbnail(&mut self) {
        if let Some(image) = &self.image_data {
            self.image_thumbnail = Some(image.thumbnail(self.preview_size, self.preview_size));
        }
    }
    fn process_image(&mut self, input_state: &InputState) {
        if let Some(image_data) = &self.image_data {
            self.image_processed = Some(process_image(
                image_data,
                input_state,
                self.preview_size,
                false,
            ));
        }
    }
    fn reset_process_image(&mut self) {
        self.image_processed = None;
    }
    fn set_fltk_image(&mut self, frame: &mut Frame) -> Result<(), FltkError> {
        if let Some(thumbnail) = &self.image_thumbnail {
            let (w, h) = thumbnail.dimensions();
            let fltk_img = fl_image::RgbImage::new(
                &thumbnail.to_rgb8(),
                w as i32,
                h as i32,
                ColorDepth::Rgb8,
            )?;
            frame.set_image(Some(fltk_img));
        }
        Ok(())
    }
}

struct InputState {
    vignette: Option<(f64, f64)>,
    filmgrain: Option<(f64, f64)>,
    bleachbypass: Option<(f64, f64)>,
}
impl InputState {
    fn new() -> InputState {
        InputState {
            vignette: None,
            filmgrain: None,
            bleachbypass: None,
        }
    }
    fn set_vignette(&mut self, slider_radius: &NiceSlider, slider_alpha: &NiceSlider) {
        self.vignette = Some((slider_radius.value(), slider_alpha.value()));
    }
    fn reset_vignette(&mut self) {
        self.vignette = None;
    }
    fn set_filmgrain(&mut self, slider_strength: &NiceSlider, slider_alpha: &NiceSlider) {
        self.filmgrain = Some((slider_strength.value(), slider_alpha.value()));
    }
    fn reset_filmgrain(&mut self) {
        self.filmgrain = None;
    }
    fn set_bleachbypass(&mut self, slider_strength: &NiceSlider, slider_alpha: &NiceSlider) {
        self.bleachbypass = Some((slider_strength.value(), slider_alpha.value()));
    }
    fn reset_bleachbypass(&mut self) {
        self.bleachbypass = None;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // settings
    let preview_size: u32 = 400;
    // initial state
    let mut input_state = InputState::new();
    let mut data_state = DataState::new();
    // setup fltk gui
    // theme
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let (s, r) = app::channel::<Message>();

    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    // define window
    let mut win = window::Window::default()
        .with_size(810, 500)
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
    let mut btn_open_file = button::Button::new(10, 420, 100, 20, "Open File");
    let mut btn_save_file = button::Button::new(10, 450, 100, 20, "Save File");
    btn_save_file.deactivate();
    let mut btn_process_file = button::LightButton::new(120, 420, 100, 20, "Process");
    btn_process_file.deactivate();
    btn_process_file.turn_on(false);
    let mut slider_jpg_quality = valuator::HorValueSlider::new(120, 450, 100, 20, "JPG Quality");
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
    slider_vignette_alpha.set_value(0.7);
    slider_vignette_radius.emit(s, Message::VignetteChange);
    slider_vignette_alpha.emit(s, Message::VignetteChange);
    vignette_controls.end();
    vignette_controls.deactivate();
    let mut vignette_active = CheckButton::default()
        .with_size(15, 15)
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
    slider_filmgrain_strength.emit(s, Message::FilmgrainChange);
    slider_filmgrain_alpha.emit(s, Message::FilmgrainChange);
    filmgrain_controls.end();
    filmgrain_controls.deactivate();
    let mut filmgrain_active = CheckButton::default()
        .with_size(15, 15)
        .below_of(&filmgrain_controls, 10);
    filmgrain_active.emit(s, Message::FilmgrainToggle);

    // bleachbypass controls
    let mut bleachbypass_controls = Group::new(680, 10, 120, 400, "BleachBypass");
    bleachbypass_controls.set_align(Align::BottomRight);
    bleachbypass_controls.set_frame(FrameType::BorderBox);
    let mut slider_bleachbypass_blur = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(
            bleachbypass_controls.x() + 20,
            bleachbypass_controls.y() + 10,
        )
        .with_label("Blur");
    slider_bleachbypass_blur.set_range(2.0, 0.1);
    slider_bleachbypass_blur.set_step(0.1, 1);
    slider_bleachbypass_blur.set_value(0.0);
    let mut slider_bleachbypass_alpha = valuator::NiceSlider::default()
        .with_size(20, 370)
        .with_pos(
            bleachbypass_controls.x() + 80,
            bleachbypass_controls.y() + 10,
        )
        .with_label("Alpha");
    slider_bleachbypass_alpha.set_range(1.0, 0.0);
    slider_bleachbypass_alpha.set_step(0.1, 1);
    slider_bleachbypass_alpha.set_value(0.2);
    slider_bleachbypass_blur.emit(s, Message::BleachbypassChange);
    slider_bleachbypass_alpha.emit(s, Message::BleachbypassChange);
    bleachbypass_controls.end();
    bleachbypass_controls.deactivate();
    let mut bleachbypass_active = CheckButton::default()
        .with_size(15, 15)
        .below_of(&bleachbypass_controls, 10);
    bleachbypass_active.emit(s, Message::BleachbypassToggle);

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
                        data_state.set_image(input_path)?;
                    }
                    // draw initial view
                    data_state.set_fltk_image(&mut preview_frame)?;
                    btn_process_file.activate();
                    app::redraw();
                }

                Message::ProcessFile => {
                    data_state.process_image(&input_state);
                    btn_process_file.turn_on(true);
                    btn_save_file.activate();
                    btn_process_file.turn_on(false);
                    app::redraw();
                }
                Message::SaveFile => {
                    save_chooser.show();
                    let save_path = save_chooser.filename();
                    if save_path.file_name().is_some() {
                        match &mut data_state.image_processed {
                            Some(image) => {
                                image_save(
                                    image.clone(),
                                    slider_jpg_quality.value() as u8,
                                    save_path,
                                )?;
                                data_state.reset_process_image();
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
                        input_state.reset_vignette();
                        data_state.reset_thumbnail();
                    } else {
                        vignette_controls.activate();
                        vignette_active.set_checked(true);
                        input_state.set_vignette(&slider_vignette_radius, &slider_vignette_alpha);
                    };
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
                Message::VignetteChange => {
                    input_state.set_vignette(&slider_vignette_radius, &slider_vignette_alpha);
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
                Message::FilmgrainToggle => {
                    if filmgrain_controls.active() {
                        filmgrain_controls.deactivate();
                        filmgrain_active.set_checked(false);
                        input_state.reset_filmgrain();
                        data_state.reset_thumbnail();
                    } else {
                        filmgrain_controls.activate();
                        filmgrain_active.set_checked(true);
                        input_state
                            .set_filmgrain(&slider_filmgrain_strength, &slider_filmgrain_alpha);
                    };
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
                Message::FilmgrainChange => {
                    input_state.set_filmgrain(&slider_filmgrain_strength, &slider_filmgrain_alpha);
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
                Message::BleachbypassToggle => {
                    if bleachbypass_controls.active() {
                        bleachbypass_controls.deactivate();
                        bleachbypass_active.set_checked(false);
                        input_state.reset_bleachbypass();
                        data_state.reset_thumbnail();
                    } else {
                        bleachbypass_controls.activate();
                        bleachbypass_active.set_checked(true);
                        input_state.set_bleachbypass(
                            &slider_bleachbypass_blur,
                            &slider_bleachbypass_alpha,
                        );
                    };
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
                Message::BleachbypassChange => {
                    input_state
                        .set_bleachbypass(&slider_bleachbypass_blur, &slider_bleachbypass_alpha);
                    data_state.process_thumbnail(&input_state);
                    data_state.set_fltk_image(&mut preview_frame)?;
                    app::redraw();
                }
            }
        }
    }

    Ok(())
}

fn process_image(
    image: &DynamicImage,
    input_state: &InputState,
    preview_size: u32,
    preview: bool,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut base_image = image.clone().to_rgb8();
    if let Some(bleachbypass_input) = input_state.bleachbypass {
        let overlay = bleach_bypass(&base_image, bleachbypass_input.0 as f32);
        if let Some(overlay) = overlay {
            palette_blend(
                &mut base_image,
                &overlay,
                bleachbypass_input.1 as f32,
                |c1, c2| c1.overlay(c2),
            );
        };
    }
    if let Some(vignette_input) = input_state.vignette {
        let radius = if preview {
            vignette_input.0
        } else {
            get_preview_scale(image, &preview_size) * vignette_input.0
        };
        let vignette = create_vignette(width, height, radius as u32, true);
        palette_blend(
            &mut base_image,
            &vignette,
            vignette_input.1 as f32,
            |c1, c2| c1.multiply(c2),
        );
    }
    if let Some(filmgrain_input) = input_state.filmgrain {
        let filmgrain = film_grain(width, height, filmgrain_input.0 as u8);
        palette_blend(
            &mut base_image,
            &filmgrain,
            filmgrain_input.1 as f32,
            |c1, c2| c1.multiply(c2),
        );
    }

    base_image
}

fn get_preview_scale(image_data: &DynamicImage, preview_size: &u32) -> f64 {
    let (w, h) = image_data.dimensions();
    let longer_axis = if w > h { w } else { h };
    longer_axis as f64 / *preview_size as f64
}

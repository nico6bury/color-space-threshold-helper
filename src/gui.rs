use std::{cell::{BorrowMutError, RefCell}, rc::Rc};

use color_space_threshold_helper::{enums::{ColorSpace, InterfaceMessage}, process::ThreshParams};
use fltk::{app::{self, App, Receiver}, button::CheckButton, enums::{Align, Color}, group::{Flex, FlexType}, menu::Choice, prelude::{MenuExt, ValuatorExt}, valuator::HorValueSlider};
use fltk::button::Button;
use fltk::dialog::{self, FileDialogOptions, FileDialogType};
use fltk::enums::FrameType;
use fltk::frame::Frame;
use fltk::image::SharedImage;
use fltk::group::{Group, Tile};
use fltk::prelude::{ButtonExt, GroupExt, ImageExt, WidgetBase, WidgetExt};
use fltk::window::Window;

const GROUP_FRAME: FrameType = FrameType::GtkThinUpBox;
const BUTTON_FRAME: FrameType = FrameType::GtkRoundUpFrame;
const BUTTON_DOWN_FRAME: FrameType = FrameType::GtkRoundDownFrame;
const PASS_ALIGN: Align = Align::Inside.union(Align::Left);

pub struct GUI {
    ux_app: App,
    // ux_main_window: Window,
    image_loaded: Rc<RefCell<Option<SharedImage>>>,
    image_frame: Frame,
    // msg_sender: Sender<InterfaceMessage>,
    msg_receiver: Receiver<InterfaceMessage>,
    ux_d1l_slider: HorValueSlider,
    ux_d1h_slider: HorValueSlider,
    ux_d1_pass_ck: CheckButton,
    ux_d2l_slider: HorValueSlider,
    ux_d2h_slider: HorValueSlider,
    ux_d2_pass_ck: CheckButton,
    ux_d3l_slider: HorValueSlider,
    ux_d3h_slider: HorValueSlider,
    ux_d3_pass_ck: CheckButton,
    ux_color_space_choice: Choice,
}//end struct GUI

impl GUI {
    pub fn quit(&self) {
        self.ux_app.quit();
    }//end quit()

    pub fn wait(&self) -> bool {
        self.ux_app.wait()
    }//end wait()

    pub fn get_receiver(&self) -> Receiver<InterfaceMessage> {
        return self.msg_receiver.clone();
    }//end get_receiver()

    // Changes the displayed image to the one provided
    pub fn load_image(&mut self, image: SharedImage) -> Result<(),BorrowMutError> {
        let img_ref_clone = (&mut self.image_loaded).clone();
        let img_ref_clone_res = img_ref_clone.as_ref().try_borrow_mut();
        match img_ref_clone_res {
            Err(err) => Err(err),
            Ok(mut img_ref) => {
                *img_ref = Some(image);
                self.image_frame.redraw();
                Ok(())
            },
        }//end matching 
    }//end load_image()

    /// Tries to get threshold parameters from widgets
    pub fn get_thresh_params(&self) -> Option<ThreshParams> {
        let color_space = ColorSpace::from_str(&self.ux_color_space_choice.choice().unwrap_or_else(|| "".to_string()));
        if color_space.is_none() {return None;}
        let color_space = color_space.unwrap();
        
        Some(ThreshParams {
            color_space,
            depth1_min: self.ux_d1l_slider.value() as u8,
            depth1_max: self.ux_d1h_slider.value() as u8,
            depth1_pass: self.ux_d1_pass_ck.is_checked(),
            depth2_min: self.ux_d2l_slider.value() as u8,
            depth2_max: self.ux_d2h_slider.value() as u8,
            depth2_pass: self.ux_d2_pass_ck.is_checked(),
            depth3_min: self.ux_d3l_slider.value() as u8,
            depth3_max: self.ux_d3h_slider.value() as u8,
            depth3_pass: self.ux_d3_pass_ck.is_checked(),
        })
    }//end get_thresh_params()

    pub fn initialize() -> GUI {
        // set up app, main window, channel stuff
        let csth_app = App::default();
        let mut main_window = Window::default()
            .with_size(700, 435)
            .with_label("Color Space Threshold Helper");
        main_window.end();
        main_window.make_resizable(true);

        let (s,r) = app::channel();

        // set up tiles and group containers for parts of gui
        let mut tile_group = Tile::default()
            .with_pos(0,0)
            .with_size(main_window.w(), main_window.h());
        tile_group.end();
        tile_group.set_frame(FrameType::FlatBox);
        main_window.add(&tile_group);

        let mut ux_image_group = Group::default()
            .with_pos(0,0)
            .with_size(main_window.w() / 2, main_window.h() - 50);
        ux_image_group.end();
        ux_image_group.set_frame(GROUP_FRAME);
        tile_group.add(&ux_image_group);

        let mut ux_param_flex = Flex::default()
            .with_pos(ux_image_group.x() + ux_image_group.w(), 0)
            .with_size(main_window.w() - ux_image_group.w(), ux_image_group.h());
        ux_param_flex.end();
        ux_param_flex.set_type(FlexType::Column);
        ux_param_flex.set_frame(GROUP_FRAME);
        ux_param_flex.set_margin(10);
        tile_group.add(&ux_param_flex);

        let mut ux_button_flex = Flex::default()
            .with_pos(0, ux_image_group.h())
            .with_size(main_window.w(), main_window.h() - ux_image_group.h());
        ux_button_flex.end();
        ux_button_flex.set_type(FlexType::Row);
        ux_button_flex.set_margin(10);
        ux_button_flex.set_frame(GROUP_FRAME);
        tile_group.add(&ux_button_flex);

        // define widgets for the image group
        let mut img_display_frame = Frame::default()
            .with_pos(ux_image_group.x() + 1, ux_image_group.y() + 1)
            .with_size(ux_image_group.w() - 2, ux_image_group.h() - 2);
        img_display_frame.set_frame(FrameType::EngravedFrame);
        ux_image_group.add(&img_display_frame);
        let img_ref: Rc<RefCell<Option<SharedImage>>> = Rc::from(RefCell::from(None));
        img_display_frame.draw({
            let img_ref_clone = (&img_ref).clone();
            move |f| {
                let mut img_ref = img_ref_clone.as_ref().borrow_mut();
                if let Some(img) = img_ref.as_mut() {
                    img.scale(f.w(), f.h(), true, true);
                    img.draw(f.x(), f.y(), f.w(), f.h());
                }//end if we have an image to draw from
        }});

        // define widgets for the button group
        let mut get_files_btn = Button::default()
            .with_label("Get Image");
        get_files_btn.set_frame(BUTTON_FRAME);
        get_files_btn.set_down_frame(BUTTON_DOWN_FRAME);
        get_files_btn.clear_visible_focus();
        ux_button_flex.add(&get_files_btn);
        get_files_btn.set_callback({
            let sender_clone = s.clone();
            move |_| {
                let mut dialog = dialog::NativeFileChooser::new(FileDialogType::BrowseFile);
                dialog.set_option(FileDialogOptions::Preview.union(FileDialogOptions::UseFilterExt));
                dialog.set_filter("Image File\t*.{jpeg,png,webp,svg,tif,tiff}");
                dialog.set_title("Select Image File");
                dialog.show();
                // make sure dialog didn't have an error or anything
                let dialog_error = dialog.error_message().unwrap_or_else(|| "".to_string()).replace("No error", "");
                if dialog_error == "" {
                    sender_clone.send(InterfaceMessage::LoadImage(dialog.filename()));
                } else {println!("Encountered dialog error: {dialog_error}");}
            }//end closure
        });

        let mut reset_btn = Button::default()
            .with_label("Reset");
        reset_btn.set_frame(BUTTON_FRAME);
        reset_btn.set_down_frame(BUTTON_DOWN_FRAME);
        reset_btn.clear_visible_focus();
        ux_button_flex.add(&reset_btn);
        reset_btn.set_callback({
            let sender_clone = s.clone();
            move |_| {sender_clone.send(InterfaceMessage::Reset);}
        });

        let mut color_space_choice = Choice::default();
        color_space_choice.add_choice(&ColorSpace::get_variants().join("|"));
        ux_button_flex.add(&color_space_choice);

        let mut thresh_color_btn = Button::default()
            .with_label("Threshold Color");
        thresh_color_btn.set_frame(BUTTON_FRAME);
        thresh_color_btn.set_down_frame(BUTTON_DOWN_FRAME);
        thresh_color_btn.clear_visible_focus();
        ux_button_flex.add(&thresh_color_btn);
        thresh_color_btn.set_callback({
            let sender_clone = s.clone();
            move |btn| {
                let color_res = dialog::color_chooser(
                    "Choose In-Threshold Color",
                    dialog::ColorMode::Rgb
                );
                if let Some(color) = color_res {
                    btn.set_color(Color::rgb_color(color.0, color.1, color.2));
                    btn.redraw();
                    sender_clone.send(InterfaceMessage::ThreshColor(color));
                }//end if we got a color
            }//end closure
        });

        let mut now_thresh_btn = Button::default()
            .with_label("Threshold");
        now_thresh_btn.set_frame(BUTTON_FRAME);
        now_thresh_btn.set_down_frame(BUTTON_DOWN_FRAME);
        now_thresh_btn.clear_visible_focus();
        ux_button_flex.add(&now_thresh_btn);
        now_thresh_btn.set_callback({
            let sender_clone = s.clone();
            move |_| {
                sender_clone.send(InterfaceMessage::DoThreshold);
            }//end closure
        });

        // define widgets for the param group
        let d1_label = Frame::default()
            .with_label("Depth 1 Min/Max Sliders")
            .with_align(Align::Bottom.union(Align::Inside));
        ux_param_flex.add(&d1_label);

        let mut d1_pass_chk = CheckButton::default()
            .with_label("Depth 1 Pass")
            .with_align(PASS_ALIGN);
        d1_pass_chk.set(true);
        ux_param_flex.add(&d1_pass_chk);

        let mut d1l_slider = HorValueSlider::default();
        d1l_slider.set_minimum(0.);
        d1l_slider.set_maximum(255.);
        d1l_slider.set_step(1.,1);
        d1l_slider.set_value(0.);
        ux_param_flex.add(&d1l_slider);
        
        let mut d1h_slider = HorValueSlider::default();
        d1h_slider.set_minimum(0.);
        d1h_slider.set_maximum(255.);
        d1h_slider.set_step(1.,1);
        d1h_slider.set_value(255.);
        ux_param_flex.add(&d1h_slider);

        let spacer_frame_1 = Frame::default();
        ux_param_flex.add(&spacer_frame_1);

        let d2_label = Frame::default()
            .with_label("Depth 2 Min/Max Sliders")
            .with_align(Align::Bottom.union(Align::Inside));
        ux_param_flex.add(&d2_label);

        let mut d2_pass_chk = CheckButton::default()
            .with_label("Depth 2 Pass")
            .with_align(PASS_ALIGN);
        d2_pass_chk.set(true);
        ux_param_flex.add(&d2_pass_chk);

        let mut d2l_slider = HorValueSlider::default();
        d2l_slider.set_minimum(0.);
        d2l_slider.set_maximum(255.);
        d2l_slider.set_step(1.,1);
        d2l_slider.set_value(0.);
        ux_param_flex.add(&d2l_slider);
        
        let mut d2h_slider = HorValueSlider::default();
        d2h_slider.set_minimum(0.);
        d2h_slider.set_maximum(255.);
        d2h_slider.set_step(1.,1);
        d2h_slider.set_value(255.);
        ux_param_flex.add(&d2h_slider);

        let spacer_frame_2 = Frame::default();
        ux_param_flex.add(&spacer_frame_2);

        let d3_label = Frame::default()
            .with_label("Depth 3 Min/Max Sliders")
            .with_align(Align::Bottom.union(Align::Inside));
        ux_param_flex.add(&d3_label);

        let mut d3_pass_chk = CheckButton::default()
            .with_label("Depth 3 Pass")
            .with_align(PASS_ALIGN);
        d3_pass_chk.set(true);
        ux_param_flex.add(&d3_pass_chk);

        let mut d3l_slider = HorValueSlider::default();
        d3l_slider.set_minimum(0.);
        d3l_slider.set_maximum(255.);
        d3l_slider.set_step(1.,1);
        d3l_slider.set_value(0.);
        ux_param_flex.add(&d3l_slider);
        
        let mut d3h_slider = HorValueSlider::default();
        d3h_slider.set_minimum(0.);
        d3h_slider.set_maximum(255.);
        d3h_slider.set_step(1.,1);
        d3h_slider.set_value(255.);
        ux_param_flex.add(&d3h_slider);

        // clean up, package stuff together, show window
        main_window.show();
        main_window.set_callback(move |_| {s.send(InterfaceMessage::Quit)});
        GUI {
            ux_app: csth_app,
            // ux_main_window: main_window,
            image_loaded: img_ref,
            image_frame: img_display_frame,
            // msg_sender: s,
            msg_receiver: r,
            ux_d1l_slider: d1l_slider,
            ux_d1h_slider: d1h_slider,
            ux_d1_pass_ck: d1_pass_chk,
            ux_d2l_slider: d2l_slider,
            ux_d2h_slider: d2h_slider,
            ux_d2_pass_ck: d2_pass_chk,
            ux_d3l_slider: d3l_slider,
            ux_d3h_slider: d3h_slider,
            ux_d3_pass_ck: d3_pass_chk,
            ux_color_space_choice: color_space_choice,
        }//end struct construction
    }//end initialize()
}//end impl for GUI
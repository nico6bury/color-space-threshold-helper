use std::{cell::{BorrowMutError, RefCell}, error::Error, fmt::Display, rc::Rc};

use fltk::{app::{self, App, Receiver, Sender}, enums::FrameType, frame::Frame, group::{Group, Tile}, image::Image, prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt}, window::Window};

const GROUP_FRAME: FrameType = FrameType::GtkThinUpBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceMessage {
    LoadImage,
    Quit
}//end enum InterfaceMessage

pub struct GUI {
    ux_app: App,
    ux_main_window: Window,
    image_loaded: Rc<RefCell<Option<Image>>>,
    msg_sender: Sender<InterfaceMessage>,
    msg_receiver: Receiver<InterfaceMessage>,
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
    pub fn load_image(&mut self, image: Image) -> Result<(),BorrowMutError> {
        let img_ref_clone = (&mut self.image_loaded).clone();
        let img_ref_clone_res = img_ref_clone.as_ref().try_borrow_mut();
        match img_ref_clone_res {
            Err(err) => Err(err),
            Ok(mut img_ref) => {
                *img_ref = Some(image);
                Ok(())
            },
        }//end matching 
    }//end load_image()

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

        let mut ux_param_group = Group::default()
            .with_pos(ux_image_group.x() + ux_image_group.w(), 0)
            .with_size(main_window.w() - ux_image_group.w(), ux_image_group.h());
        ux_param_group.end();
        ux_param_group.set_frame(GROUP_FRAME);
        tile_group.add(&ux_param_group);

        let mut ux_buton_group = Group::default()
            .with_pos(0, ux_image_group.h())
            .with_size(main_window.w(), main_window.h() - ux_image_group.h());
        ux_buton_group.end();
        ux_buton_group.set_frame(GROUP_FRAME);
        tile_group.add(&ux_buton_group);

        // define widgets for the image group
        let mut img_display_frame = Frame::default()
            .with_pos(ux_image_group.x() + 10, ux_image_group.y() + 10)
            .with_size(ux_image_group.w() - 20, ux_image_group.h() - 20);
        img_display_frame.set_frame(FrameType::EngravedFrame);
        let img_ref: Rc<RefCell<Option<Image>>> = Rc::from(RefCell::from(None));
        img_display_frame.draw({
            let img_ref_clone = (&img_ref).clone();
            move |f| {
                let mut img_ref = img_ref_clone.as_ref().borrow_mut();
                if let Some(img) = img_ref.as_mut() {
                    img.scale(f.w(), f.h(), true, true);
                    img.draw(f.x(), f.y(), f.w(), f.h());
                }//end if we have an image to draw from
        }});

        // clean up, package stuff together, show window
        main_window.show();
        GUI {
            ux_app: csth_app,
            ux_main_window: main_window,
            image_loaded: img_ref,
            msg_sender: s,
            msg_receiver: r,
        }//end struct construction
    }//end initialize()
}//end impl for GUI
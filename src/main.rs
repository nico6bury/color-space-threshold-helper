use color_space_threshold_helper::{enums::InterfaceMessage, process::ImageToProc};
use fltk::{enums::ColorDepth, image::{RgbImage, SharedImage}};
use gui::GUI;

mod gui;

fn main() {
    println!("Hello world!");
    // set up gui-related variables
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();

    // do the main application loop
    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::Quit) => gui.quit(),
            Some(InterfaceMessage::Reset) => println!("Reset not yet supported"),
            Some(InterfaceMessage::LoadImage(img_path)) => {
                println!("Got told to load an image at {}", img_path.to_string_lossy());
                match ImageToProc::read_image(img_path) {
                    Err(err) => println!("Couldn't load image because ImageError: {:?}",err),
                    Ok(img) => {
                        let (data,w,h,color_type) = img.deconstruct();
                        let color_type = match color_type {
                            image::ColorType::L8 => Some(ColorDepth::L8),
                            image::ColorType::La8 => Some(ColorDepth::La8),
                            image::ColorType::Rgb8 => Some(ColorDepth::Rgb8),
                            image::ColorType::Rgba8 => Some(ColorDepth::Rgba8),
                            _ => None,
                        };
                        match color_type {
                            None => println!("Image has unsupported color depth!"),
                            Some(color_type) => {
                                match RgbImage::new(data,w as i32,h as i32,color_type) {
                                    Err(err) => println!("Couldn't convert to FLTK image because FLTK error: {:?}", err),
                                    Ok(rgb_img) => {
                                        match SharedImage::from_image(rgb_img){ 
                                            Err(err) => println!("Couldn't load image because couldn't convert rgb to shared. FLTKError: {:?}", err),
                                            Ok(shared_img) => gui.load_image(shared_img)
                                                .unwrap_or_else(|e| println!("Couldn't load image because BorrowMutError: {:?}",e)),
                                        }//end matching whether we can convert RgbImage to SharedImage
                                    },//end case that we can create an RgbImage
                                }//end matching whether we can create an RgbImage
                            },//end case that color_depth is valid
                        }//end matching whether we have valid color_depth to assign
                    },//end case that we can read image from path
                }//end matching whether we can read image from path
            },
            Some(InterfaceMessage::ThreshColor(color)) => println!("Thresh COlor not yet supported. {:?}",color),
            Some(InterfaceMessage::DoThreshold) => println!("Got told to do thresholding, but that's not implemented yet."),
            None => (),
        }//end matching message received
    }//end main app loop
    println!("Goodbye world!");
}//end main method

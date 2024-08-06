use color_space_threshold_helper::{enums::InterfaceMessage, process::ImageToProc};
use fltk::{enums::ColorDepth, image::{RgbImage, SharedImage}};
use gui::GUI;
use image::DynamicImage;

mod gui;

fn main() {
    println!("Hello world!");
    // set up gui-related variables
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();
    // setup variables to hold data during the program
    let mut last_img_opened = ImageToProc::blank();
    let mut last_th_color: (u8,u8,u8) = (255,0,0);
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
                        match dynamic_img_to_shared_img(img.get_image()) {
                            Err(err) => println!("{err}"),
                            Ok(shared_img) => match gui.load_image(shared_img) {
                                Err(err) => println!("Couldn't load image because of BorrowMutError: {:?}",err),
                                Ok(_) => last_img_opened = img,
                            }//end case that we can convert dynamic to shared image, try loading it
                        }//end matching whether we can convert from dynamic to shared image
                    },//end case that we can read image from path
                }//end matching whether we can read image from path
            },
            Some(InterfaceMessage::ThreshColor(color)) => last_th_color = color,
            Some(InterfaceMessage::DoThreshold) => {
                match gui.get_thresh_params() {
                    None => println!("Couldn't get threshold parameters."),
                    Some(th_params) => {
                        let th_img = last_img_opened.threshold_img(th_params, last_th_color);
                        match dynamic_img_to_shared_img(&th_img) {
                            Err(err) => println!("Couldn't load thresholded image because: {err}"),
                            Ok(shared_image) => gui.load_image(shared_image)
                                .unwrap_or_else(|e| println!("Couldn't load image because of BorrowMutError: {:?}",e))
                        }//end matching whether we can convert the dynamic image to a shared image
                    },//end case that we can get the thresholding parameters 
                }//end matching whether we can get thresholding parameters
            },
            None => (),
        }//end matching message received
    }//end main app loop
    println!("Goodbye world!");
}//end main method

fn dynamic_img_to_shared_img(
    dynamic_img: &DynamicImage
) -> Result<SharedImage, String> {
    let (data,w,h,color_type) = ImageToProc::deconstruct(dynamic_img);
    let color_type = match color_type {
        image::ColorType::L8 => Some(ColorDepth::L8),
        image::ColorType::La8 => Some(ColorDepth::La8),
        image::ColorType::Rgb8 => Some(ColorDepth::Rgb8),
        image::ColorType::Rgba8 => Some(ColorDepth::Rgba8),
        _ => None,
    };
    match color_type {
        None => Err(format!("Image has unsupported color depth!")),
        Some(color_type) => {
            match RgbImage::new(data,w as i32,h as i32,color_type) {
                Err(err) => Err(format!("Couldn't convert to FLTK image because FLTK error: {:?}", err)),
                Ok(rgb_img) => {
                    match SharedImage::from_image(rgb_img){ 
                        Err(err) => Err(format!("Couldn't load image because couldn't convert rgb to shared. FLTKError: {:?}", err)),
                        Ok(shared_img) => Ok(shared_img),
                    }//end matching whether we can convert RgbImage to SharedImage
                },//end case that we can create an RgbImage
            }//end matching whether we can create an RgbImage
        },//end case that color_depth is valid
    }//end matching whether we have valid color_depth to assign
}//end dynamic_components_to_shared_image
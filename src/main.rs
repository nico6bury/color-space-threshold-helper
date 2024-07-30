use fltk::image::SharedImage;
use gui::{InterfaceMessage, GUI};

mod gui;

fn main() {
    println!("Hello world!");
    let mut gui = GUI::initialize();
    let recv = gui.get_receiver();
    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::LoadImage(img_path)) => {
                println!("Got told to load an image at {}", img_path.to_string_lossy());
                match SharedImage::load(img_path) {
                    Err(err) => println!("Couldn't load image because FLTKError: {:?}", err),
                    Ok(img) => {
                        gui.load_image(img)
                            .unwrap_or_else(|e| println!("Couldn't load image because BorrowMutError: {:?}",e));
                    },
                }//end matching whether we loaded the shared image
            },
            Some(InterfaceMessage::Quit) => gui.quit(),
            None => (),
        }//end matching message received
    }//end main app loop
    println!("Goodbye world!");
}//end main method

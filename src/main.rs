use gui::{InterfaceMessage, GUI};

mod gui;

fn main() {
    let gui = GUI::initialize();
    let recv = gui.get_receiver();
    println!("Hello, world!");
    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::LoadImage(img_path)) => {
                println!("Got told to load an image at {}", img_path.to_string_lossy());
            },
            Some(InterfaceMessage::Quit) => gui.quit(),
            None => (),
        }//end matching message received
    }//end main app loop
}//end main method

use gui::{InterfaceMessage, GUI};

mod gui;

fn main() {
    println!("Hello world!");
    let gui = GUI::initialize();
    let recv = gui.get_receiver();
    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::LoadImage(img_path)) => {
                println!("Got told to load an image at {}", img_path.to_string_lossy());
            },
            Some(InterfaceMessage::Quit) => gui.quit(),
            None => (),
        }//end matching message received
    }//end main app loop
    println!("Goodbye world!");
}//end main method

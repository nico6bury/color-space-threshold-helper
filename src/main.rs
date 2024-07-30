use gui::{InterfaceMessage, GUI};

mod gui;

fn main() {
    let gui = GUI::initialize();
    let recv = gui.get_receiver();
    println!("Hello, world!");
    while gui.wait() {
        match recv.recv() {
            Some(InterfaceMessage::LoadImage) => {
                println!("Got told to load an image");
            },
            Some(InterfaceMessage::Quit) => gui.quit(),
            None => (),
        }//end matching message received
    }//end main app loop
}//end main method

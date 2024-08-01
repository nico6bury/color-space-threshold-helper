use std::path::PathBuf;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceMessage {
    LoadImage(PathBuf),
    Reset,
    /// Contains an rbg color
    ThreshColor((u8,u8,u8)),
    Quit
}//end enum InterfaceMessage


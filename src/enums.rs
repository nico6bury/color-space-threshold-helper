use std::path::PathBuf;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceMessage {
    LoadImage(PathBuf),
    Reset,
    /// Contains an rbg color
    ThreshColor((u8,u8,u8)),
    DoThreshold,
    Quit
}//end enum InterfaceMessage

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorSpace {
    RGB,
    HSBorHSV,
    HSL,
    HSI,
    LabCIE,
    YUV,
}//end enum ColorSpace

impl ColorSpace {
    pub fn from_str(str: &str) -> Option<ColorSpace> {
        match str {
            "RGB" => Some(ColorSpace::RGB),
            "HSB" | "HSV" | "HSBorHSV" => Some(ColorSpace::HSBorHSV),
            "HSL" => Some(ColorSpace::HSL),
            "HSI" => Some(ColorSpace::HSI),
            "LabCIE" => Some(ColorSpace::LabCIE),
            "YUV" => Some(ColorSpace::YUV),
            _ => None,
        }//end matching str
    }//end from_str()

    pub fn get_variants() -> Vec<String> {
        vec![
            "RGB",
            "HSB or HSV",
            "HSL",
            "HSI",
            "LabCIE",
            "YUV",
        ].iter().map(|s| s.to_string()).collect()
    }//end get_variants()
}//end impl for ColorSpace
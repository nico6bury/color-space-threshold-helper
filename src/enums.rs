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

/// This enum represents the potential color space options
/// for the user to threshold.
/// In order to add a new ColorSpace, edit the following:
/// - variants in ColorSpace
/// - each implemented function of ColorSpace
/// - process::convert_from_rgb()
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
#[non_exhaustive]
pub enum ColorSpace {
    RGB,
    HSBorHSV,
    HSL,
    HSI,
    LabCIE,
    YUV,
    CMY,
    HunterLab,
    LCH,
    LUV,
    XYZ,
    YXY,
}//end enum ColorSpace

impl ColorSpace {
    pub fn from_str(str: &str) -> Option<ColorSpace> {
        match str {
            "RGB" => Some(ColorSpace::RGB),
            "HSB" | "HSV" | "HSB or HSV" => Some(ColorSpace::HSBorHSV),
            "HSL" => Some(ColorSpace::HSL),
            "HSI" => Some(ColorSpace::HSI),
            "LabCIE" => Some(ColorSpace::LabCIE),
            "YUV" => Some(ColorSpace::YUV),
            "CMY" => Some(ColorSpace::CMY),
            "HunterLab" => Some(ColorSpace::HunterLab),
            "LCH" => Some(ColorSpace::LCH),
            "LUV" => Some(ColorSpace::LUV),
            "XYZ" => Some(ColorSpace::XYZ),
            "YXY" => Some(ColorSpace::YXY),
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
            "CMY",
            "HunterLab",
            "LCH",
            "LUV",
            "XYZ",
            "YXY",
        ].iter().map(|s| s.to_string()).collect()
    }//end get_variants()

    pub fn channel_names(&self) -> [String; 3] {
        match self {
            ColorSpace::RGB => ["Red".to_string(),"Green".to_string(),"Blue".to_string()],
            ColorSpace::HSBorHSV => ["Hue".to_string(),"Saturation".to_string(),"Brightness/Value".to_string()],
            ColorSpace::HSL => ["Hue".to_string(),"Saturation".to_string(),"Lightness".to_string()],
            ColorSpace::HSI => ["Hue".to_string(),"Saturation".to_string(),"Intensity".to_string()],
            ColorSpace::LabCIE => ["L".to_string(),"a".to_string(),"b".to_string()],
            ColorSpace::YUV => ["Y".to_string(),"U".to_string(),"V".to_string()],
            ColorSpace::CMY => ["Cyan".to_string(),"Magenta".to_string(),"Yellow".to_string()],
            ColorSpace::HunterLab => ["".to_string(),"".to_string(),"".to_string()],
            ColorSpace::LCH => ["Luminance".to_string(),"Chroma".to_string(),"Hue".to_string()],
            ColorSpace::LUV => ["Luminance".to_string(),"U".to_string(),"V".to_string()],
            ColorSpace::XYZ => ["X".to_string(),"Y".to_string(),"Z".to_string()],
            ColorSpace::YXY => ["Y".to_string(),"X".to_string(),"Y".to_string()],
        }//end matching self to channel names
    }//end channel_names()
}//end impl for ColorSpace
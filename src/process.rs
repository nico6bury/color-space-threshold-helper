use std::path::PathBuf;

use image::{ColorType, DynamicImage, ImageError, ImageReader};

use crate::enums::ColorSpace;

pub struct ThreshParams {
    pub color_space: ColorSpace,
    
    pub depth1_min: u8,
    pub depth1_max: u8,
    pub depth1_pass: bool,
    
    pub depth2_min: u8,
    pub depth2_max: u8,
    pub depth2_pass: bool,

    pub depth3_min: u8,
    pub depth3_max: u8,
    pub depth3_pass: bool,
}//end struct ThreshParams

#[derive(Clone,Debug,PartialEq)]
pub struct ImageToProc {
    img: DynamicImage,
}

impl ImageToProc {
    /// Attempts to read the image into an ImageToProc
    pub fn read_image(path: PathBuf) -> Result<ImageToProc, ImageError> {
        Ok(
            ImageToProc{
                img: ImageReader::open(path)?.decode()?,
            }
        )
    }//end read_image()

    /// Deconstructs the image into some components that could
    /// be used to make an image of another type.
    pub fn deconstruct(&self) -> (&[u8],u32,u32,ColorType) {
        (
            self.img.as_bytes(),
            self.img.width(),
            self.img.height(),
            self.img.color(),
        )
    }//end deconstruct()

    /// Creates a small, blank image
    pub fn blank() -> ImageToProc {
        ImageToProc {
            img: DynamicImage::new(1, 1, ColorType::Rgb8)
        }//end struct construction
    }//end blank()

}//end impl for ImageToProc
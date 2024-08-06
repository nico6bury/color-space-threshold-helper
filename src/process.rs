use std::path::PathBuf;

use color_space::{Cmy, FromRgb, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageError, ImageReader, Rgba};

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

    /// Gets reference to contained image object
    pub fn get_image(&self) -> &DynamicImage {&self.img}

    /// Deconstructs the image into some components that could
    /// be used to make an image of another type.
    pub fn deconstruct(img: &DynamicImage) -> (&[u8],u32,u32,ColorType) {
        (
            img.as_bytes(),
            img.width(),
            img.height(),
            img.color(),
        )
    }//end deconstruct()

    /// Creates a small, blank image
    pub fn blank() -> ImageToProc {
        ImageToProc {
            img: DynamicImage::new(1, 1, ColorType::Rgb8)
        }//end struct construction
    }//end blank()

    /// Thresholds the image, returning a new image that has been thresholded
    pub fn threshold_img(&self, params: ThreshParams, thresh_color: (u8,u8,u8)) -> DynamicImage {
        let mut new_img = DynamicImage::new(self.img.width(), self.img.height(), self.img.color());
        self.img.pixels()
            .for_each(|(x,y,mut value)|{
            let converted_pixel = convert_from_rgb(&value, params.color_space);
            if !ImageToProc::is_pixel_in_threshold(converted_pixel, &params) {
                value.0[0] = thresh_color.0;
                value.0[1] = thresh_color.1;
                value.0[2] = thresh_color.2;
                value.0[3] = 255;
            }//end if we should mark pixel with thresh_color
            new_img.put_pixel(x, y, value);
        });

        return new_img;
    }//end threshold_img()

    /// Returns true if the pixel is within the threshold given, false otherwise
    fn is_pixel_in_threshold(pixel: [u8; 3], params: &ThreshParams) -> bool {
        let d1 = pixel[0];
        let d2 = pixel[1];
        let d3 = pixel[2];
        let d1_b = match params.depth1_pass {
            true => d1 >= params.depth1_min && d1 <= params.depth1_max,
            false => d1 <= params.depth1_min && d1 >= params.depth1_max,
        };
        let d2_b = match params.depth2_pass {
            true => d2 >= params.depth2_min && d2 <= params.depth2_max,
            false => d2 <= params.depth2_min && d2 >= params.depth2_max,
        };
        let d3_b = match params.depth3_pass {
            true => d3 >= params.depth3_min && d3 <= params.depth3_max,
            false => d3 <= params.depth3_min && d3 >= params.depth3_max,
        };
        d1_b && d2_b && d3_b
    }//end is_pixel_in_threshold
}//end impl for ImageToProc

/// Converts depth-3 rgb values into target color space
pub fn convert_from_rgb<>(pixel: &Rgba<u8>, target: ColorSpace) -> [u8; 3] {
    // TODO: Finish implementation
    let d1 = pixel.0[0] as f64;
    let d2 = pixel.0[1] as f64;
    let d3 = pixel.0[2] as f64;
    let rgb = Rgb::new(d1,d2,d3);
    let c64: [f64; 3];
    match target {
        ColorSpace::RGB => c64 = [d1,d2,d3],
        ColorSpace::HSBorHSV => {
            let hsv = Hsv::from_rgb(&rgb);
            c64 = [hsv.h,hsv.s,hsv.v];
        },
        ColorSpace::HSL => {
            let hsl = Hsl::from_rgb(&rgb);
            c64 = [hsl.h,hsl.s,hsl.l];
        },
        ColorSpace::HSI => {
            todo!();
        },
        ColorSpace::LabCIE => {
            let lab = Lab::from_rgb(&rgb);
            c64 = [lab.l,lab.a,lab.b];
        },
        ColorSpace::YUV => {
            todo!();
        },
        ColorSpace::CMY => {
            let cmy = Cmy::from_rgb(&rgb);
            c64 = [cmy.c,cmy.m,cmy.y];
        },
        ColorSpace::HunterLab => {
            let hunter_lab = HunterLab::from_rgb(&rgb);
            c64 = [hunter_lab.l,hunter_lab.a,hunter_lab.b];
        },
        ColorSpace::LCH => {
            let lch = Lch::from_rgb(&rgb);
            c64 = [lch.l,lch.c,lch.h];
        },
        ColorSpace::LUV => {
            let luv = Luv::from_rgb(&rgb);
            c64 = [luv.l,luv.u,luv.v];
        },
        ColorSpace::XYZ => {
            let xyz = Xyz::from_rgb(&rgb);
            c64 = [xyz.x,xyz.y,xyz.z];
        },
        ColorSpace::YXY => {
            let yxy = Yxy::from_rgb(&rgb);
            c64 = [yxy.y1,yxy.x,yxy.y2];
        },
    };//end matching based on target color space

    [
        c64[0].ceil().min(255.).max(0.) as u8,
        c64[1].ceil().min(255.).max(0.) as u8,
        c64[2].ceil().min(255.).max(0.) as u8,
    ]
}//end convert_from_rgb()
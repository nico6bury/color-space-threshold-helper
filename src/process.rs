use core::f64;
use std::path::PathBuf;

// use color_space::{Cmy, FromRgb, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageError, ImageReader};
use palette::{FromColor, GetHue, Hsl, Hsv, Lab, Srgb};

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
                let converted_pixel = convert_from_rgb([value.0[0],value.0[1],value.0[2]], params.color_space);
                if !ImageToProc::is_pixel_in_threshold(converted_pixel, &params) {
                    value.0[0] = thresh_color.0;
                    value.0[1] = thresh_color.1;
                    value.0[2] = thresh_color.2;
                    value.0[3] = 255;
                }//end if we should mark pixel with thresh_color
                new_img.put_pixel(x, y, value);
            }
        );

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
pub fn convert_from_rgb(u8_rgb: [u8; 3], target: ColorSpace) -> [u8; 3] {
    let r = u8_rgb[0];
    let g = u8_rgb[1];
    let b = u8_rgb[2];
    let big_rgb: Srgb<f64> = Srgb::new(r as f64, g as f64, b as f64);
    let c8: [u8; 3];
    match target {
        ColorSpace::RGB => c8 = [r,g,b],
        ColorSpace::HSBorHSV => {
            let hsv = Hsv::from_color(big_rgb);
            // get values in variables
            let mut h = hsv.get_hue().into_positive_degrees();
            let mut s = hsv.saturation;
            // issues with palette conversion, so we do it ourself
            let v = r.max(g).max(b);
            // Convert all values to scale of [0,255]
            h = (h * 255.) / 360.;
            s = (s * 255.) / 1.;
            // Convert all values to u8
            let h = h.ceil() as u8;
            let s = s.ceil() as u8;
            // c64 = [hsv.h,hsv.s,hsv.v];
            c8 = [h,s,v];
        },
        ColorSpace::HSL => {
            let hsl = Hsl::from_color(big_rgb);
            // get values in variables
            let mut h = hsl.get_hue().into_positive_degrees();
            let mut s = hsl.saturation;
            let mut l = hsl.lightness;
            // Convert all values to scale of [0,255]
            h = (h * 255.) / 360.;
            s = (s * 255.) / 1.;
            l = (l * 255.) / 1.;
            // Convert all values to u8
            let h = h.ceil() as u8;
            let s = s.ceil() as u8;
            let l = l.ceil() as u8;
            c8 = [h,s,l];
        },
        ColorSpace::HSI => {
            // conversion formula from:
            // http://eng.usf.edu/~hady/courses/cap5400/rgb-to-hsi.pdf
            let r = r as f64;
            let g = g as f64;
            let b = b as f64;
            let r = r / (r + g + b);
            let g = g / (r + g + b);
            let b = b / (r + g + b);
            let h;
            if b <= g {
                h = (
                    0.5 * ((r - g) + (r - b))
                    /
                    ( (r - g).powi(2) + (r - b) * (g - b) ).powf(0.5)
                ).acos();
            } else {
                h = 2. * f64::consts::PI - (
                    0.5 * ( (r - g) + (r - b) )
                    /
                    ( (r - g).powi(2) + (r - b) * (g - b) ).powf(0.5)
                ).acos();
            }
            let s = 1. - 3. * r.min(g).min(b);
            let i = (r + g + b) / (3. * 255.);
            // convert values into ranges of [0,360], [0,100], [0,255]
            let h = h * 180. / f64::consts::PI;
            let s = s * 100.;
            let i = i * 255.;
            // normalize values into ranges of [0,255],[0,255],[0,255]
            let h = (h * 255.) / 360.;
            let s = (s * 255.) / 100.;
            let i = i;
            c8 = [h as u8,s as u8,i as u8];
        },
        ColorSpace::LabCIE => {
            let lab = Lab::from_color(big_rgb);
            // get values in variables
            let mut l = lab.l;
            let mut a = lab.a;
            let mut b = lab.b;
            // Convert all values to scale of [0,255]
            l = (l * 255.) / 100.;
            a += 128.;
            a = a.min(255.).max(0.);
            b += 128.;
            b = b.min(255.).max(0.);
            // convert all values to u8
            let l = l.ceil() as u8;
            let a = a.ceil() as u8;
            let b = b.ceil() as u8;
            c8 = [l,a,b];
        },
        ColorSpace::YUV => {
            // conversion formula taken from:
            // https://softpixel.com/~cwright/programming/colorspace/yuv/
            let r = r as f64;
            let g = g as f64;
            let b = b as f64;
            // do the coefficient calculations
            let y = r * 0.299000 + g * 0.587000 + b * 0.114000;
            let u = r * -0.168736 + g * -0.331264 + b * 0.5 + 128.;
            let v = r * 0.500000 + g * -0.418688 + b * -0.081312 + 128.;
            // convert to u8
            let y = y.ceil() as u8;
            let u = u.ceil() as u8;
            let v = v.ceil() as u8;
            c8 = [y,u,v];
        },
    };//end matching based on target color space

    return c8;
}//end convert_from_rgb()
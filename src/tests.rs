use crate::{enums::ColorSpace, process};

const ALLOWED_DIFF: u8 = 1;

#[test]
pub fn hsb_conversion_white() {
    let rgb = [255,255,255];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(0),hsb[1].abs_diff(0),hsb[2].abs_diff(255)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb white", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb white", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb white", dif[2]);
}

#[test]
pub fn hsb_conversion_black() {
    let rgb = [0,0,0];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(0),hsb[1].abs_diff(0),hsb[2].abs_diff(0)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb black", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb black", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb black", dif[2]);
}

#[test]
pub fn hsb_conversion_red() {
    let rgb = [255,80,80];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(0),hsb[1].abs_diff(174),hsb[2].abs_diff(255)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb red", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb red", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb red", dif[2]);
}

#[test]
pub fn hsb_conversion_blue() {
    let rgb = [102,0,204];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(191),hsb[1].abs_diff(255),hsb[2].abs_diff(204)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb blue", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb blue", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb blue", dif[2]);
}

#[test]
pub fn hsb_conversion_yellow() {
    let rgb = [255,255,204];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(42),hsb[1].abs_diff(51),hsb[2].abs_diff(255)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb yellow", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb yellow", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb yellow", dif[2]);
}

#[test]
pub fn hsb_conversion_green() {
    let rgb = [0,153,51];
    let hsb = process::convert_from_rgb(rgb, ColorSpace::HSBorHSV);
    let dif = [hsb[0].abs_diff(99),hsb[1].abs_diff(255),hsb[2].abs_diff(153)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsb green", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsb green", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsb green", dif[2]);
}

#[test]
pub fn hsl_conversion_white() {
    let rgb = [255,255,255];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(0),hsl[1].abs_diff(0),hsl[2].abs_diff(255)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl white", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl white", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl white", dif[2]);
}

#[test]
pub fn hsl_conversion_black() {
    let rgb = [0,0,0];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(0),hsl[1].abs_diff(0),hsl[2].abs_diff(0)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl black", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl black", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl black", dif[2]);
}

#[test]
pub fn hsl_conversion_red() {
    let rgb = [255,80,80];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(0),hsl[1].abs_diff(255),hsl[2].abs_diff(167)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl red", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl red", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl red", dif[2]);
}

#[test]
pub fn hsl_conversion_blue() {
    let rgb = [102,0,204];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(191),hsl[1].abs_diff(255),hsl[2].abs_diff(102)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl blue", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl blue", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl blue", dif[2]);
}

#[test]
pub fn hsl_conversion_yellow() {
    let rgb = [255,255,204];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(42),hsl[1].abs_diff(255),hsl[2].abs_diff(229)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl yellow", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl yellow", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl yellow", dif[2]);
}

#[test]
pub fn hsl_conversion_green() {
    let rgb = [0,153,51];
    let hsl = process::convert_from_rgb(rgb, ColorSpace::HSL);
    let dif = [hsl[0].abs_diff(197),hsl[1].abs_diff(255),hsl[2].abs_diff(76)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsl green", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsl green", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsl green", dif[2]);
}

#[test]
pub fn hsi_conversion_white() {
    let rgb = [255,255,255];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(0),hsi[1].abs_diff(0),hsi[2].abs_diff(255)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi white", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi white", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi white", dif[2]);
}

#[test]
pub fn hsi_conversion_black() {
    let rgb = [0,0,0];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(0),hsi[1].abs_diff(0),hsi[2].abs_diff(0)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi black", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi black", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi black", dif[2]);
}

#[test]
pub fn hsi_conversion_red() {
    let rgb = [255,80,80];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(0),hsi[1].abs_diff(107),hsi[2].abs_diff(138)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi red", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi red", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi red", dif[2]);
}

#[test]
pub fn hsi_conversion_blue() {
    let rgb = [102,0,204];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(197),hsi[1].abs_diff(255),hsi[2].abs_diff(102)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi blue", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi blue", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi blue", dif[2]);
}

#[test]
pub fn hsi_conversion_yellow() {
    let rgb = [255,255,204];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(42),hsi[1].abs_diff(36),hsi[2].abs_diff(237)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi yellow", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi yellow", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi yellow", dif[2]);
}

#[test]
pub fn hsi_conversion_green() {
    let rgb = [0,153,51];
    let hsi = process::convert_from_rgb(rgb, ColorSpace::HSI);
    let dif = [hsi[0].abs_diff(98),hsi[1].abs_diff(255),hsi[2].abs_diff(68)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] hsi green", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] hsi green", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] hsi green", dif[2]);
}

#[test]
pub fn lab_conversion_white() {
    let rgb = [255,255,255];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(255),lab[1].abs_diff(127),lab[2].abs_diff(127)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab white", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab white", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab white", dif[2]);
}

#[test]
pub fn lab_conversion_black() {
    let rgb = [0,0,0];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(0),lab[1].abs_diff(127),lab[2].abs_diff(127)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab black", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab black", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab black", dif[2]);
}

#[test]
pub fn lab_conversion_red() {
    let rgb = [255,80,80];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(151),lab[1].abs_diff(193),lab[2].abs_diff(165)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab red", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab red", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab red", dif[2]);
}

#[test]
pub fn lab_conversion_blue() {
    let rgb = [102,0,204];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(82),lab[1].abs_diff(197),lab[2].abs_diff(49)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab blue", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab blue", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab blue", dif[2]);
}

#[test]
pub fn lab_conversion_yellow() {
    let rgb = [255,255,204];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(252),lab[1].abs_diff(119),lab[2].abs_diff(151)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab yellow", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab yellow", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab yellow", dif[2]);
}

#[test]
pub fn lab_conversion_green() {
    let rgb = [0,153,51];
    let lab = process::convert_from_rgb(rgb, ColorSpace::LabCIE);
    let dif = [lab[0].abs_diff(140),lab[1].abs_diff(72),lab[2].abs_diff(170)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] lab green", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] lab green", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] lab green", dif[2]);
}

#[test]
pub fn yuv_conversion_white() {
    let rgb = [255,255,255];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(255),yuv[1].abs_diff(128),yuv[2].abs_diff(128)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv white", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv white", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv white", dif[2]);
}

#[test]
pub fn yuv_conversion_black() {
    let rgb = [0,0,0];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(0),yuv[1].abs_diff(128),yuv[2].abs_diff(128)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv black", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv black", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv black", dif[2]);
}

#[test]
pub fn yuv_conversion_red() {
    let rgb = [255,80,80];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(132),yuv[1].abs_diff(98),yuv[2].abs_diff(215)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv red", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv red", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv red", dif[2]);
}

#[test]
pub fn yuv_conversion_blue() {
    let rgb = [102,0,204];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(53),yuv[1].abs_diff(212),yuv[2].abs_diff(162)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv blue", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv blue", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv blue", dif[2]);
}

#[test]
pub fn yuv_conversion_yellow() {
    let rgb = [255,255,204];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(249),yuv[1].abs_diff(102),yuv[2].abs_diff(132)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv yellow", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv yellow", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv yellow", dif[2]);
}

#[test]
pub fn yuv_conversion_green() {
    let rgb = [0,153,51];
    let yuv = process::convert_from_rgb(rgb, ColorSpace::YUV);
    let dif = [yuv[0].abs_diff(95),yuv[1].abs_diff(102),yuv[2].abs_diff(59)];
    assert!(dif[0] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[0] yuv green", dif[0]);
    assert!(dif[1] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[1] yuv green", dif[1]);
    assert!(dif[2] <= ALLOWED_DIFF, "Diff was {}, higher than allowed for diff[2] yuv green", dif[2]);
}
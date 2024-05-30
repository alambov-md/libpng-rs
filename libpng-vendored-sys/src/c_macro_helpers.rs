#![allow(non_snake_case)]
use crate::{
    png_image, PNG_FORMAT_FLAG_ALPHA, PNG_FORMAT_FLAG_COLOR, PNG_FORMAT_FLAG_COLORMAP,
    PNG_FORMAT_FLAG_LINEAR,
};

pub fn PNG_IMAGE_FAILED(png_cntrl: &png_image) -> bool {
    (png_cntrl.warning_or_error & 0x03) > 1
}

pub fn PNG_IMAGE_SAMPLE_CHANNELS(fmt: u32) -> usize {
    ((fmt & (PNG_FORMAT_FLAG_COLOR | PNG_FORMAT_FLAG_ALPHA)) + 1) as usize
}

pub fn PNG_IMAGE_SAMPLE_COMPONENT_SIZE(fmt: u32) -> usize {
    (((fmt & PNG_FORMAT_FLAG_LINEAR) >> 2) + 1) as usize
}

pub fn PNG_IMAGE_PIXEL_(test: impl FnOnce(u32) -> usize, fmt: u32) -> usize {
    if (fmt & PNG_FORMAT_FLAG_COLORMAP) > 0 {
        1
    } else {
        test(fmt) as usize
    }
}

pub fn PNG_IMAGE_PIXEL_CHANNELS(fmt: u32) -> usize {
    PNG_IMAGE_PIXEL_(PNG_IMAGE_SAMPLE_CHANNELS, fmt)
}

pub fn PNG_IMAGE_PIXEL_COMPONENT_SIZE(fmt: u32) -> usize {
    PNG_IMAGE_PIXEL_(PNG_IMAGE_SAMPLE_COMPONENT_SIZE, fmt)
}

pub fn PNG_IMAGE_ROW_STRIDE(image: &png_image) -> usize {
    PNG_IMAGE_PIXEL_CHANNELS(image.format) * (image.width as usize)
}

pub fn PNG_IMAGE_BUFFER_SIZE(image: &png_image, row_stride: usize) -> usize {
    PNG_IMAGE_PIXEL_COMPONENT_SIZE(image.format) * (image.height as usize) * row_stride
}

pub fn PNG_IMAGE_SIZE(image: &png_image) -> usize {
    PNG_IMAGE_BUFFER_SIZE(image, PNG_IMAGE_ROW_STRIDE(image))
}

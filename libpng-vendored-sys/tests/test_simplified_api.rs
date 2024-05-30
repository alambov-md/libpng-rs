use std::{
    ffi::CString,
    fs::read,
    path::PathBuf,
    ptr::{null, null_mut},
};

use libc::c_void;

use libpng_vendored_sys::{
    png_image, png_image_begin_read_from_file, png_image_begin_read_from_memory,
    png_image_finish_read, png_image_free, PNG_FORMAT_RGBA, PNG_IMAGE_FAILED, PNG_IMAGE_SIZE,
    PNG_IMAGE_VERSION,
};

#[test]
fn test_png_image_read_from_file() {
    let mut image = empty_image();

    let status =
        unsafe { png_image_begin_read_from_file(&mut *image, test_image_c_string().as_ptr()) };

    assert_eq!(status, 1);
    assert!(!PNG_IMAGE_FAILED(&image));

    image.format = PNG_FORMAT_RGBA;

    let mut buffer = vec![0_u8; PNG_IMAGE_SIZE(&image)];

    let status = unsafe {
        png_image_finish_read(
            &mut *image,
            null(),
            buffer.as_mut_ptr() as *mut c_void,
            0,
            null_mut(),
        )
    };

    assert_eq!(status, 1);
    assert!(!PNG_IMAGE_FAILED(&image));
    assert!(image.opaque.is_null());

    unsafe { png_image_free(&mut *image) };
}

#[test]
fn test_png_image_read_from_memory() {
    let image_vec = read(test_image_path()).unwrap();

    let mut image = empty_image();

    let status = unsafe {
        png_image_begin_read_from_memory(
            &mut *image,
            image_vec.as_ptr() as *const c_void,
            image_vec.len(),
        )
    };

    assert_eq!(status, 1);
    assert!(!PNG_IMAGE_FAILED(&image));

    image.format = PNG_FORMAT_RGBA;

    let mut buffer = vec![0_u8; PNG_IMAGE_SIZE(&image)];

    let status = unsafe {
        png_image_finish_read(
            &mut *image,
            null(),
            buffer.as_mut_ptr() as *mut c_void,
            0,
            null_mut(),
        )
    };

    assert_eq!(status, 1);
    assert!(!PNG_IMAGE_FAILED(&image));
    assert!(image.opaque.is_null());

    unsafe { png_image_free(&mut *image) };
}

fn empty_image() -> Box<png_image> {
    Box::new(png_image {
        opaque: null_mut(),
        version: PNG_IMAGE_VERSION,
        width: 0,
        height: 0,
        format: 0,
        flags: 0,
        colormap_entries: 0,
        warning_or_error: 0,
        message: [0; 64],
    })
}

fn test_image_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test.png")
}

fn test_image_c_string() -> CString {
    CString::new(test_image_path().to_str().unwrap()).unwrap()
}

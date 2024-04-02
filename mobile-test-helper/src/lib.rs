use std::ptr::{null, null_mut};

use libc::{c_char, c_void};
use libpng_vendored_sys::*;

#[allow(dead_code)]
#[path = "../../libpng-vendored-sys/tests/c_macro_helpers.rs"]
mod c_macro_helpers;

use c_macro_helpers::PNG_IMAGE_SIZE;

#[no_mangle]
pub unsafe extern "C" fn test_read_from_png_file_to_memory(path_ptr: *const c_char) -> i32 {
    let mut image = empty_image();

    let status = png_image_begin_read_from_file(&mut *image, path_ptr);
    if status != 1 {
        return status;
    }

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

    png_image_free(&mut *image);

    return status;
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

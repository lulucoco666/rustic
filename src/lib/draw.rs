/* 
This function is adapted from the pygame library's C implementation for drawing.
The original C code was used for drawing primitives like lines on an SDL surface.
Here, we have translated and adapted a part of that functionality into Rust,
using the SDL2 crate for handling graphics in Rust.
*/

use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

fn add_pixel_to_drawn_list(x: i32, y: i32, pts: &mut [i32; 4]) {
    if x < pts[0] {
        pts[0] = x;
    }
    if y < pts[1] {
        pts[1] = y;
    }
    if x > pts[2] {
        pts[2] = x;
    }
    if y > pts[3] {
        pts[3] = y;
    }
}


fn add_pixel_to_drawn_list(x: i32, y: i32, pts: &mut [i32; 4]) {
    if x < pts[0] {
        pts[0] = x;
    }
    if y < pts[1] {
        pts[1] = y;
    }
    if x > pts[2] {
        pts[2] = x;
    }
    if y > pts[3] {
        pts[3] = y;
    }
}

fn add_line_to_drawn_list(x1: i32, y1: i32, x2: i32, pts: &mut [i32; 4]) {
    if x1 < pts[0] {
        pts[0] = x1;
    }
    if y1 < pts[1] {
        pts[1] = y1;
    }
    if x2 > pts[2] {
        pts[2] = x2;
    }
 
    if y2 > pts[3] { 
        pts[3] = y2;
    }
}



fn draw_horz_line(surf: &mut Surface, color: u32, x1: i32, y1: i32, x2: i32) {
    let pitch = surf.pitch() as usize;
    let bytes_per_pixel = surf.pixel_format_enum().byte_size_per_pixel() as usize;
    let pixels = surf.without_lock_mut().unwrap();

    let start = (pitch * y1 as usize) + (x1 as usize * bytes_per_pixel);
    let end = start + ((x2 - x1) as usize * bytes_per_pixel);

    match bytes_per_pixel {
        1 => {
            for i in (start..end).step_by(bytes_per_pixel) {
                pixels[i] = color as u8;
            }
        }
        2 => {
            for i in (start..end).step_by(bytes_per_pixel) {
                let color16 = color as u16;
                pixels[i..i+2].copy_from_slice(&color16.to_ne_bytes());
            }
        }
        3 => {
            let color_bytes = color.to_ne_bytes();
            #[cfg(target_endian = "big")]
            let color_bytes = &color_bytes[1..]; // skip the first byte for big endian

            for i in (start..end).step_by(bytes_per_pixel) {
                pixels[i..i+3].copy_from_slice(&color_bytes[..3]);
            }
        }
        4 => {
            for i in (start..end).step_by(bytes_per_pixel) {
                pixels[i..i+4].copy_from_slice(&color.to_ne_bytes());
            }
        }
        _ => {}
    }
}


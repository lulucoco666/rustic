/* 
This function is adapted from the pygame library's C implementation for drawing.
The original C code was used for drawing primitives like lines on an SDL surface.
Here, we have translated and adapted a part of that functionality into Rust,
using the SDL2 crate for handling graphics in Rust.
*/
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



extern crate palette;
extern crate byteorder;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use image::*;
use bmp::*;
pub mod image;
pub mod bmp;



fn main() {
    let image_file_path = Path::new("D:\\Alexander\\programming\\ricex\\sample_images\\4.2.03.bmp");
    let display = image_file_path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&image_file_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    let file_byte_iter = file.bytes();

    let new_img = match bmp::bmp_decode(file_byte_iter) {
        Err(why) => panic!("couldn't decode image: {}", why),
        Ok(img) => img,
    };
    new_img.print();
}

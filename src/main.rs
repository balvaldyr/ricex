#[macro_use]
extern crate kiss_ui;
extern crate palette;
extern crate byteorder;
extern crate stopwatch;
use kiss_ui::container::Horizontal;
use kiss_ui::dialog::Dialog;
use kiss_ui::image::{ImageContainer};
use kiss_ui::text::Label;
use std::error::Error;
use std::env;
use std::fs::{self, File, DirEntry, ReadDir};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use stopwatch::Stopwatch;
use jpeg_enc::*;
use image::*;
use bmp::*;
pub mod image;
pub mod bmp;
pub mod get_slice;
pub mod jpeg_enc;



fn main() {

    // let image_file_path = Path::new("D:\\Alexander\\programming\\ricex\\sample_images\\4.2.03.bmp");

    let cur_dir = env::current_dir().unwrap();
    let mut p : PathBuf = cur_dir.clone();
    p.push("sample_images");
    println!("The current directory is {}", p.as_path().display());

    let bmpdir: ReadDir = match fs::read_dir(&p) {
        Ok(s) => s,
        Err(why) => {
            panic!("couldn't open {}: {}",
                   p.display(),
                   Error::description(&why))
        }
    };


            let sw = Stopwatch::start_new();
    for entry in bmpdir {
        let image_file_path = entry.unwrap().path();
        let display = image_file_path.display();
        // print!("> {}",
        //        image_file_path.file_name().unwrap().to_str().unwrap());
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&image_file_path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut file_byte_vec: Vec<u8> = Vec::new();
        let file_size = match file.read_to_end(&mut file_byte_vec) {
            Ok(size) => size,
            Err(_) => 0,
        };

        if file_size > 0 {
            let new_img = match bmp::bmp_decode(file_byte_vec) {
                Err(why) => panic!("couldn't decode image: {}", why),
                Ok(img) => img,
            };
            // println!(" ({} x {})  ", new_img.width, new_img.height);


    		let mut path : PathBuf = cur_dir.clone();
            path.push("outputs");
            path.push(image_file_path.file_name().unwrap());
            path.set_extension("jpg");
		    let display = path.as_path().display();

		    // Open a file in write-only mode, returns `io::Result<File>`
		    let mut file = match File::create(&path) {
		        Err(why) => panic!("couldn't create {}: xx {} xx",
		                           display,
		                           why.raw_os_error().unwrap()),
		        Ok(file) => file,
		    };

		    // save_to_jpeg(& mut file, &new_img);
		    show_image(new_img.width, new_img.height, new_img.to_tuple_vec());
        }

    }
            println!("bmp::bmp_decode took {}ms", sw.elapsed_ms());

}


fn show_image(width : u32, height : u32, image_data : Vec<(u8,u8,u8)>) {
    // const WIDTH: u32 = 512;
    // const HEIGHT: u32 = 512;

    // let image_data: Vec<_> = (0..HEIGHT)
    //     .flat_map(|y| (0..WIDTH).map(move |x| color(x, y)))
    //     .collect();

    kiss_ui::show_gui(|| {
        Dialog::new(
            Horizontal::new(
                children![
                    Label::new("")
                        .set_image(kiss_ui::image::Image::new_rgb(width, height, &image_data)),
                ]
            )
        )
        .set_title(format!("Image! ({width} x {height})", width=width, height=height))
    });
}

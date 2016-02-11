use palette::Rgb;
use palette::Color;
use std::fs::File;
use std::io::Bytes;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use image::Image;

pub fn bmp_decode(stream: Bytes<File>) -> Result<Image, &'static str> {

    let mut header = BmpHeader::new();
    for byte_result in stream {
        // Safety first
        let safe_byte = match byte_result {
            Ok(d) => d,
            _ => return Err("unexpected eof"),
        };

        let header_loaded = match header.is_loaded() {
            Ok(loaded) => loaded,
            _ => return Err("header error"),
        };
        // if header is fully loaded push data to pixel vector
        // else push to header
        if !header_loaded {
            push_to_header(&mut header, safe_byte);
            println!("offset = {}", header.file_header_data_offset());
        }
    }

    let x = &header.header_vec[1..4];
    // for i in 0.. {
    //     if i == 2 {
    //         fs = cb as u32;
    //     }
    //     if i == 3 {
    //         fs |= (cb as u32) << 8;
    //     }
    //     if i == 4 {
    //         fs |= (cb as u32) << 16;
    //     }
    //     if i == 5 {
    //         fs |= (cb as u32) << 24;
    //     }
    //     if i == 6 {
    //         println!("File Size = {}", fs)
    //     }
    //
    //     if (i > 6) && (i == fs - 1) {
    //         println!("safe exit");
    //         break;
    //     }
    //     // print!("{:#x} ", cb);
    // }
    // println!("");

    let img = img_gen();

    return Ok(img);
}


struct BmpHeader {
    offset_count: u32,
    header_vec: Vec<u8>,
}

fn push_to_header(header: &mut BmpHeader, data: u8) {
    header.header_vec.push(data);
    header.offset_count += 1;
}

impl BmpHeader {
    pub fn new() -> BmpHeader {
        // assert_eq((width*height) as usize, pixels.len);
        BmpHeader {
            offset_count: 0,
            header_vec: Vec::new(),
        }
    }

    pub fn is_loaded(&self) -> Result<bool, &'static str> {
        if self.file_header_data_offset() == self.offset_count {
            Ok(true)
        } else if self.offset_count > 100 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Getters
    fn file_header_data_offset(&self) -> u32 {
        let mut offset = 14;
        if self.offset_count >= 14 {
            let relvec = &self.header_vec[10..14];
            let mut rdr = Cursor::new(relvec);
            offset = rdr.read_u32::<LittleEndian>().unwrap();
        }
        return offset;
    }
}







// TEST STUFF

fn img_gen() -> Image {
    let rgb_vec: Vec<Color> = vec![RED, RED, GREEN, GREEN, BLUE, BLUE, YELLOW, YELLOW];
    let im1 = Image::new(2, 4, rgb_vec);
    return im1;
}

const RED: Color = Color::Rgb(Rgb {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
});
const GREEN: Color = Color::Rgb(Rgb {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
});
const BLUE: Color = Color::Rgb(Rgb {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
});
const YELLOW: Color = Color::Rgb(Rgb {
    red: 1.0,
    green: 1.0,
    blue: 0.0,
});

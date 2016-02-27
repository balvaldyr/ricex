use std::fs::File;
use std::io::Bytes;
use palette::Rgb;
use palette::Color;
// modules within Ricex
use image::Image;
use get_slice::ByteToUnsigned;
use byteorder::{LittleEndian, BigEndian};


pub fn bmp_decode(stream: Bytes<File>) -> Result<Image, &'static str> {

    let mut header = BmpHeader::new();

    let (btsz, opt_btmax) = stream.size_hint();

    println!("btsz = {}", btsz);

    for byte_result in &stream {
        // Safety first
        let safe_byte = match byte_result {
            Ok(d) => d,
            _ => return Err("unexpected eof"),
        };

        let header_loaded = match header.is_loaded() {
            Ok(loaded) => loaded,
            _ => return Err("header error"),
        };
        // keep pushing bytes to header until fully loaded
        if !header_loaded {
            header.push(safe_byte);
        } else {
            break;
        }
    }

    let image_width = header.image_width();
    let image_height = header.image_height();
    let pixel_size = header.image_pixel_size();
    let row_size = header.image_row_size();


    println!("Start of Data!");
    println!("width = {}", image_width);
    println!("height = {}", image_height);
    println!("Pixel Size = {}", pixel_size);
    println!("Row Size = {}", row_size);

    // for byte_result in stream {
    //     // Safety first
    //     let safe_byte = match byte_result {
    //         Ok(d) => d,
    //         _ => return Err("unexpected eof"),
    //     };

    //     let header_loaded = match header.is_loaded() {
    //         Ok(loaded) => loaded,
    //         _ => return Err("header error"),
    //     };

    // }




    let img = img_gen();

    return Ok(img);
}


struct BmpHeader {
    offset_count: usize,
    header_vec: Vec<u8>,
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

    pub fn push(&mut self, data: u8) {
        self.header_vec.push(data);
        self.offset_count += 1;
    }


    // Getters
    fn file_header_data_offset(&self) -> usize {
        LittleEndian::get_u32_from_vector(&(self.header_vec), 10).unwrap_or(14) as usize
    }

    // File Header Types:
    // Supported :
    //      BM – 0x424D - Windows 3.1x, 95, NT, ... etc.
    //      BA – 0x4241 - OS/2 struct bitmap array
    // Unsupported :
    //      CI – OS/2 struct color icon
    //      CP – OS/2 const color pointer
    //      IC – OS/2 struct icon
    //      PT – OS/2 pointer

    fn file_header_header_type(&self) -> u16 {
        BigEndian::get_u16_from_vector(&(self.header_vec), 0).unwrap_or(0)
    }

    pub fn image_width(&self) -> i32 {
        match self.file_header_header_type() {
            0x424D => LittleEndian::get_i32_from_vector(&(self.header_vec), 18).unwrap_or(0),
            0x4241 => LittleEndian::get_u16_from_vector(&(self.header_vec), 18).unwrap_or(0) as i32,
            _ => 0,
        }
    }

    pub fn image_height(&self) -> i32 {
        match self.file_header_header_type() {
            0x424D => LittleEndian::get_i32_from_vector(&(self.header_vec), 22).unwrap_or(0),
            0x4241 => LittleEndian::get_u16_from_vector(&(self.header_vec), 20).unwrap_or(0) as i32,
            _ => 0,
        }
    }

    pub fn image_pixel_size(&self) -> u16 {
        match self.file_header_header_type() {
            0x424D => LittleEndian::get_u16_from_vector(&(self.header_vec), 28).unwrap_or(0),
            0x4241 => LittleEndian::get_u16_from_vector(&(self.header_vec), 24).unwrap_or(0),
            _ => 0,
        }
    }

    pub fn image_row_size(&self) -> u32 {
        let pix_size : u32 = self.image_pixel_size() as u32;
        let pixels_per_row : u32 = self.image_width().abs() as u32;
        ((pix_size * pixels_per_row + 31)/32) * 4
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

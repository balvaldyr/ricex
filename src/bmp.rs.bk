use palette::Rgb;
use palette::Color;
// modules within Ricex
use image::Image;
use get_slice::ByteToUnsigned;
use byteorder::{LittleEndian, BigEndian};


pub fn bmp_decode(stream: Vec<u8>) -> Result<Image, &'static str> {

    let mut header = BmpHeader::new();


    for byte_result in &stream {
        let header_loaded = match header.is_loaded() {
            Ok(loaded) => loaded,
            _ => return Err("header error"),
        };
        // keep pushing bytes to header until fully loaded
        if !header_loaded {
            header.push(*byte_result);
        } else {
            break;
        }
    }

    let image_width = header.image_width();
    let image_height = header.image_height();
    let pixel_size = header.image_pixel_size();
    let row_size: usize = header.image_row_size();

    // println!("Start of Data!");
    // println!("width = {}", image_width);
    // println!("height = {}", image_height);
    // println!("Pixel Size = {}", pixel_size);
    // println!("Row Size = {}", row_size);

    let header_offset = header.file_header_data_offset();

    // println!("Size of vec {}", stream.len());

    let data_stream = &stream[header_offset..];
    // println!("Size of vec {}", data_stream.len());


    // // raw_image_rows is a vector of slices, each corresponding to individual row data
    // // make sure our row length times row count is equal to the available data
    //
    // let mut raw_image_rows: Vec<&[u8]> = data_stream.chunks(row_size).collect();

    // // if image_height is positive then rows are in bottom to top order, so reverse order
    // if image_height.is_positive() {
    //     raw_image_rows.reverse();
    // }

    // let mut img_vec : Vec<Color> = Vec::new();

    // // The bottleneck
    // for row_slice in raw_image_rows {
    //     bmp_row_process(row_slice, pixel_size, & mut img_vec);
    // }


    // raw_image_rows is a vector of slices, each corresponding to individual row data
    // make sure our row length times row count is equal to the available data
    assert_eq!(data_stream.len(), (row_size * (image_height as usize)));
    let mut img_vec: Vec<Color> = Vec::new();

    // Decide row order based on sign of image_height
    let row_iterator: Box<Iterator<Item = &[u8]>> = match image_height.is_positive() {
        true => Box::new(data_stream.chunks(row_size).rev()),
        false => Box::new(data_stream.chunks(row_size)),
    };

    for row_slice in row_iterator {
        bmp_row_process(row_slice, pixel_size, &mut img_vec);
    }

    // let img = img_gen();
    let img = Image::new(image_width as u32, image_height.abs() as u32, img_vec);

    return Ok(img);
}


// 1, 4, 8, 16, 24 and 32 bits per pixel

fn bmp_row_process(row_slice: &[u8], pix_size: u16, vec_collector: &mut Vec<Color>) {

    if pix_size == 24 {
        // start with 24 bits per pixel... easiest
        let mut red: u8 = 0;
        let mut gre: u8 = 0;
        for (i, ubyte) in row_slice.iter().enumerate() {
            match i % 3 {
                0 => red = *ubyte,
                1 => gre = *ubyte,
                2 => vec_collector.push(Color::Rgb(Rgb::new_u8(red, gre, *ubyte))),
                _ => panic!("Remainder when dividing by 3 is 3 or greater!"),
            }
        }
    } else {
        panic!("Only handle pixel size 24");
    }
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

    pub fn image_row_size(&self) -> usize {
        let pix_size: u32 = self.image_pixel_size() as u32;
        let pixels_per_row: u32 = self.image_width().abs() as u32;
        (((pix_size * pixels_per_row + 31) / 32) * 4) as usize
    }
}

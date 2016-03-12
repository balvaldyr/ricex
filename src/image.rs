use palette::{Color, Rgb, IntoColor};
use palette::pixel::RgbPixel;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

fn get_tuple3(p : Rgb) -> (u8,u8,u8) {
    (float_to_u8(p.red), float_to_u8(p.green), float_to_u8(p.blue))
}

impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<Color>) -> Image {
        // assert_eq((width*height) as usize, pixels.len);
        Image {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx: usize = (x + self.width * y) as usize;
                let active: Color = self.pixels[idx];
                let active = Rgb::from(active);
                print!("({},{},{})",
                       float_to_u8(active.red),
                       float_to_u8(active.green),
                       float_to_u8(active.blue))
            }
            println!("");
        }
    }

    pub fn to_tuple_vec(&self) -> Vec<(u8,u8,u8)> {
        self.pixels.iter().map(|&x| get_tuple3(Rgb::from(x))).collect()
    }
}



fn float_to_u8(f: f32) -> u8 {
    let x: u8 = match f {
        n @ 0.0 ... 1.0 => (255.0 * n).round() as u8,
        _ => 0,
    };
    return x;
}



use palette::Color;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Image {
	pub fn new(width: u32, height: u32) -> Image {
		Image {
			width: width,
			height: height,
			pixels: vec![],
		}
	}
}

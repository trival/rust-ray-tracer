use crate::math_utils::Vec3;

pub struct Image {
	width: usize,
	height: usize,
	data: Vec<Vec3>,
}

impl Image {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			data: vec![Vec3::ZERO; width * height],
		}
	}

	pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
		self.data[y * self.width + x] = color;
	}

	pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
		self.data[y * self.width + x]
	}

	pub fn to_ppm(&self) -> String {
		let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

		for color in &self.data {
			let r = (color.x * 255.0).round() as u8;
			let g = (color.y * 255.0).round() as u8;
			let b = (color.z * 255.0).round() as u8;
			ppm.push_str(&format!("{} {} {}\n", r, g, b));
		}
		ppm
	}
}

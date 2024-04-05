pub mod geometry;
pub mod image;
pub mod math_utils;
pub mod renderer;
pub mod shader_lib;

pub mod utils {
	pub fn to_static<T>(t: T) -> &'static T {
		Box::leak(Box::new(t))
	}
}

pub use geometry::*;
pub use image::*;
pub use math_utils::*;
pub use renderer::*;
pub use shader_lib::*;
pub use utils::*;

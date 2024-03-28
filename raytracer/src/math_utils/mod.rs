pub use glam::{
	dvec2 as vec2, dvec3 as vec3, f64::DVec3 as Vec3, DQuat as Quat, DVec2 as Vec2, DVec4 as Vec4,
};
use rand::random;

pub trait Vec3Utils {
	fn is_zero(&self) -> bool;
	fn reflect(self, n: Vec3) -> Vec3;
	fn random() -> Vec3;
	fn random_in_unit_sphere() -> Vec3;
	fn random_unit() -> Vec3;
}

impl Vec3Utils for Vec3 {
	fn is_zero(&self) -> bool {
		self.length_squared() < 0.0001
	}

	fn reflect(self, n: Vec3) -> Vec3 {
		self - 2. * self.dot(n) * n
	}

	fn random() -> Vec3 {
		vec3(random::<f64>(), random::<f64>(), random::<f64>())
	}

	fn random_in_unit_sphere() -> Vec3 {
		loop {
			let p = Vec3::random() * 2. - 1.;
			if p.length_squared() < 1. {
				return p;
			}
		}
	}

	fn random_unit() -> Vec3 {
		Self::random_in_unit_sphere().normalize_or(Vec3::Z)
	}
}

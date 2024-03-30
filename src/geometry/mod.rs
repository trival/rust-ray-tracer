use std::f64::consts::PI;

use crate::math_utils::*;

#[derive(Clone, Copy)]
pub struct Ray {
	pub origin: Vec3,
	pub dir: Vec3,
}

impl Ray {
	pub fn new(origin: Vec3, dir: Vec3) -> Self {
		Self {
			origin,
			dir: dir.normalize(),
		}
	}

	pub fn at(&self, t: f64) -> Vec3 {
		self.origin + self.dir * t
	}
}

#[derive(Clone, Copy)]
pub struct HitData {
	pub t: f64,
	pub point: Vec3,
	pub normal: Vec3,
	pub uv: Vec2,
}

pub trait Hittable: Sync + Send {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
	center: Vec3,
	radius: f64,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64) -> Self {
		Self { center, radius }
	}

	pub fn normal_at(&self, point: Vec3) -> Vec3 {
		(point - self.center).normalize()
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData> {
		let oc = ray.origin - self.center;
		let half_b = oc.dot(ray.dir);
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = half_b * half_b - c;

		if discriminant < 0.0 {
			return None;
		}

		let sqrt_d = discriminant.sqrt();
		let t1 = -half_b - sqrt_d;
		let t2 = -half_b + sqrt_d;

		let t = if t1 >= min_t && t2 >= min_t {
			t1.min(t2)
		} else if t1 > 0.0 {
			t1
		} else {
			t2
		};

		if t >= min_t && t < max_t {
			let point = ray.at(t);
			let normal = self.normal_at(point);
			let uv = Vec2::ZERO; // TODO
			Some(HitData {
				t,
				point,
				normal,
				uv,
			})
		} else {
			None
		}
	}
}

#[derive(Clone, Copy)]
pub struct Plane {
	origin: Vec3,
	normal: Vec3,

	// precalculated
	d: f64,
}

impl Plane {
	pub fn new(origin: Vec3, normal_dir: Vec3) -> Self {
		let n = normal_dir.normalize();
		Self {
			origin,
			normal: n,
			d: n.dot(origin),
		}
	}

	pub fn intersect(&self, ray: &Ray) -> f64 {
		let denom = self.normal.dot(ray.dir);
		if denom.abs() > 1e-6 {
			let t = (self.d - self.normal.dot(ray.origin)) / denom;
			if t >= 0. {
				return t;
			}
		}
		-1.
	}
}

#[derive(Clone, Copy)]
pub struct Quad {
	plane: Plane,
	u: Vec3,
	v: Vec3,

	// precalculated
	w: Vec3,
}

impl Quad {
	pub fn new(center: Vec3, width: f64, height: f64, rot: Quat) -> Self {
		let u = rot * Vec3::X * width;
		let v = rot * Vec3::Y * height;
		Self::new_uv(center - u / 2. - v / 2., u, v)
	}

	pub fn new_wh(width: f64, height: f64) -> Self {
		Self::new(Vec3::ZERO, width, height, Quat::IDENTITY)
	}

	pub fn new_uv(origin: Vec3, u: Vec3, v: Vec3) -> Self {
		let n = u.cross(v);
		let plane = Plane::new(origin, n);
		let w = n / n.length_squared();
		Self { plane, u, v, w }
	}

	pub fn rotate(&mut self, rotation_origin: Vec3, rot: Quat) {
		self.u = rot * self.u;
		self.v = rot * self.v;
		let n = self.u.cross(self.v);
		let dir = self.plane.origin - rotation_origin;
		let orig = rot * dir + rotation_origin;
		self.plane = Plane::new(orig, n);
		self.w = n / n.length_squared();
	}

	pub fn rotated(&self, rotation_origin: Vec3, rot: Quat) -> Self {
		let mut quad = self.clone();
		quad.rotate(rotation_origin, rot);
		quad
	}

	pub fn rotate_about_center(&mut self, rot: Quat) {
		let center = self.plane.origin + self.u / 2. + self.v / 2.;
		self.rotate(center, rot);
	}

	pub fn rotated_about_center(&self, rot: Quat) -> Self {
		let mut quad = self.clone();
		quad.rotate_about_center(rot);
		quad
	}

	pub fn translate(&mut self, translation: Vec3) {
		self.plane.origin += translation;
	}

	pub fn translated(&self, translation: Vec3) -> Self {
		let mut quad = self.clone();
		quad.translate(translation);
		quad
	}
}

impl Hittable for Quad {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData> {
		let t = self.plane.intersect(ray);
		if t >= min_t && t < max_t {
			let point = ray.at(t);
			let q = point - self.plane.origin;
			let u = self.w.dot(q.cross(self.v)); // can be reused for texture coords
			let v = self.w.dot(self.u.cross(q)); // can be reused for texture coords
			if u >= 0. && u <= 1. && v >= 0. && v <= 1. {
				Some(HitData {
					t,
					point,
					normal: self.plane.normal,
					uv: vec2(u, v),
				})
			} else {
				None
			}
		} else {
			None
		}
	}
}

#[derive(Clone, Copy)]
pub struct Cube {
	quads: [Quad; 6],
	center: Vec3,
}

impl Cube {
	pub fn new(center: Vec3, width: f64, height: f64, depth: f64) -> Self {
		let half_width = width / 2.;
		let half_height = height / 2.;
		let half_depth = depth / 2.;

		let front = Quad::new(
			center + vec3(0., 0., half_depth),
			width,
			height,
			Quat::IDENTITY,
		);
		let back = Quad::new(
			center + vec3(0., 0., -half_depth),
			width,
			height,
			Quat::from_rotation_z(PI),
		);
		let left = Quad::new(
			center + vec3(-half_width, 0., 0.),
			depth,
			height,
			Quat::from_rotation_y(PI / 2.),
		);
		let right = Quad::new(
			center + vec3(half_width, 0., 0.),
			depth,
			height,
			Quat::from_rotation_y(-PI / 2.),
		);
		let top = Quad::new(
			center + vec3(0., half_height, 0.),
			width,
			depth,
			Quat::from_rotation_x(-PI / 2.),
		);
		let bottom = Quad::new(
			center + vec3(0., -half_height, 0.),
			width,
			depth,
			Quat::from_rotation_x(PI / 2.),
		);

		Self {
			quads: [front, back, left, right, top, bottom],
			center,
		}
	}

	pub fn front(&self) -> &Quad {
		&self.quads[0]
	}

	pub fn back(&self) -> &Quad {
		&self.quads[1]
	}

	pub fn left(&self) -> &Quad {
		&self.quads[2]
	}

	pub fn right(&self) -> &Quad {
		&self.quads[3]
	}

	pub fn top(&self) -> &Quad {
		&self.quads[4]
	}

	pub fn bottom(&self) -> &Quad {
		&self.quads[5]
	}

	pub fn rotate(&mut self, rotation_origin: Vec3, rot: Quat) {
		for quad in self.quads.iter_mut() {
			quad.rotate(rotation_origin, rot);
		}
	}

	pub fn rotated(&self, rotation_origin: Vec3, rot: Quat) -> Self {
		let mut box_ = self.clone();
		box_.rotate(rotation_origin, rot);
		box_
	}

	pub fn rotate_about_center(&mut self, rot: Quat) {
		for quad in self.quads.iter_mut() {
			quad.rotate(self.center, rot);
		}
	}

	pub fn rotated_about_center(&self, rot: Quat) -> Self {
		let mut box_ = self.clone();
		box_.rotate_about_center(rot);
		box_
	}

	pub fn translate(&mut self, translation: Vec3) {
		for quad in self.quads.iter_mut() {
			quad.translate(translation);
		}
		self.center += translation;
	}

	pub fn translated(&self, translation: Vec3) -> Self {
		let mut box_ = self.clone();
		box_.translate(translation);
		box_
	}
}

impl Hittable for Cube {
	fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<HitData> {
		let mut closest = None;
		let mut closest_t = max_t;
		for quad in self.quads.iter() {
			if let Some(i) = quad.hit(ray, min_t, closest_t) {
				closest = Some(i);
				closest_t = i.t;
			}
		}
		closest
	}
}

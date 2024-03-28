use crate::geometry::*;
use crate::image::Image;
use crate::math_utils::*;
use rand::random;
use std::thread;

#[derive(Clone, Copy)]
enum Form {
	Sphere(Sphere),
	Quad(Quad),
	Box(Box),
}

#[derive(Clone, Copy)]
struct SceneObject {
	form: Form,
	color: Vec3,
}

#[derive(Clone)]
pub struct Scene {
	objects: Vec<SceneObject>,
}

const MIN_T: f64 = 0.001;
const MAX_T: f64 = 1.0e10;

impl Scene {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	pub fn add_sphere(&mut self, sphere: Sphere, color: Vec3) {
		self.objects.push(SceneObject {
			form: Form::Sphere(sphere),
			color,
		});
	}

	pub fn add_quad(&mut self, quad: Quad, color: Vec3) {
		self.objects.push(SceneObject {
			form: Form::Quad(quad),
			color,
		});
	}

	pub fn add_box(&mut self, _box: Box, color: Vec3) {
		self.objects.push(SceneObject {
			form: Form::Box(_box),
			color,
		});
	}
}

fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Vec3 {
	if depth == 0 {
		return Vec3::ZERO;
	}

	let mut closest_hit: Option<HitData> = None;
	let mut obj_color = Vec3::ZERO;

	for object in &scene.objects {
		let closest_t = if let Some(closest_hit) = closest_hit {
			closest_hit.t
		} else {
			MAX_T
		};

		let hit = match object.form {
			Form::Sphere(ref sphere) => sphere.hit(ray, MIN_T, closest_t),
			Form::Quad(ref quad) => quad.hit(ray, MIN_T, closest_t),
			Form::Box(ref _box) => _box.hit(ray, MIN_T, closest_t),
		};
		if hit.is_some() {
			closest_hit = hit;
			obj_color = object.color;
		}
	}

	if let Some(hit) = closest_hit {
		let reflected_ray = ray.dir.reflect(hit.normal);

		let mut scatter_dir = reflected_ray + Vec3::random_unit() * 0.6;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		let scattered = Ray::new(hit.point, scatter_dir);
		return obj_color * ray_color(&scattered, scene, depth - 1);
	}

	let t = 0.5 * (ray.dir.y + 1.);
	let col1 = Vec3::ONE;
	let col2 = vec3(0.5, 0.7, 1.);
	col1.lerp(col2, t)
}

#[derive(Clone, Copy)]
pub struct Camera {
	origin: Vec3,
	dir: Vec3,
	near: f64,
	up: Vec3,
}

impl Camera {
	pub fn new(origin: Vec3, dir: Vec3, near: f64) -> Self {
		Self {
			origin,
			dir,
			near,
			up: Vec3::Y,
		}
	}

	fn view_port_directions(&self) -> (Vec3, Vec3) {
		let u = self.dir.cross(self.up).normalize();
		let v = self.dir.cross(u).normalize();
		(u, v)
	}

	pub fn render(
		&self,
		scene: &Scene,
		img_width: usize,
		img_height: usize,
		rays_per_pixel: usize,
		max_bounces: usize,
	) -> Image {
		let mut image = Image::new(img_width, img_height);

		let height = img_height as f64 / img_width as f64;

		let pixel_width = 1.0 / img_width as f64;
		let pixel_height = height / img_height as f64;

		let viewport_center = self.origin + self.dir * self.near;
		let (viewport_u, viewport_v) = self.view_port_directions();

		for y in 0..img_height {
			for x in 0..img_width {
				let mut color = Vec3::ZERO;
				for _ in 0..rays_per_pixel {
					let pixel_x = -0.5 + x as f64 * pixel_width + random::<f64>() * pixel_width;
					let pixel_y = -height / 2. + y as f64 * pixel_height + random::<f64>() * pixel_height;
					let ray_dir = viewport_center + viewport_u * pixel_x + viewport_v * pixel_y - self.origin;
					let ray = Ray::new(self.origin, ray_dir);
					color += ray_color(&ray, scene, max_bounces) / rays_per_pixel as f64;
				}
				image.set_pixel(x, y, color);
			}
		}

		image
	}

	pub fn render_parallel(
		&self,
		scene: &Scene,
		img_width: usize,
		img_height: usize,
		rays_per_pixel: usize,
		max_bounces: usize,
		threads: usize,
	) -> Image {
		let mut handles = vec![];

		for _ in 0..threads {
			let s = scene.clone();
			let c = self.clone();
			handles.push(thread::spawn(move || {
				c.render(
					&s,
					img_width,
					img_height,
					rays_per_pixel / threads,
					max_bounces,
				)
			}));
		}

		let mut imgs = Vec::with_capacity(threads);

		for handle in handles {
			imgs.push(handle.join().unwrap());
		}

		for x in 0..img_width {
			for y in 0..img_height {
				let mut color = Vec3::ZERO;
				for img in &imgs {
					color += img.get_pixel(x, y);
				}
				color /= threads as f64;
				imgs[threads - 1].set_pixel(x, y, color);
			}
		}

		imgs.pop().unwrap()
	}
}

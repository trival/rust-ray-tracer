use crate::geometry::*;
use crate::image::Image;
use crate::math_utils::*;
use crate::utils::to_static;
use rand::random;
use std::thread;

pub trait Material: Sync + Send {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray>;
	fn emitted(&self, scattered_color: Option<Vec3>, hit: &HitData) -> Vec3;
}

pub trait Sky: Sync + Send {
	fn shade(&self, ray: &Ray) -> Vec3;
}

pub struct SceneObject {
	form: &'static dyn Hittable,
	material: &'static dyn Material,
}

pub fn obj(form: &'static dyn Hittable, material: &'static dyn Material) -> SceneObject {
	SceneObject { form, material }
}

pub struct Scene {
	objects: &'static [SceneObject],
	sky: &'static dyn Sky,
}

impl Scene {
	pub fn new(sky: &'static dyn Sky, objects: &'static [SceneObject]) -> Self {
		Self { objects, sky }
	}
}

pub struct SceneBuilder {
	objects: Vec<SceneObject>,
	sky: Option<&'static dyn Sky>,
}

pub fn build_scene() -> SceneBuilder {
	SceneBuilder {
		objects: vec![],
		sky: None,
	}
}

impl SceneBuilder {
	pub fn add(mut self, form: &'static dyn Hittable, material: &'static dyn Material) -> Self {
		self.objects.push(obj(form, material));
		self
	}

	pub fn set_sky(mut self, sky: &'static dyn Sky) -> Self {
		self.sky = Some(sky);
		self
	}

	pub fn build(self) -> &'static Scene {
		to_static(Scene::new(
			self.sky.unwrap_or(&DEFAULT_SKY),
			to_static(self.objects),
		))
	}
}

const MIN_T: f64 = 0.001;
const MAX_T: f64 = 1.0e10;

fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Vec3 {
	if depth == 0 {
		return Vec3::ZERO;
	}

	let mut closest_hit: Option<HitData> = None;
	let mut closest_obj: Option<&SceneObject> = None;

	for object in scene.objects {
		let closest_t = if let Some(closest_hit) = closest_hit {
			closest_hit.t
		} else {
			MAX_T
		};

		let hit = object.form.hit(ray, MIN_T, closest_t);
		if hit.is_some() {
			closest_hit = hit;
			closest_obj = Some(object);
		}
	}

	if let Some(hit) = closest_hit {
		let obj = closest_obj.unwrap();
		let scattered = obj.material.scatter(ray, &hit);
		return obj
			.material
			.emitted(scattered.map(|r| ray_color(&r, scene, depth - 1)), &hit);
	}

	scene.sky.shade(ray)
}

pub struct DefaultSky;
impl Sky for DefaultSky {
	fn shade(&self, ray: &Ray) -> Vec3 {
		let t = 0.5 * (ray.dir.y + 1.);
		let col1 = Vec3::ONE;
		let col2 = vec3(0.5, 0.7, 1.);
		col1.lerp(col2, t)
	}
}
pub const DEFAULT_SKY: DefaultSky = DefaultSky;

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
				image.set_pixel(x, y, color.pow(0.8));
			}
		}

		image
	}

	pub fn render_parallel(
		&'static self,
		scene: &'static Scene,
		img_width: usize,
		img_height: usize,
		rays_per_pixel: usize,
		max_bounces: usize,
		threads: usize,
	) -> Image {
		let mut handles = vec![];

		for _ in 0..threads {
			handles.push(thread::spawn(move || {
				self.render(
					scene,
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

pub fn make_camera(origin: Vec3, dir: Vec3, near: f64) -> &'static Camera {
	to_static(Camera::new(origin, dir, near))
}

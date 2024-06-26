use extend::ext;
use glam::{dvec3 as vec3, f64::DVec3 as Vec3};
use rand::random;
use std::thread;

#[ext]
impl Vec3 {
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

struct Ray {
	origin: Vec3,
	dir: Vec3,
}

impl Ray {
	fn new(origin: Vec3, dir: Vec3) -> Self {
		Self {
			origin,
			dir: dir.normalize(),
		}
	}

	fn at(&self, t: f64) -> Vec3 {
		self.origin + self.dir * t
	}
}

#[derive(Clone, Copy)]
struct Sphere {
	center: Vec3,
	radius: f64,
}

impl Sphere {
	fn new(center: Vec3, radius: f64) -> Self {
		Self { center, radius }
	}

	fn normal_at(&self, point: Vec3) -> Vec3 {
		(point - self.center).normalize()
	}

	fn intersect(&self, ray: &Ray) -> f64 {
		let oc = ray.origin - self.center;
		let half_b = oc.dot(ray.dir);
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = half_b * half_b - c;

		if discriminant < 0.0 {
			return -1.0;
		}

		let sqrt_d = discriminant.sqrt();
		let t1 = -half_b - sqrt_d;
		let t2 = -half_b + sqrt_d;

		if t1 > 0.0 && t2 > 0.0 {
			t1.min(t2)
		} else if t1 > 0.0 {
			t1
		} else {
			t2
		}
	}
}

#[derive(Clone, Copy)]
struct Quad {
	origin: Vec3,
	u: Vec3,
	v: Vec3,

	// cached normal
	normal: Vec3,
	d: f64,
	w: Vec3,
}

impl Quad {
	fn new(origin: Vec3, u: Vec3, v: Vec3) -> Self {
		let n = u.cross(v);
		let normal = n.normalize();
		let w = n / n.length_squared();
		Self {
			origin,
			u,
			v,
			normal,
			w,
			d: normal.dot(origin),
		}
	}

	fn intersect(&self, ray: &Ray) -> f64 {
		let denom = self.normal.dot(ray.dir);
		if denom.abs() > 1e-6 {
			let t = (self.d - self.normal.dot(ray.origin)) / denom;
			if t >= 0. {
				let q = ray.at(t) - self.origin;
				let u = self.w.dot(q.cross(self.v)); // can be reused for texture coords
				let v = self.w.dot(self.u.cross(q)); // can be reused for texture coords
				if u >= 0. && u <= 1. && v >= 0. && v <= 1. {
					return t;
				}
			}
		}
		-1.
	}
}

struct Image {
	width: usize,
	height: usize,
	data: Vec<Vec3>,
}

impl Image {
	fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			data: vec![Vec3::ZERO; width * height],
		}
	}

	fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
		self.data[y * self.width + x] = color;
	}

	fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
		self.data[y * self.width + x]
	}

	fn to_ppm(&self) -> String {
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

#[derive(Clone, Copy)]
enum Form {
	Sphere(Sphere),
	Quad(Quad),
}

#[derive(Clone, Copy)]
struct SceneObject {
	form: Form,
	color: Vec3,
}

#[derive(Clone)]
struct Scene {
	objects: Vec<SceneObject>,
}

const MIN_T: f64 = 0.001;
const MAX_T: f64 = 1.0e10;

impl Scene {
	fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	fn add_sphere(&mut self, sphere: Sphere, color: Vec3) {
		self.objects.push(SceneObject {
			form: Form::Sphere(sphere),
			color,
		});
	}

	fn add_quad(&mut self, quad: Quad, color: Vec3) {
		self.objects.push(SceneObject {
			form: Form::Quad(quad),
			color,
		});
	}

	fn closest_object(&self, ray: &Ray) -> Option<(&SceneObject, f64)> {
		let mut closest_t = MAX_T;
		let mut closest_object = None;

		for object in &self.objects {
			let t = match object.form {
				Form::Sphere(ref sphere) => sphere.intersect(ray),
				Form::Quad(ref quad) => quad.intersect(ray),
			};

			if t > MIN_T && t < closest_t {
				closest_t = t;
				closest_object = Some(object);
			}
		}

		closest_object.map(|obj| (obj, closest_t))
	}
}

fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Vec3 {
	if depth == 0 {
		return Vec3::ZERO;
	}

	if let Some((obj, t)) = scene.closest_object(ray) {
		let hit_normal = match obj.form {
			Form::Sphere(ref sphere) => sphere.normal_at(ray.at(t)),
			Form::Quad(ref quad) => quad.normal,
		};
		let reflected_ray = ray.dir.reflect(hit_normal);

		let mut scatter_dir = reflected_ray + Vec3::random_unit() * 0.6;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		let scattered = Ray::new(ray.at(t), scatter_dir);
		return obj.color * ray_color(&scattered, scene, depth - 1);
	}

	let t = 0.5 * (ray.dir.y + 1.);
	let col1 = Vec3::ONE;
	let col2 = vec3(0.5, 0.7, 1.);
	col1.lerp(col2, t)
}

#[derive(Clone, Copy)]
struct Camera {
	origin: Vec3,
	dir: Vec3,
	near: f64,
	up: Vec3,
}

impl Camera {
	fn new(origin: Vec3, dir: Vec3, near: f64) -> Self {
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

	fn render(
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

	fn render_parallel(
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

fn main() {
	let mut scene = Scene::new();
	scene.add_sphere(Sphere::new(Vec3::ZERO, 0.5), vec3(0.8, 0.3, 0.3));
	scene.add_sphere(Sphere::new(vec3(1., 0., 0.), 0.5), vec3(0.3, 0.8, 0.3));
	scene.add_sphere(Sphere::new(vec3(-1., 0., 0.), 0.5), vec3(0.3, 0.8, 0.8));
	scene.add_sphere(Sphere::new(vec3(0., -200.5, 0.), 200.), vec3(0.5, 0.5, 0.5));
	scene.add_quad(
		Quad::new(vec3(-3., -1., -1.), vec3(2., 0., -3.), vec3(0., 3., 0.)),
		vec3(0.8, 0.8, 0.3),
	);

	let cam = Camera::new(vec3(0., 1.5, 4.), vec3(0., -0.3, -1.), 0.7);

	let width = 600;
	let height = 400;
	let rays_per_pixel = 200;
	let max_bounces = 50;
	let threads = 8;

	let img = if threads <= 1 {
		cam.render(&scene, width, height, rays_per_pixel, max_bounces)
	} else {
		cam.render_parallel(&scene, width, height, rays_per_pixel, max_bounces, threads)
	};

	println!("{}", img.to_ppm());
}

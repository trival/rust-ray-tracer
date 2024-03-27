use extend::ext;
use glam::{dvec3, f64::DVec3};
use rand::random;

#[ext]
impl DVec3 {
	fn is_zero(&self) -> bool {
		self.length_squared() < 0.0001
	}

	fn reflect(self, n: DVec3) -> DVec3 {
		self - 2. * self.dot(n) * n
	}

	fn random() -> DVec3 {
		dvec3(random::<f64>(), random::<f64>(), random::<f64>())
	}

	fn random_in_unit_sphere() -> DVec3 {
		loop {
			let p = DVec3::random() * 2. - 1.;
			if p.length_squared() < 1. {
				return p;
			}
		}
	}

	fn random_unit() -> DVec3 {
		Self::random_in_unit_sphere().normalize_or(DVec3::Z)
	}
}

struct Ray {
	origin: DVec3,
	dir: DVec3,
}

impl Ray {
	fn new(origin: DVec3, dir: DVec3) -> Self {
		Self {
			origin,
			dir: dir.normalize(),
		}
	}

	fn at(&self, t: f64) -> DVec3 {
		self.origin + self.dir * t
	}
}

struct Sphere {
	center: DVec3,
	radius: f64,
}

impl Sphere {
	fn new(center: DVec3, radius: f64) -> Self {
		Self { center, radius }
	}

	fn normal_at(&self, point: DVec3) -> DVec3 {
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

struct Image {
	width: usize,
	height: usize,
	data: Vec<DVec3>,
}

impl Image {
	fn new(width: usize, height: usize) -> Self {
		Self {
			width,
			height,
			data: vec![DVec3::ZERO; width * height],
		}
	}

	fn set_pixel(&mut self, x: usize, y: usize, color: DVec3) {
		self.data[y * self.width + x] = color;
	}

	fn get_pixel(&self, x: usize, y: usize) -> DVec3 {
		self.data[y * self.width + x]
	}

	fn to_ppm(&self) -> String {
		let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

		for y in 0..self.height {
			for x in 0..self.width {
				let color = self.get_pixel(x, y);
				let r = (color.x * 255.0).round() as u8;
				let g = (color.y * 255.0).round() as u8;
				let b = (color.z * 255.0).round() as u8;
				ppm.push_str(&format!("{} {} {}\n", r, g, b));
			}
		}
		ppm
	}
}

struct SceneObject {
	sphere: Sphere,
	color: DVec3,
}

struct Scene {
	objects: Vec<SceneObject>,
}

const MIN_T: f64 = 0.001;
const MAX_T: f64 = 1.0e10;

impl Scene {
	fn closest_object(&self, ray: &Ray) -> Option<(&SceneObject, f64)> {
		let mut closest_t = MAX_T;
		let mut closest_object = None;

		for object in &self.objects {
			let t = object.sphere.intersect(ray);
			if t > MIN_T && t < closest_t {
				closest_t = t;
				closest_object = Some(object);
			}
		}
		closest_object.map(|obj| (obj, closest_t))
	}
}

fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> DVec3 {
	if depth == 0 {
		return DVec3::ZERO;
	}

	if let Some((obj, t)) = scene.closest_object(ray) {
		let hit_normal = obj.sphere.normal_at(ray.at(t));
		let reflected_ray = ray.dir.reflect(hit_normal);

		let mut scatter_dir = reflected_ray + DVec3::random_unit() * 0.6;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		let scattered = Ray::new(ray.at(t), scatter_dir);
		obj.color * ray_color(&scattered, scene, depth - 1)
	} else {
		let t = 0.5 * (ray.dir.y + 1.);
		let col1 = DVec3::ONE;
		let col2 = dvec3(0.5, 0.7, 1.);
		col1.lerp(col2, t)
	}
}

struct Camera {
	origin: DVec3,
	dir: DVec3,
	near: f64,
	up: DVec3,
}

impl Camera {
	fn new(origin: DVec3, dir: DVec3, near: f64) -> Self {
		Self {
			origin,
			dir,
			near,
			up: DVec3::Y,
		}
	}

	fn view_port_directions(&self) -> (DVec3, DVec3) {
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
				let mut color = DVec3::ZERO;
				for _ in 0..rays_per_pixel {
					let pixel_x = -0.5 + x as f64 * pixel_width + random::<f64>() * pixel_width;
					let pixel_y =
						-height / 2. + y as f64 * pixel_height + random::<f64>() * pixel_height;
					let ray_dir =
						viewport_center + viewport_u * pixel_x + viewport_v * pixel_y - self.origin;
					let ray = Ray::new(self.origin, ray_dir);
					color += ray_color(&ray, scene, max_bounces) / rays_per_pixel as f64;
				}
				image.set_pixel(x, y, color);
			}
		}

		image
	}
}

fn main() {
	let scene = Scene {
		objects: vec![
			SceneObject {
				sphere: Sphere::new(dvec3(0., 0., 0.), 0.5),
				color: dvec3(0.8, 0.3, 0.3),
			},
			SceneObject {
				sphere: Sphere::new(dvec3(1., 0., 0.), 0.5),
				color: dvec3(0.3, 0.8, 0.3),
			},
			SceneObject {
				sphere: Sphere::new(dvec3(-1., 0., 0.), 0.5),
				color: dvec3(0.3, 0.3, 0.8),
			},
			SceneObject {
				sphere: Sphere::new(dvec3(0., -100.5, 0.), 100.),
				color: dvec3(0.5, 0.5, 0.5),
			},
		],
	};

	let cam = Camera::new(dvec3(0., 0., 3.), dvec3(0., 0., -1.), 0.7);
	let img = cam.render(&scene, 300, 200, 200, 50);

	println!("{}", img.to_ppm());
}

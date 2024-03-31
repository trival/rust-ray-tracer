use raytracer::{utils::to_static, *};

struct Metal {
	color: Vec3,
	shininess: f64,
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray> {
		let reflected_ray = ray.dir.reflect(hit.normal);

		let mut scatter_dir = reflected_ray + Vec3::random_unit() * self.shininess;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		Some(Ray::new(hit.point, scatter_dir))
	}

	fn emitted(&self, scattered: Option<(Ray, Vec3)>, _hit: &HitData) -> Vec3 {
		self.color * scattered.unwrap().1
	}
}

fn metal(color: Vec3, shininess: f64) -> &'static Metal {
	to_static(Metal { color, shininess })
}

struct Light {
	color: Vec3,
}

impl Material for Light {
	fn scatter(&self, _ray: &Ray, _hit: &HitData) -> Option<Ray> {
		None
	}

	fn emitted(&self, _scattered: Option<(Ray, Vec3)>, _hit: &HitData) -> Vec3 {
		self.color
	}
}

fn light(color: Vec3) -> &'static Light {
	to_static(Light { color })
}

fn main() {
	let objects = [
		obj(sphere(Vec3::ZERO, 0.5), metal(vec3(0.8, 0.3, 0.3), 0.6)),
		obj(
			sphere(vec3(1., 0., 0.), 0.5),
			metal(vec3(0.3, 0.8, 0.3), 0.2),
		),
		obj(sphere(vec3(-1., 0., 0.), 0.5), light(vec3(1.9, 1.9, 3.8))),
		obj(
			quad_uv(vec3(-3., -1., -1.), vec3(2., 0., -2.), vec3(0., 3., 0.)),
			metal(vec3(0.9, 0.9, 0.4), 0.05),
		),
		obj(
			sphere(vec3(0., -200.5, 0.), 200.),
			metal(vec3(0.5, 0.5, 0.5), 1.),
		),
	];

	let cam = to_static(Camera::new(vec3(0., 1.5, 4.), vec3(0., -0.3, -1.), 0.7));
	let scene = to_static(Scene::new(&DEFAULT_SKY, to_static(objects)));

	let width = 600;
	let height = 400;
	let rays_per_pixel = 200;
	let max_bounces = 50;
	let threads = 8;

	let img = if threads <= 1 {
		cam.render(scene, width, height, rays_per_pixel, max_bounces)
	} else {
		cam.render_parallel(scene, width, height, rays_per_pixel, max_bounces, threads)
	};

	println!("{}", img.to_ppm());
}

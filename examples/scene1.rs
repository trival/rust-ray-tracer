use raytracer::*;

struct Mat {
	color: Vec3,
}

fn mat(color: Vec3) -> Mat {
	Mat { color }
}

impl Material for Mat {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray> {
		let reflected_ray = ray.dir.reflect(hit.normal);

		let mut scatter_dir = reflected_ray + Vec3::random_unit() * 0.6;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		Some(Ray::new(hit.point, scatter_dir))
	}

	fn emitted(&self, scattered: Option<(Ray, Vec3)>, _hit: &HitData) -> Vec3 {
		self.color * scattered.map_or(Vec3::ZERO, |(_, c)| c)
	}
}

fn main() {
	let mut scene = Scene::new(DEFAULT_SKY);
	scene.add(Sphere::new(Vec3::ZERO, 0.5), mat(vec3(0.8, 0.3, 0.3)));
	scene.add(Sphere::new(vec3(1., 0., 0.), 0.5), mat(vec3(0.3, 0.8, 0.3)));
	scene.add(
		Sphere::new(vec3(-1., 0., 0.), 0.5),
		mat(vec3(0.3, 0.8, 0.8)),
	);
	scene.add(
		Sphere::new(vec3(0., -200.5, 0.), 200.),
		mat(vec3(0.5, 0.5, 0.5)),
	);
	scene.add(
		Quad::new_uv(vec3(-3., -1., -1.), vec3(2., 0., -3.), vec3(0., 3., 0.)),
		mat(vec3(0.9, 0.9, 0.2)),
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

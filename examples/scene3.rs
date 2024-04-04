use std::f64::consts::TAU;

use rand::random;
use raytracer::{utils::to_static, *};

fn rnd() -> f64 {
	random::<f64>()
}

struct Metal {
	color: Vec3,
	roughness: f64,
}
impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray> {
		let d = ray.dir.dot(hit.normal);
		if d < 0. {
			return Some(Ray::new(hit.point, ray.dir));
		}
		let reflected_ray = ray.dir.reflect(hit.normal);

		let mut scatter_dir = reflected_ray + Vec3::random_unit() * self.roughness;
		if scatter_dir.is_zero() {
			scatter_dir = reflected_ray;
		}

		Some(Ray::new(hit.point, scatter_dir))
	}

	fn emitted(&self, scattered: Option<(Ray, Vec3)>, _hit: &HitData) -> Vec3 {
		let (ray, color) = scattered.unwrap();
		if ray.dir == _hit.ray.dir {
			return color;
		}
		self.color * color
	}
}

fn metal(color: Vec3, shininess: f64) -> &'static Metal {
	to_static(Metal {
		color,
		roughness: shininess,
	})
}

fn main() {
	let mut scene = build_scene();
	for _ in 0..10 {
		let w = rnd() * 4. + 2.;
		let h = rnd() * 4. + 2.;
		let mut b = Cube::new(Vec3::ZERO, w, h, rnd() + 0.1);
		b.translate(Vec3::Y * (h / 2.1));
		b.rotate_about_center(Quat::from_rotation_y(rnd() * TAU));
		// b.rotate_about_center(Quat::from_rotation_y(0.25 * PI));
		let col = Vec3::random();
		scene = scene.add(to_static(b), metal(col, rnd()));
	}
	scene = scene.add(
		sphere(vec3(0., -200., 0.), 200.),
		metal(vec3(0.5, 0.5, 0.5), 0.7),
	);

	let cam = make_camera(vec3(0., 5., 10.), vec3(0., -0.3, -1.), 1.2);
	let scene = scene.build();

	let width = 400;
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

use std::f64::consts::TAU;

use rand::random;
use raytracer::*;

fn rnd() -> f64 {
	random::<f64>()
}

struct Metal {
	color: Vec3,
	roughness: f64,
}
impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray> {
		if !hit.is_front {
			return Some(Ray::new(hit.point, ray.dir));
		}

		if rnd() < self.roughness / 2. {
			return Some(Ray::new(hit.point, Vec3::random_in_hemisphere(hit.normal)));
		}

		Some(metallic_scatter(ray, hit, self.roughness))
	}
	fn emitted(&self, scattered: Option<Vec3>, hit: &HitData) -> Vec3 {
		let color = scattered.unwrap();
		if !hit.is_front {
			return color;
		}
		self.color * color
	}
}
fn metal(color: Vec3, roughness: f64) -> &'static Metal {
	to_static(Metal { color, roughness })
}

fn main() {
	let mut boxes = objects();

	for _ in 0..10 {
		let w = rnd() * 4. + 2.;
		let h = rnd() * 4. + 2.;

		let b = Cube::new(Vec3::ZERO, w, h, rnd() + 0.1)
			.translated(Vec3::Y * (h / 2.1))
			.rotated_about_center(Quat::from_rotation_y(rnd() * TAU))
			.to_static();

		let mat = metal(Vec3::random(), rnd());

		boxes.add(b, mat);
	}

	let scene = build_scene()
		.add_all(boxes)
		.add(
			sphere(vec3(0., -200., 0.), 200.),
			metal(vec3(0.5, 0.5, 0.5), 0.7),
		)
		.build();

	let cam = make_camera(vec3(0., 5., 10.), vec3(0., -0.3, -1.), 1.2);

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

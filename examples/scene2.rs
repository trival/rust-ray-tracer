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
		Some(metallic_scatter(ray, hit, self.roughness))
	}
	fn emitted(&self, scattered: Option<Vec3>, _hit: &HitData) -> Vec3 {
		self.color * scattered.unwrap()
	}
}
fn metal(color: Vec3, roughness: f64) -> &'static Metal {
	to_static(Metal { color, roughness })
}

fn main() {
	let mut objs = objects();

	for _ in 0..10 {
		let w = rnd() * 2. + 2.;
		let h = rnd() * 2. + 2.;

		let q = Quad::new_wh(w, h)
			.translated(Vec3::Y * (h / 2.1))
			.rotated_about_center(Quat::from_rotation_y(rnd() * TAU))
			.to_static();

		let mat = metal(Vec3::random(), rnd());

		objs.add(q, mat);
	}

	let scene = build_scene()
		.add_all(objs)
		.add(
			sphere(vec3(0., -200., 0.), 200.),
			metal(vec3(0.5, 0.5, 0.5), 0.5),
		)
		.build();

	let cam = make_camera(vec3(0., 5., 10.), vec3(0., -0.35, -1.), 1.9);

	let width = 400;
	let height = 500;
	let rays_per_pixel = 200;
	let max_bounces = 100;
	let threads = 8;

	let img = if threads <= 1 {
		cam.render(scene, width, height, rays_per_pixel, max_bounces)
	} else {
		cam.render_parallel(scene, width, height, rays_per_pixel, max_bounces, threads)
	};

	println!("{}", img.to_ppm());
}

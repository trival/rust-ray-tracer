use std::f64::consts::TAU;

use rand::random;
use raytracer::*;

fn rnd() -> f64 {
	random::<f64>()
}

fn main() {
	let mut scene = Scene::new();
	for _ in 0..10 {
		let w = rnd() * 4. + 2.;
		let h = rnd() * 4. + 2.;
		let mut b = Box::new(Vec3::ZERO, w, h, rnd() + 0.1);
		b.translate(Vec3::Y * (h / 2.1));
		b.rotate_about_center(Quat::from_rotation_y(rnd() * TAU));
		// b.rotate_about_center(Quat::from_rotation_y(0.25 * PI));
		let col = Vec3::random();
		scene.add_box(b, col);
	}
	scene.add_sphere(Sphere::new(vec3(0., -200., 0.), 200.), vec3(0.5, 0.5, 0.5));

	let cam = Camera::new(vec3(0., 5., 10.), vec3(0., -0.3, -1.), 1.2);

	let width = 400;
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

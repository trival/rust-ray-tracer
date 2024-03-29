use std::f64::consts::TAU;

use rand::random;
use raytracer::*;

fn rnd() -> f64 {
	random::<f64>()
}

fn main() {
	let mut scene = Scene::new();
	for _ in 0..10 {
		let w = rnd() * 2. + 2.;
		let h = rnd() * 2. + 2.;
		let mut q = Quad::new_wh(w, h);
		q.translate(Vec3::Y * (h / 2.1));
		q.rotate_about_center(Quat::from_rotation_y(rnd() * TAU));
		let col = Vec3::random();
		scene.add_quad(q, col);
	}
	scene.add_sphere(Sphere::new(vec3(0., -200., 0.), 200.), vec3(0.5, 0.5, 0.5));

	let cam = Camera::new(vec3(0., 5., 10.), vec3(0., -0.3, -1.), 1.9);

	let width = 400;
	let height = 600;
	let rays_per_pixel = 200;
	let max_bounces = 100;
	let threads = 8;

	let img = if threads <= 1 {
		cam.render(&scene, width, height, rays_per_pixel, max_bounces)
	} else {
		cam.render_parallel(&scene, width, height, rays_per_pixel, max_bounces, threads)
	};

	println!("{}", img.to_ppm());
}

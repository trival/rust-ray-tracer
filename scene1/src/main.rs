use raytracer::*;

fn main() {
	let mut scene = Scene::new();
	scene.add_sphere(Sphere::new(Vec3::ZERO, 0.5), vec3(0.8, 0.3, 0.3));
	scene.add_sphere(Sphere::new(vec3(1., 0., 0.), 0.5), vec3(0.3, 0.8, 0.3));
	scene.add_sphere(Sphere::new(vec3(-1., 0., 0.), 0.5), vec3(0.3, 0.8, 0.8));
	scene.add_sphere(Sphere::new(vec3(0., -200.5, 0.), 200.), vec3(0.5, 0.5, 0.5));
	scene.add_quad(
		Quad::new_uv(vec3(-3., -1., -1.), vec3(2., 0., -3.), vec3(0., 3., 0.)),
		vec3(0.9, 0.9, 0.2),
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

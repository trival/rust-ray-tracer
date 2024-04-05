use raytracer::*;

const LIGHT_POS: Vec3 = vec3(-1., 0., 0.);
const LIGHT_RADIUS: f64 = 0.5;
const LIGHT_DISTANCE: f64 = 15.;

struct Metal {
	color: Vec3,
	roughness: f64,
}
impl Material for Metal {
	fn scatter(&self, ray: &Ray, hit: &HitData) -> Option<Ray> {
		let light_point = LIGHT_POS + Vec3::random_unit() * LIGHT_RADIUS;
		let light_vec = light_point - hit.point;
		let len = light_vec.length();
		let light_strength = 1.0 - (len / LIGHT_DISTANCE);

		if rand::random::<f64>() > 0.25 * light_strength || hit.normal.dot(light_vec) < 0. {
			Some(metallic_scatter(ray, hit, self.roughness))
		} else {
			Some(Ray::new(hit.point, light_vec))
		}
	}
	fn emitted(&self, scattered: Option<Vec3>, _hit: &HitData) -> Vec3 {
		self.color * scattered.unwrap()
	}
}
fn metal(color: Vec3, roughness: f64) -> &'static Metal {
	to_static(Metal { color, roughness })
}

struct Light {
	color: Vec3,
}
impl Material for Light {
	fn scatter(&self, _ray: &Ray, _hit: &HitData) -> Option<Ray> {
		None
	}
	fn emitted(&self, _scattered: Option<Vec3>, _hit: &HitData) -> Vec3 {
		self.color
	}
}
fn light(color: Vec3) -> &'static Light {
	to_static(Light { color })
}

fn main() {
	let cam = make_camera(vec3(0., 1.5, 4.), vec3(0., -0.3, -1.), 0.7);
	let scene = build_scene()
		.add(sphere(Vec3::ZERO, 0.5), metal(vec3(0.8, 0.3, 0.3), 0.6))
		.add(
			sphere(vec3(1., 0., 0.), 0.5),
			metal(vec3(0.3, 0.8, 0.3), 0.2),
		)
		.add(sphere(LIGHT_POS, LIGHT_RADIUS), light(vec3(1.9, 1.9, 3.8)))
		.add(
			quad_uv(vec3(-3., -1., -1.), vec3(2., 0., -2.), vec3(0., 3., 0.)),
			metal(vec3(0.9, 0.9, 0.4), 0.05),
		)
		.add(
			sphere(vec3(0., -200.5, 0.), 200.),
			metal(vec3(0.5, 0.5, 0.5), 1.),
		)
		.build();

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

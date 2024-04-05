use crate::math_utils::*;
use crate::{HitData, Ray};

pub fn metallic_scatter(ray: &Ray, hit: &HitData, roughness: f64) -> Ray {
	let reflected_ray = ray.dir.reflect(hit.normal);

	let mut scatter_dir = reflected_ray + Vec3::random_unit() * roughness;
	if scatter_dir.is_zero() {
		scatter_dir = reflected_ray;
	}

	Ray::new(hit.point, scatter_dir)
}

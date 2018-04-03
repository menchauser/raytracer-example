use ray::Ray;
use hitable::HitRecord;
use vec::{Vec3, dot};
use rand::random;

struct Scatter {
    is_scattered: bool,
    attenuation: Vec3,
    scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Scatter;
}


fn random_in_unit_sphere() -> Vec3 {
    let singular = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let rand_vec = Vec3::new(
            random::<f32>(),
            random::<f32>(),
            random::<f32>(),
        );
        let p = 2.0 * rand_vec - &singular;
        if dot(&p, &p) < 1.0 {
            break p;
        }
    }
}


pub struct Lambertian {
    albedo: Vec3
}

pub fn lambertian(a: Vec3) -> Lambertian {
    Lambertian {
        albedo: a
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Scatter {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - &hit_record.p);
        let attenuation = self.albedo.clone();
        Scatter {
            is_scattered: true,
            attenuation: attenuation,
            scattered: scattered
        }
    }
}
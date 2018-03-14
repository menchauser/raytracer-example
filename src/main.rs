extern crate rand;

mod vec;
mod ray;
mod hitable;
mod sphere;
mod camera;

use vec::*;
use ray::Ray;
use hitable::*;
use sphere::Sphere;
use camera::*;

use std::f32;

static WHITE_COLOR: Vec3 = Vec3 { e: [1.0, 1.0, 1.0] };
static LIGHT_BLUE_COLOR: Vec3 = Vec3 { e: [0.5, 0.7, 1.0] };

fn random_in_unit_sphere() -> Vec3 {
    let singular = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let rand_vec = Vec3::new(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
        let p = 2.0 * rand_vec - &singular;
        if dot(&p, &p) < 1.0 {
            break p;
        }
    }
}

fn color(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            let target = &rec.p + &rec.normal + random_in_unit_sphere();
            let new_ray = Ray::new(rec.p.clone(), target - &rec.p);
            0.5 * color(&new_ray, world, depth + 1)
        }
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * &WHITE_COLOR + t * &LIGHT_BLUE_COLOR
        }
    }
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let cam = Camera::new();

    let objs: Vec<Box<Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];
    let world = HitableList { list: objs };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / (nx as f32);
                let v = (j as f32 + rand::random::<f32>()) / (ny as f32);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }

            col /= ns as f32;

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

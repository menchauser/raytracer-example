extern crate rand;

mod vec;
mod ray;
mod hitable;
mod sphere;
mod camera;

use vec::Vec3;
use ray::Ray;
use hitable::*;
use sphere::Sphere;
use camera::*;

use std::f32;
use rand::Rng;


static WHITE_COLOR: Vec3 = Vec3 { e: [1.0, 1.0, 1.0] };
static LIGHT_BLUE_COLOR: Vec3 = Vec3 { e: [0.5, 0.7, 1.0] };


fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let rec = world.hit(r, 0.0, f32::MAX);
    match rec {
        Some(r) => {
            let n = r.normal;
            0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
        },
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * &WHITE_COLOR + t * &LIGHT_BLUE_COLOR
        },
    }
}


fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let cam = Camera::new();

    let sphere1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0
    );
    let objs: Vec<&Hitable> = vec![&sphere1, &sphere2];
    let world = HitableList { 
        list: objs,
    };

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.next_f32()) / (nx as f32);
                let v = (j as f32 + rng.next_f32()) / (ny as f32);
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }

            col /= ns as f32;

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
mod vec;
mod ray;
mod hitable;
mod sphere;

use vec::Vec3;
use ray::Ray;
use hitable::*;
use sphere::Sphere;

use std::f32;


fn color(r: Ray, world: &Hitable) -> Vec3 {
    let rec = world.hit(r, 0.0, f32::MAX);
    match rec {
        Some(r) => {
            let n = r.normal;
            0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
        },
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        },
    }
}


fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r, &world);

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
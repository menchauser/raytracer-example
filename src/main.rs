mod vec;
mod ray;

use vec::Vec3;
use ray::Ray;


fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}


fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r);

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.9 * col.g()) as u32;
            let ib = (255.9 * col.b()) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
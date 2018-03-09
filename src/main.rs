mod vec;

use vec::*;


fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(
                (i as f32) / (nx as f32),
                (j as f32) / (ny as f32),
                0.2
            );

            let ir = (255.99 * col.r()) as u32;
            let ig = (255.9 * col.g()) as u32;
            let ib = (255.9 * col.b()) as u32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Unit testing for the Canvas types
use super::*;

#[test]
// This test validates the writing of a Canvas
fn test_print_to_ppm() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = ColorRgb::new(0.5, 0.0, 0.0);
    let c2 = ColorRgb::new(0.0, 0.5, 0.0);
    let c3 = ColorRgb::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(Pixel::new(0, 0, c1));
    canvas.write_pixel(Pixel::new(2, 1, c2));
    canvas.write_pixel(Pixel::new(4, 2, c3));
    // TODO: Need to manage path to /images directory
    canvas.write_to_ppm("test_to_print.ppm");
}
#[test]
// This test validates the printing of a rocket trayectory using Vector and colors in a PPM Canvas
fn test_projectile_launch_canvas() {
    // Bring Geometry module into scope for this test
    use crate::geometry::vector::*;

    #[derive(Debug)]
    struct Projectile {
        position: Point3<f64>,
        velocity: Vector3<f64>,
    }

    struct Environment {
        gravity: Vector3<f64>,
        wind: Vector3<f64>,
    }

    let mut proj = Projectile {
        position: Point3::up(),
        velocity: Vector3::new(1.0, 1.8, 0.0).normalized() * 11.25,
    };

    let env = Environment {
        gravity: Vector3::down() / 10f64,
        wind: Vector3::left() / 100f64,
    };

    let mut canvas = Canvas::new(900, 550);

    fn tick<'a, 'b>(env: &'a Environment, proj: &'b mut Projectile) -> &'b mut Projectile {
        proj.position = proj.position + proj.velocity;
        proj.velocity = proj.velocity + env.gravity + env.wind;
        proj
    }

    println!(
        "Launch position: - x: {:^5.2}, y: {:^5.2}, z: {:^5.2}",
        proj.position.x, proj.position.y, proj.position.z
    );

    let mut pixel = Pixel::new(0, 0, ColorRgb::green());
    pixel.x = proj.position.x as usize;
    pixel.y = proj.position.y as usize;
    canvas.write_pixel(Pixel::new(0, 0, ColorRgb::red()));

    while proj.position.y > 0.0 {
        tick(&env, &mut proj);
        if proj.position.y <= 0.0 {
            break;
        }
        // println!(
        //     "Projectile position - x: {:^5.2}, y: {:^5.2}, z: {:^5.2}",
        //     proj.position.x, proj.position.y, proj.position.z
        // );
        // println!(
        //     "[p.x, p.y]: {:^5.2}, {:^5.2} | [c.x, c.y]: {:^5.2}, {:^5.2}",
        //     proj.position.x, proj.position.y, canvas.width, canvas.height
        //     );

        if (proj.position.x as usize) < canvas.width && (proj.position.y as usize) < canvas.height {
            pixel.x = proj.position.x as usize;
            pixel.y = proj.position.y as usize;
            canvas.write_pixel(pixel);
        }
    }
    // println!("========================== End");

    canvas.write_to_ppm("test_projectile_lauch_canvas.ppm");
}

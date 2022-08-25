// Copyright 2022 Rodrigo Santiago.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Unit testing for Vector3 and Point3 types
use super::*;

use super::Axis::XYZ as xyz;

#[test]
// This test validates the construction of the Vector3 and Point3 types
fn vector_and_point_construction_integrity() {
    let v_one = Vector3::one();
    let v_zero = Vector3::zero();
    let v_back = Vector3::back();
    let v_down = Vector3::down();
    let v_forward = Vector3::forward();
    let v_left = Vector3::left();
    let v_right = Vector3::right();
    let v_up = Vector3::up();
    assert_eq!(
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        },
        v_one
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        v_zero
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        },
        v_back
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0
        },
        v_down
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        },
        v_forward
    );
    assert_eq!(
        Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0
        },
        v_left
    );
    assert_eq!(
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0
        },
        v_right
    );
    assert_eq!(
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        },
        v_up
    );
    let p_one = Point3::one();
    let p_zero = Point3::zero();
    let p_back = Point3::back();
    let p_down = Point3::down();
    let p_forward = Point3::forward();
    let p_left = Point3::left();
    let p_right = Point3::right();
    let p_up = Point3::up();
    assert_eq!(
        Point3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        },
        p_one
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_zero
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            w: 1.0
        },
        p_back
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 1.0
        },
        p_down
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 1.0
        },
        p_forward
    );
    assert_eq!(
        Point3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_left
    );
    assert_eq!(
        Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 1.0
        },
        p_right
    );
    assert_eq!(
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 1.0
        },
        p_up
    );
}

#[test]
// This test validates the operation overloading Add, Sub, Div, Equality, Mul, Neg, AddAssign, SubAssign  for the Vector3 and Point3
fn vector_and_point_operator_overloading_integrity() {
    let v1 = Vector3::new(xyz(2.0, 3.5, 4.0));
    let v2 = Vector3::new(xyz(3.0, 7.5, 8.0));
    let v3 = Vector3::new(xyz(2.55555, 7.88888, 9.34343));
    let v4 = Vector3::new(xyz(2.55553, 7.88887, 9.34342));
    let p1 = Point3::new(xyz(2.5, 3.5, 4.5));
    let p2 = Point3::new(xyz(3.0, 7.0, 8.0));
    let p3 = Point3::new(xyz(2.55555, 7.88888, 9.34343));
    let p4 = Point3::new(xyz(2.55553, 7.88887, 9.34342));
    // Add two Vector3
    assert_eq!(v1 + v2,
        Vector3 {
            x: 5.0,
            y: 11.0,
            z: 12.0
        }
    );
    // Subs two Vector3
    assert_eq!(
        v1 - v2,
        Vector3 {
            x: -1.0,
            y: -4.0,
            z: -4.0
        }
    );
    // Mul Vector3 by scalar
    assert_eq!(
        v1 * 3.0,
        Vector3 {
            x: 6.0,
            y: 10.5,
            z: 12.0
        }
    );
    // Validate Equality
    assert!(v3.equal(v4));
    // Div Vector3 by scalar
    assert!(Vector3::new(xyz(1.50, 3.75, 4.0)).equal(v2 / 2.0));
    // Neg Vector3
    assert_eq!(-v1, Vector3::new(xyz(-2.0, -3.5, -4.0)));
    // Sub two Point3
    assert_eq!(p1 - p2, Vector3::new(xyz(-0.5, -3.5, -3.5)));
    // Div Point3 by scalar
    assert!(Point3::new(xyz(1.25, 1.75, 2.25)).equal(p1 / 2.0));
    // Mul Point3 by scalar
    assert!(p3.equal(Point3::new(xyz(2.55553, 7.88887, 9.34342))));
    // Neg Point3
    assert_eq!(-p1, Point3::new(xyz(-2.5, -3.5, -4.5)));
    // AddAssign Vector3 and AddAssign Point3
    let mut vx = Vector3::zero();
    vx += v1;
    vx -= v1;
    assert!(vx.equal(Vector3::zero()));
    // Test chain operators
    println!("{:?}", v1 + v4 - v1 - v3 + (v2 - v4) / 1.522445523);
    println!("{:?}", v3 + p4 + v1);
    println!("{:?}", p1 - p2 / 3.7626374);
    println!("{:?}", p2 - v1);
    println!("{:?}", v2 + v1);
}

#[test]
// This test validates the implementation of the fuctions in the VecOps trait
fn vector_common_operations_integrity() {
    // Magnitude
    let v1 = Vector3::new(xyz(1.0, 2.0, 3.0));
    assert_eq!(v1.magnitude(), 14f64.sqrt());
    // Normalization
    let mut v2 = v1;
    assert_eq!(v2.normalized().magnitude(), 1f64);
    // Dot product
    let a = Vector3::new(xyz(1.0, 2.0, 3.0));
    let b = Vector3::new(xyz(2.0, 3.0, 4.0));
    assert_eq!(Vector3::dot(a, b), 20f64);
    // Cross product
    assert_eq!(Vector3::cross(a, b), Vector3::new(xyz(-1.0, 2.0, -1.0)));
    assert_eq!(Vector3::cross(b, a), Vector3::new(xyz(1.0, -2.0, 1.0)));
    // Min, Max and Get Components
    assert_eq!(a.min_component(), (0, 'x', 1.0));
    assert_eq!(a.max_component(), (2, 'z', 3.0));
    assert_eq!(a.this(0).unwrap(), (0, 'x', 1.0));
    assert_eq!(a.this(9), None);
    assert_eq!(b.this(b.min_component().0).unwrap(), (0, 'x', 2.0));
    assert_eq!(a.this_name('z').unwrap(), (2, 'z', 3.0));
}

#[test]
// This test validates integrity by simulating a rocket launch
fn simulate_rocket_lauch() {
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
        velocity: Vector3::new(xyz(1.0, 1.0, 0.0)).normalized(),
    };

    let env = Environment {
        gravity: Vector3::down() / 10f64,
        wind: Vector3::left() / 100f64,
    };

    fn tick<'a, 'b>(env: &'a Environment, proj: &'b mut Projectile) -> &'b mut Projectile {
        proj.position = proj.position + proj.velocity;
        proj.velocity = proj.velocity + env.gravity + env.wind;
        proj
    }

    println!(
        "Launch position: - x: {:^5.2}, y: {:^5.2}, z: {:^5.2}",
        proj.position.x, proj.position.y, proj.position.z
    );
    while proj.position.y > 0.0 {
        tick(&env, &mut proj);
        if proj.position.y <= 0.0 {
            break;
        }
        println!(
            "Projectile position - x: {:^5.2}, y: {:^5.2}, z: {:^5.2}",
            proj.position.x, proj.position.y, proj.position.z
        );
    }
    println!("========================== End");
}

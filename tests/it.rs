/*!
Integration testing
*/

use ruxel::geometry::{matrix::Matrix4, vector::*};

#[test]
fn first_test() {
    let vec = Vector3::new(1.0, 2.0, 3.4);
    println!("vector: {:?}", vec);
    assert_eq!(vec.x, 1.0);

    let mut vc = Vector3::new(2.0, 3.0, 3.0);
    vc.x = 3.5;
    assert_eq!(vc.x, 3.5);

    let p = Point3::point_new(2.0, 2.0, 2.5);
    println!("point: {:?}", p);

    let vd = Vector3::default();
    println!("{:?}", vd);

    let vec1 = Vector3::one();
    let vec2 = Vector3::one();
    println!("{:#?}", (vec1 + vec2) + vec1);

    let vec1 = Vector3::new(2.0, 3.0, 4.0);
    let vec2 = Vector3::new(3.0, 4.0, 5.0);
    println!("{:#?}", (vec1 * vec2) + vec1);
}

#[test]
fn second_test() {}

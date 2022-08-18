/**
Integration testing
*/
use ruxel::geometry::vector::*;

#[test]
fn first_test() {
    let vec = Vector3::new(1.0, 2.0, 3.4);
    println!("vector: {:?}", vec);
    assert_eq!(vec.x, 1.0);

    let mut vc = Vector3::new(2.0, 3.0, 3.0);
    vc.x = 3.5;
    let x = vc.x;
    assert_eq!(vc.x, 3.5);

    let nmv = Vector3::one();
    let _y = nmv.x;
    println!("{:#?}", x);

    println!("{:#?}", nmv.x);
}

#[test]
fn second_test() {}

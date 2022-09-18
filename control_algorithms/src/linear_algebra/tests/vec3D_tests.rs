use crate::Vec3D;

#[test]
pub fn vec_create_test() {
    let new = Vec3D::new(1.0, 2.0, 3.0);

    assert_eq!(new.x, 1.0);
    assert_eq!(new.y, 2.0);
    assert_eq!(new.z, 3.0);
}

#[test]
pub fn vec_access_by_index_test() {
    let new = Vec3D::new(1.0, 2.0, 3.0);

    assert_eq!(new[1], 1.0);
    assert_eq!(new[2], 2.0);
    assert_eq!(new[3], 3.0);
}

#[test]
#[should_panic]
pub fn vec_panic_access_by_index_out_of_bounds_test() {
    let new = Vec3D::new(1.0, 2.0, 3.0);
    let _panic = new[4];
}

#[test]
pub fn vec_addition_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(1.0, 2.0, 3.0);
    let new = a + b;

    assert_eq!(new[1], 2.0);
    assert_eq!(new[2], 4.0);
    assert_eq!(new[3], 6.0);
}

#[test]
pub fn vec_subtraction_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(1.0, 2.0, 3.0);
    let new = a - b;

    assert_eq!(new[1], 0.0);
    assert_eq!(new[2], 0.0);
    assert_eq!(new[3], 0.0);
}

#[test]
pub fn vec_multiplication_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(1.0, 2.0, 3.0);
    let new = a * b;

    assert_eq!(new[1], 1.0);
    assert_eq!(new[2], 4.0);
    assert_eq!(new[3], 9.0);
}

#[test]
pub fn vec_division_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(1.0, 2.0, 3.0);
    let new = a / b;

    assert_eq!(new[1], 1.0);
    assert_eq!(new[2], 1.0);
    assert_eq!(new[3], 1.0);
}

#[test]
#[should_panic]
pub fn vec_division_panic_division_by_zero_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = Vec3D::new(0.0, 0.0, 0.0);
    let _new = a / b;
}

#[test]
pub fn vec_scalar_addition_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = 2.0;
    let new = a + b;

    assert_eq!(new[1], 3.0);
    assert_eq!(new[2], 4.0);
    assert_eq!(new[3], 5.0);
}

#[test]
pub fn vec_scalar_subtraction_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = 2.0;
    let new = a - b;

    assert_eq!(new[1], -1.0);
    assert_eq!(new[2], 0.0);
    assert_eq!(new[3], 1.0);
}

#[test]
pub fn vec_scalar_multiplication_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = 2.0;
    let new = a * b;

    assert_eq!(new[1], 2.0);
    assert_eq!(new[2], 4.0);
    assert_eq!(new[3], 6.0);
}

#[test]
pub fn vec_scalar_division_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = 2.0;
    let new = a / b;

    assert_eq!(new[1], 0.5);
    assert_eq!(new[2], 1.0);
    assert_eq!(new[3], 1.5);
}

#[test]
#[should_panic]
pub fn vec_scalar_division_panic_division_by_zero_test() {
    let a = Vec3D::new(1.0, 2.0, 3.0);
    let b = 0.0;
    let _new = a / b;
}
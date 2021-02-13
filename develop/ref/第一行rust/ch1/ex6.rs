fn main() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_different = (desired - result).abs();
    assert!(absolute_different<=f32::EPSILON);
    assert_eq!(f32::NAN == f32::NAN, false);
}
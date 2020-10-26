use mathlib::trig::*;
#[test]
fn test_deg_rad_arithmetic() {
    let pi_by_two = RadianAngle(0.5 * PI32);
    let ninety = DegreeAngle(90.0);
    // RHS has preference over LHS, so the resulting angle is a `RadianAngle`
    let straight_angle = ninety + pi_by_two;
    assert_eq!(PI32, straight_angle.into_rad());
}
#[test]
fn test_rad_deg_arithmetic() {
    let pi_by_two = RadianAngle(0.5 * PI32);
    let ninety = DegreeAngle(90.0);
    // RHS has preference over LHS, so the resulting angle is a `DegreeAngle`
    let straight_angle = pi_by_two + ninety;
    assert_eq!(180.0, straight_angle.into_deg());
}

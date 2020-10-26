/*
 *     Copyright (c) 2020, Sayan Nandan <ohsayan@outlook.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

//! # `mathlib`
//!
//! `mathlib` is an effort to provide a native math library for Rust developers that covers
//! a multiple domains in mathematics.
//!
//! **Note:** This library is current a work-in-progress
pub mod trig {
    //! # `trig`
    //! The trig module provides primitives and methods for trigonometric operations. Most importantly,
    //! the `trig` module provides two types: `DegreeAngle` and `RadianAngle` which can be used for most trigonometric
    //! operations
    //!
    //! ## Type coercions
    //! Always remember that RHS is preferred over LHS. Let's take a look at a couple of cases:
    //! - `DegreeAngle<T>` + `RadianAngle<T>` = `RadianAngle<T>`
    //! - `RadianAngle<T>` + `DegreeAngle<T>` = `DegreeAngle<T>`
    //!
    //!
    pub use std::f32::consts::PI as PI32;
    pub use std::f64::consts::PI as PI64;

    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, PartialEq)]
    /// An angle in degrees
    pub struct DegreeAngle<T>(pub T);
    #[derive(Debug, PartialEq)]
    /// An angle in radians
    pub struct RadianAngle<T>(pub T);
    /// All objects implementing the `IntoRad` trait can be converted into radians
    pub trait IntoRad<T> {
        fn into_rad(&self) -> T;
    }
    /// All objects implementing the `IntoDeg` trait can be converted into degrees
    pub trait IntoDeg<T> {
        fn into_deg(&self) -> T;
    }
    impl IntoRad<f32> for DegreeAngle<f32> {
        fn into_rad(&self) -> f32 {
            self.0.to_radians()
        }
    }
    impl IntoRad<f64> for DegreeAngle<f64> {
        fn into_rad(&self) -> f64 {
            self.0.to_radians()
        }
    }
    impl IntoDeg<f32> for DegreeAngle<f32> {
        fn into_deg(&self) -> f32 {
            self.0
        }
    }
    impl IntoDeg<f64> for DegreeAngle<f64> {
        fn into_deg(&self) -> f64 {
            self.0
        }
    }
    impl IntoRad<f32> for RadianAngle<f32> {
        fn into_rad(&self) -> f32 {
            self.0
        }
    }
    impl IntoRad<f64> for RadianAngle<f64> {
        fn into_rad(&self) -> f64 {
            self.0
        }
    }
    impl IntoDeg<f32> for RadianAngle<f32> {
        fn into_deg(&self) -> f32 {
            self.0.to_degrees()
        }
    }
    impl IntoDeg<f64> for RadianAngle<f64> {
        fn into_deg(&self) -> f64 {
            self.0.to_degrees()
        }
    }
    impl PartialEq<DegreeAngle<f32>> for RadianAngle<f32> {
        fn eq(&self, other: &DegreeAngle<f32>) -> bool {
            other.into_rad() == self.0
        }
    }
    impl PartialEq<DegreeAngle<f64>> for RadianAngle<f64> {
        fn eq(&self, other: &DegreeAngle<f64>) -> bool {
            other.into_rad() == self.0
        }
    }
    impl PartialEq<RadianAngle<f32>> for DegreeAngle<f32> {
        fn eq(&self, other: &RadianAngle<f32>) -> bool {
            other.into_deg() == self.0
        }
    }
    impl PartialEq<RadianAngle<f64>> for DegreeAngle<f64> {
        fn eq(&self, other: &RadianAngle<f64>) -> bool {
            other.into_deg() == self.0
        }
    }
    impl<T> Add for RadianAngle<T>
    where
        T: Into<T> + std::ops::Add<Output = T>,
    {
        type Output = RadianAngle<T>;
        fn add(self, other: RadianAngle<T>) -> Self::Output {
            RadianAngle(self.0.into() + other.0.into())
        }
    }
    impl<T> Sub for RadianAngle<T>
    where
        T: Into<T> + std::ops::Sub<Output = T>,
    {
        type Output = RadianAngle<T>;
        fn sub(self, other: RadianAngle<T>) -> Self::Output {
            RadianAngle(self.0.into() - other.0.into())
        }
    }
    impl<T> Mul for RadianAngle<T>
    where
        T: Into<T> + std::ops::Mul<Output = T>,
    {
        type Output = RadianAngle<T>;
        fn mul(self, other: RadianAngle<T>) -> Self::Output {
            RadianAngle(self.0.into() * other.0.into())
        }
    }
    impl<T> Div for RadianAngle<T>
    where
        T: Into<T> + std::ops::Div<Output = T>,
    {
        type Output = RadianAngle<T>;
        fn div(self, other: RadianAngle<T>) -> Self::Output {
            RadianAngle(self.0.into() / other.0.into())
        }
    }
    impl<T> Add for DegreeAngle<T>
    where
        T: Into<T> + std::ops::Add<Output = T>,
    {
        type Output = DegreeAngle<T>;
        fn add(self, other: DegreeAngle<T>) -> Self::Output {
            DegreeAngle(self.0.into() + other.0.into())
        }
    }
    impl<T> Sub for DegreeAngle<T>
    where
        T: Into<T> + std::ops::Sub<Output = T>,
    {
        type Output = DegreeAngle<T>;
        fn sub(self, other: DegreeAngle<T>) -> Self::Output {
            DegreeAngle(self.0.into() - other.0.into())
        }
    }
    impl<T> Mul for DegreeAngle<T>
    where
        T: Into<T> + std::ops::Mul<Output = T>,
    {
        type Output = DegreeAngle<T>;
        fn mul(self, other: DegreeAngle<T>) -> Self::Output {
            DegreeAngle(self.0.into() * other.0.into())
        }
    }
    impl<T> Div for DegreeAngle<T>
    where
        T: Into<T> + std::ops::Div<Output = T>,
    {
        type Output = DegreeAngle<T>;
        fn div(self, other: DegreeAngle<T>) -> Self::Output {
            DegreeAngle(self.0.into() / other.0.into())
        }
    }
    impl Add<RadianAngle<f32>> for DegreeAngle<f32> {
        type Output = RadianAngle<f32>;
        fn add(self, other: RadianAngle<f32>) -> Self::Output {
            RadianAngle(self.0.to_radians() + other.0)
        }
    }
    impl Add<RadianAngle<f64>> for DegreeAngle<f64> {
        type Output = RadianAngle<f64>;
        fn add(self, other: RadianAngle<f64>) -> Self::Output {
            RadianAngle(self.0.to_radians() + other.0)
        }
    }
    impl Sub<RadianAngle<f32>> for DegreeAngle<f32> {
        type Output = RadianAngle<f32>;
        fn sub(self, other: RadianAngle<f32>) -> Self::Output {
            RadianAngle(self.0.to_radians() - other.0)
        }
    }
    impl Sub<RadianAngle<f64>> for DegreeAngle<f64> {
        type Output = RadianAngle<f64>;
        fn sub(self, other: RadianAngle<f64>) -> Self::Output {
            RadianAngle(self.0.to_radians() - other.0)
        }
    }
    impl Div<RadianAngle<f32>> for DegreeAngle<f32> {
        type Output = RadianAngle<f32>;
        fn div(self, other: RadianAngle<f32>) -> Self::Output {
            RadianAngle(self.0.to_radians() / other.0)
        }
    }
    impl Div<RadianAngle<f64>> for DegreeAngle<f64> {
        type Output = RadianAngle<f64>;
        fn div(self, other: RadianAngle<f64>) -> Self::Output {
            RadianAngle(self.0.to_radians() / other.0)
        }
    }
    impl Mul<RadianAngle<f32>> for DegreeAngle<f32> {
        type Output = RadianAngle<f32>;
        fn mul(self, other: RadianAngle<f32>) -> Self::Output {
            RadianAngle(self.0.to_radians() * other.0)
        }
    }
    impl Mul<RadianAngle<f64>> for DegreeAngle<f64> {
        type Output = RadianAngle<f64>;
        fn mul(self, other: RadianAngle<f64>) -> Self::Output {
            RadianAngle(self.0.to_radians() * other.0)
        }
    }

    impl Add<DegreeAngle<f32>> for RadianAngle<f32> {
        type Output = DegreeAngle<f32>;
        fn add(self, other: DegreeAngle<f32>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() + other.0)
        }
    }
    impl Add<DegreeAngle<f64>> for RadianAngle<f64> {
        type Output = DegreeAngle<f64>;
        fn add(self, other: DegreeAngle<f64>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() + other.0)
        }
    }
    impl Sub<DegreeAngle<f32>> for RadianAngle<f32> {
        type Output = DegreeAngle<f32>;
        fn sub(self, other: DegreeAngle<f32>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() - other.0)
        }
    }
    impl Sub<DegreeAngle<f64>> for RadianAngle<f64> {
        type Output = DegreeAngle<f64>;
        fn sub(self, other: DegreeAngle<f64>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() - other.0)
        }
    }
    impl Div<DegreeAngle<f32>> for RadianAngle<f32> {
        type Output = DegreeAngle<f32>;
        fn div(self, other: DegreeAngle<f32>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() / other.0)
        }
    }
    impl Mul<DegreeAngle<f32>> for RadianAngle<f32> {
        type Output = DegreeAngle<f32>;
        fn mul(self, other: DegreeAngle<f32>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() * other.0)
        }
    }
    impl Mul<DegreeAngle<f64>> for RadianAngle<f64> {
        type Output = DegreeAngle<f64>;
        fn mul(self, other: DegreeAngle<f64>) -> Self::Output {
            DegreeAngle(self.0.to_degrees() * other.0)
        }
    }
    #[test]
    fn test_angles() {
        let pi_by_two = RadianAngle(0.5 * PI32);
        let pi = RadianAngle(PI32);
        let ninety = DegreeAngle(90.0);
        assert_eq!(pi_by_two, ninety);
        let three_pi_by_two = pi_by_two + pi;
        let two_seventy = DegreeAngle(270.0);
        assert_eq!(three_pi_by_two, two_seventy);
    }
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
}

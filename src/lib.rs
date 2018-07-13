// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "strict", deny(warnings))]
#![deny(missing_docs)]

//! This crates provides a single structure `Vector3d`, which is a
//! generic three-dimensional vector type, which should work well with
//! `dimensioned`.

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clapme;
use clapme::ClapMe;

/// A 3D vector.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, ClapMe)]
pub struct Vector3d<T> {
    /// The x component of the vector.
    pub x: T,
    /// The y component of the vector.
    pub y: T,
    /// The z component of the vector.
    pub z: T,
}

impl<T> Vector3d<T> {
    /// Create a new `Vector3d`.
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d { x: x, y: y, z: z }
    }
    /// The dot product of two vectors.  Note that we assume that the
    /// vector components have commutative multiplication.
    pub fn dot<U: Mul<T, Output=X>, X: Add<Output=X>>(self, rhs: Vector3d<U>) -> X {
        rhs.x*self.x + rhs.y*self.y + rhs.z*self.z
    }
}

// impl Vector3d<f64> {
//     pub fn ran(scale: f64) -> Vector3d<f64> {
//         unsafe {
//             let mut x = 2.0 * RAN.ran() - 1.0;
//             let mut y = 2.0 * RAN.ran() - 1.0;
//             let mut r2 = x * x + y * y;
//             while r2 >= 1.0 || r2 == 0.0 {
//                 x = 2.0 * RAN.ran() - 1.0;
//                 y = 2.0 * RAN.ran() - 1.0;
//                 r2 = x * x + y * y;
//             }
//             let mut fac = scale * (-2.0 * r2.ln() / r2).sqrt();
//             let mut out = Vector3d {
//                 x: x * fac,
//                 y: y * fac,
//                 z: 0.0,
//             };

//             x = 2.0 * RAN.ran() - 1.0;
//             y = 2.0 * RAN.ran() - 1.0;
//             r2 = x * x + y * y;
//             while r2 >= 1.0 || r2 == 0.0 {
//                 x = 2.0 * RAN.ran() - 1.0;
//                 y = 2.0 * RAN.ran() - 1.0;
//                 r2 = x * x + y * y;
//             }
//             fac = scale * (-2.0 * r2.ln() / r2).sqrt();
//             out[2] = x * fac;
//             out
//         }
//     }
// }

/// These three operators (`Add`, `Sub`, and `Neg`) do not change
/// units, and so we can implement them expecting type `T` to not
/// change. We could be more generic, and implement them similarly to
/// how we will do `Mul`, but that is added complication with no known
/// practical gain.

use std::ops::Add;
impl<T: Add<T, Output = T>> Add<Vector3d<T>> for Vector3d<T> {
    type Output = Vector3d<T>;
    fn add(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

use std::ops::Sub;
impl<T: Sub<T, Output = T>> Sub<Vector3d<T>> for Vector3d<T> {
    type Output = Vector3d<T>;
    fn sub(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

use std::ops::Neg;
impl<T: Neg<Output = T>> Neg for Vector3d<T> {
    type Output = Vector3d<T>;
    fn neg(self) -> Self::Output {
        Vector3d::new(-self.x, -self.y, -self.z)
    }
}

use std::ops::Mul;
impl<S: Clone, X, T: Mul<S, Output=X>> Mul<S> for Vector3d<T> {
    type Output = Vector3d<X>;
    fn mul(self, rhs: S) -> Self::Output {
        Vector3d::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

use std::ops::Div;
impl<S: Clone, X, T: Div<S, Output=X>> Div<S> for Vector3d<T> {
    type Output = Vector3d<X>;
    fn div(self, rhs: S) -> Self::Output {
        Vector3d::new(self.x / rhs.clone(), self.y / rhs.clone(), self.z / rhs)
    }
}

impl<T: Clone> Vector3d<T> {
    /// The cross product of two vectors.  Note that we assume that
    /// the components of both vector types have commutative
    /// multiplication.
    pub fn cross<U: Clone + Mul<T, Output=X>,
             X: Add<Output=X> + Sub<Output=X>>(self, rhs: Vector3d<U>) -> Vector3d<X> {
        Vector3d::new(rhs.z.clone()*self.y.clone() - rhs.y.clone()*self.z.clone(),
                      rhs.x.clone()*self.z.clone() - rhs.z*self.x.clone(),
                      rhs.y*self.x - rhs.x.clone()*self.y.clone())
    }
}

impl<T: Clone + Mul<T, Output=X>, X: Add<Output=X>> Vector3d<T> {
    /// The square of the vector.
    pub fn norm2(self) -> X {
        self.clone().dot(self)
    }
}

use std::ops::Index;
impl<T> Index<usize> for Vector3d<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

use std::ops::IndexMut;
impl<T> IndexMut<usize> for Vector3d<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

use std::fmt;
impl<T> fmt::Display for Vector3d<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3d<T> {
    /// Create a new `Vector3d`.
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d { x: x, y: y, z: z }
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
impl<T> Add<Vector3d<T>> for Vector3d<T>
    where T: Add<T, Output = T>
{
    type Output = Vector3d<T>;
    fn add(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

use std::ops::Sub;
impl<T> Sub<Vector3d<T>> for Vector3d<T>
    where T: Sub<T, Output = T>
{
    type Output = Vector3d<T>;
    fn sub(self, rhs: Vector3d<T>) -> Self::Output {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

use std::ops::Neg;
impl<T> Neg for Vector3d<T>
    where T: Neg<Output = T>
{
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

/// The dot product is the first of our custom operations. We create a
/// trait with an associated type.

pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// And then we implement it. Again, we are assuming that our vectors
/// are over some type that does not change with addition. If we
/// weren't making that assumption, this would get a good deal
/// messier.

impl<U, X, T> Dot<Vector3d<U>> for Vector3d<T>
    where X: Add<Output=X>,
          T: Mul<U, Output=X>
{
    type Output = X;
    fn dot(self, rhs: Vector3d<U>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

/// The cross product follows a similar pattern.

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<T, U, X> Cross<Vector3d<U>> for Vector3d<T>
    where T: Mul<U, Output=X> + Clone,
          X: Add<Output=X> + Sub<Output=X>,
          U: Clone
{
    type Output = Vector3d<X>;
    fn cross(self, rhs: Vector3d<U>) -> Self::Output {
        Vector3d::new(self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
                      self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z,
                      self.x * rhs.y - self.y.clone() * rhs.x.clone())
    }
}

/// For the norm-squared, we can just call out to `Dot` that we've
/// already defined.

pub trait Norm2 {
    type Output;
    fn norm2(self) -> Self::Output;
}

impl<T, X> Norm2 for Vector3d<T>
    where T: Clone + Mul<T, Output=X>,
          X: Add<Output=X>
{
    type Output = X;
    fn norm2(self) -> Self::Output {
        self.clone().dot(self)
    }
}

/// Implementing `Norm` is a bit trickier. For this, we need to take a
/// square root. We have a couple options.
///
/// 1. We could just implement it for primitives and leave it to users
///    to make a norm for anything else they want.
///
/// 2. We could use the `Float` trait from the num crate. This is more
///    flexible, but still leaves out dimensioned.
///
/// 3. We could use the `Sqrt` trait from dimensioned. This gives us
///    support for dimensioned and primitives, but requires our vector
///    library be aware of dimensioned.
///
/// We will go with option 3.

// pub trait Norm {
//     type Output;
//     fn norm(self) -> Self::Output;
// }

// use dimensioned::Sqrt;
// impl<T> Norm for Vector3d<T>
//     where Vector3d<T>: Norm2,
//           <Vector3d<T> as Norm2>::Output: Sqrt
// {
//     type Output = <<Vector3d<T> as Norm2>::Output as Sqrt>::Output;
//     fn norm(self) -> Self::Output {
//         self.norm2().sqrt()
//     }
// }

/// Since we have a norm function and scalar division, we can produce
/// a normalized version of a vector.

// pub trait Normalized {
//     type Output;
//     fn normalized(self) -> Self::Output;
// }

// impl<T> Normalized for Vector3d<T>
//     where Vector3d<T>: Clone + Norm + Div<<Vector3d<T> as Norm>::Output>
// {
//     type Output = Quot<Self, <Self as Norm>::Output>;
//     fn normalized(self) -> Self::Output {
//         let n = self.clone().norm();
//         self / n
//     }
// }

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

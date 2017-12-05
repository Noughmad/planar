//! A collection of strongly typed planar geometry structs.
//!
//! It strongly separates "length" types (distance between points) and "position" type
//! (absolute point location). The semantics of this separation are:
//!
//! length + length = length
//! position + length = position
//! length - length = length
//! position - length = position
//! position - position = length
//!
//! All other operations between the two types are not allowed.
//! Both lengths and positions can be multiplied with a scalar factor.
//! However, they differ in behavior when multiplied by an arbitrary planar transformation.
//! Namely, position types transform according to both translation and scale,
//! while length types only scale but do not translate.
//!
//! The types generic over the scalar type `T`, and are tagged with an opaque `Unit` type.
//! Conversions between different unit types are handled with different transforms.
//! Further, one-dimensional types are tagged with a dimension type `D`, which prevents
//! from adding together widths and heights.
//!
//! This works
//!
//! ```
//! use planar::Width;
//! struct cm;
//!
//! let p1: Width<f64, cm> = Width::new(10.0);
//! let p2: Width<f64, cm> = Width::new(100.0);
//! let p3 = p1 + p2;
//!
//! assert_eq!(p3, Width::new(110.0));
//! ```
//!
//! But this fails to compile
//!
//! ```compile_fail
//! use planar::Width;
//! struct cm;
//! struct mm;
//! let p1: Width<f64, cm> = Width::new(10.0);
//! let p2: Width<f64, mm> = Width::new(100.0);
//! let p3 = p1 + p2;
//! ```

mod oned;
mod twod;

pub use oned::*;
pub use twod::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//! Algorithms as defined in the ["Media Capture and Streams"][mediacapture_streams] spec.
//!
//! [mediacapture_streams]: https://www.w3.org/TR/mediacapture-streams/

pub mod fitness_distance;
pub mod select_settings;

pub use self::fitness_distance::*;
pub use self::select_settings::*;

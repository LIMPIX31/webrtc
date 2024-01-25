//! Pure Rust implementation of the constraint logic defined in the ["Media Capture and Streams"][mediacapture_streams] spec.
//!
//! [mediacapture_streams]: https://www.w3.org/TR/mediacapture-streams/
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

pub mod algorithms;
pub mod errors;
pub mod macros;
pub mod property;

pub mod capabilities;
pub mod capability;
pub mod constraint;
pub mod constraints;
pub mod enumerations;
pub mod setting;
pub mod settings;
pub mod supported_constraints;

#[allow(unused_imports)]
pub use self::{capabilities::MediaStreamCapabilities, settings::MediaStreamSettings};
#[allow(unused_imports)]
pub use self::{
    capabilities::MediaTrackCapabilities,
    capability::MediaTrackCapability,
    constraint::{
        MediaTrackConstraint, MediaTrackConstraintResolutionStrategy, ResolvedMediaTrackConstraint,
        ResolvedValueConstraint, ResolvedValueRangeConstraint, ResolvedValueSequenceConstraint,
        SanitizedMediaTrackConstraint, ValueConstraint, ValueRangeConstraint,
        ValueSequenceConstraint,
    },
    constraints::{
        AdvancedMediaTrackConstraints, BoolOrMediaTrackConstraints, MandatoryMediaTrackConstraints,
        MediaStreamConstraints, MediaTrackConstraintSet, MediaTrackConstraints,
        ResolvedAdvancedMediaTrackConstraints, ResolvedMandatoryMediaTrackConstraints,
        ResolvedMediaTrackConstraintSet, ResolvedMediaTrackConstraints,
        SanitizedMandatoryMediaTrackConstraints, SanitizedMediaTrackConstraintSet,
        SanitizedMediaTrackConstraints,
    },
    enumerations::{FacingMode, ResizeMode},
    property::MediaTrackProperty,
    setting::MediaTrackSetting,
    settings::MediaTrackSettings,
    supported_constraints::MediaTrackSupportedConstraints,
};

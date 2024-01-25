pub mod buffer;
pub mod sample;

pub use sample::Sample;

pub mod sealed {
    pub trait Sealed {}
}

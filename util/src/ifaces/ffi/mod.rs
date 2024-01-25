#[cfg(target_family = "windows")]
pub mod windows;
#[cfg(target_family = "windows")]
pub use self::windows::ifaces;

#[cfg(target_family = "unix")]
pub mod unix;
#[cfg(target_family = "unix")]
pub use self::unix::ifaces;

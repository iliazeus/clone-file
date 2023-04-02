mod common;

pub use common::*;

#[cfg(any(doc, target_os = "linux"))]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(any(doc, not(target_os = "linux")))]
pub mod unsupported;

#[cfg(not(target_os = "linux"))]
pub use unsupported::*;

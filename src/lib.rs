//! SigmaOS-usb — USB stack model.
//!
//! Models USB devices, endpoints, descriptors, hubs, and the
//! USB host controller interface. Deterministic and std-only
//! (no actual hardware access).

pub mod usb;
pub mod usb_formatter;
pub mod tpm;

pub use usb::{UsbError, UsbDevice, UsbDriver};
pub use usb_formatter::UsbFormatter;


#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod models;

// Re-export for convenience
pub use models::*;

// XML parsing features (optional)
#[cfg(feature = "xml")]
pub use quick_xml;

#[cfg(feature = "xml")]
/// Parse XML string into Vmix structure
///
/// # Examples
///
/// ```ignore
/// use vmix_core::{Vmix, from_str};
///
/// let xml = r#"<vmix><version>1.0</version>...</vmix>"#;
/// let vmix: Vmix = from_str(xml)?;
/// ```
pub fn from_str(s: &str) -> Result<Vmix, quick_xml::DeError> {
    quick_xml::de::from_str(s)
}

#[cfg(feature = "xml")]
/// Serialize Vmix structure to XML string
///
/// # Examples
///
/// ```ignore
/// use vmix_core::{Vmix, to_string};
///
/// let vmix = Vmix { /* ... */ };
/// let xml = to_string(&vmix)?;
/// ```
pub fn to_string(vmix: &Vmix) -> Result<alloc::string::String, quick_xml::SeError> {
    quick_xml::se::to_string(vmix)
}

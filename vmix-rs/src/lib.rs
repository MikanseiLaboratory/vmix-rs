// Re-export vmix-core (always available)
pub use vmix_core;

/// Shortcut function catalog scraped from the vMix help reference.
///
/// Available with the default features and with `full`. The data is embedded
/// at compile time, so reading it does not touch the network.
pub mod shortcuts {
    pub use vmix_shortcuts::*;
}

// Re-export vmix-tcp when enabled
#[cfg(feature = "tcp")]
pub use vmix_tcp;

// Re-export vmix-http when enabled
#[cfg(feature = "http")]
pub use vmix_http;

// Convenience re-exports for common types
pub mod models {
    pub use vmix_core::*;
}

#[cfg(feature = "tcp")]
pub mod commands {
    pub use vmix_tcp::commands::*;
}

#[cfg(feature = "tcp")]
pub mod acts {
    pub use vmix_tcp::acts::*;
}

#[cfg(feature = "tcp")]
pub mod traits {
    pub use vmix_tcp::traits::*;

    #[cfg(feature = "http")]
    pub use vmix_http::traits::*;
}

#[cfg(feature = "tcp")]
pub mod vmix {
    pub use vmix_tcp::vmix::*;
}

#[cfg(feature = "http")]
pub mod http {
    pub use vmix_http::client::*;
}

// Top-level convenience re-exports for backward compatibility
#[cfg(feature = "tcp")]
pub use vmix_tcp::{VmixApi, VmixTcpApiClient};

#[cfg(feature = "http")]
pub use vmix_http::{HttpVmixClient, VmixApiClient};

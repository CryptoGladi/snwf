use std::{fmt::Debug, sync::Arc};

/// To make it easier to call [`tokio::time::timeout`] with a custom error.
macro_rules! timeout {
    ($x:expr, $error:expr, $timeout:expr) => {
        tokio::time::timeout($timeout, $x).await.map_err($error)
    };

    ($x:expr, $error:expr) => {
        tokio::time::timeout(crate::common::DEFAULT_TIMEOUT, $x)
            .await
            .map_err($error)
    };
}

pub(crate) use timeout;

/// Generate config for [`Sender`] and [`Recipient`]
macro_rules! generate_config {
    ($name:ident, $config_for:ident) => {
        #[doc = "Config for [`"]
        #[doc = stringify!($config_for)]
        #[doc = "`]\n"]
        #[doc = "# Warning!\n"]
        #[doc = "**Generate by macros**"]
        #[derive(Clone)]
        pub struct $name<'a> {
            pub addr: std::net::IpAddr,
            pub port_for_send_files: u16,
            pub port_for_handshake: u16,
            pub timeout: std::time::Duration,
            pub progress_fn: Option<crate::common::alias::ProgressFn<'a>>,
        }

        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                //write!(f, " Hi: {}", self.id)
                f.debug_struct(stringify!($name))
                    .field("addr", &self.addr)
                    .field("port_for_send_files", &self.port_for_send_files)
                    .field("port_for_handshake", &self.port_for_handshake)
                    .field("timeout", &self.timeout)
                    .field("progress_fn: is_none()", &self.progress_fn.is_none())
                    .finish()
            }
        }
    };
}

pub(crate) use generate_config;

/// Generate new implementation for [`generate_config`]
macro_rules! generate_new_for_config {
    ($name_config:ident) => {
        #[doc = "New for [`"]
        #[doc = stringify!(Self)]
        #[doc = "`]\n"]
        #[doc = "* `addr` - IP address.\n"]
        #[doc = "* `port_for_send_files` - Port for sending files. Uses this port only [`crate::protocol`].\n"]
        #[doc = "* `port_for_handshake` - Handshake port. The [`crate::protocol`] does not use it.\n"]
        #[doc = "# Warning!\n"]
        #[doc = "**Generate by macros**"]
        pub fn new(
            addr: std::net::IpAddr,
            port_for_send_files: u16,
            port_for_handshake: u16
        ) -> Self {
            Self {
                config: $name_config {
                    addr,
                    port_for_send_files,
                    port_for_handshake,
                    timeout: crate::common::DEFAULT_TIMEOUT,
                    progress_fn: None
                },
            }
        }
    };
}

pub(crate) use generate_new_for_config;
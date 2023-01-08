//! Useful macros

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

/// Generate config for [`Sender`](crate::sender::Sender) and [`Recipient`](crate::recipient::Recipient)
macro_rules! generate_config {
    ($name:ident, $config_for:ident) => {
        #[doc = "Config for [`"]
        #[doc = stringify!($config_for)]
        #[doc = "`]\n"]
        #[doc = "# Warning!\n"]
        #[doc = "**Generate by macros**"]
        #[derive(Clone)]
        pub struct $name<'a> {
            #[doc = "IP address for bind or connect"]
            pub(crate) addr: std::net::IpAddr,

            #[doc = "Port for sending files. Uses this port only [`crate::protocol`]"]
            pub(crate) port_for_send_files: u16,

            #[doc = "Handshake port. The [`crate::protocol`] does not use it"]
            pub(crate) port_for_handshake: u16,

            #[doc = "Timeout for getting error"]
            pub(crate) timeout: std::time::Duration,

            #[doc = "Callback to check the progress of the operation\n\n"]
            #[doc = "To change it, you need to call set_progress_fn"]
            pub(crate) progress_fn: Option<crate::core::ProgressFn<'a>>,
        }

        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                //write!(f, " Hi: {}", self.id)
                f.debug_struct(stringify!($name))
                    .field("addr", &self.addr)
                    .field("port_for_send_files", &self.port_for_send_files)
                    .field("port_for_handshake", &self.port_for_handshake)
                    .field("timeout", &self.timeout)
                    .field("progress_fn.is_none()", &self.progress_fn.is_none())
                    .finish()
            }
        }

        impl crate::core::CoreConfig for $name<'_> {
            #[inline]
            fn get_addr(&self) -> std::net::IpAddr {
                self.addr
            }

            #[inline]
            fn get_port_for_send_files(&self) -> u16 {
                self.port_for_send_files
            }

            #[inline]
            fn get_port_for_handshake(&self) -> u16 {
                self.port_for_handshake
            }

            #[inline]
            fn get_timeout(&self) -> std::time::Duration {
                self.timeout
            }

            #[inline]
            fn run_progress_fn(&self, progressing: Progressing) {
                if let Some(progress_fn) = self.progress_fn.clone() {
                    progress_fn.lock().unwrap()(progressing);
                }
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
        #[doc = "* `addr` - IP address for bind or connect.\n"]
        #[doc = "* `port_for_send_files` - port for sending files. Uses this port only [`crate::protocol`].\n"]
        #[doc = "* `port_for_handshake` - handshake port. The [`crate::protocol`] does not use it.\n"]
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

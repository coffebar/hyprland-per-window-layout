extern crate nix;

pub use self::inner::*;

mod inner {
    use nix::sys::socket::{self, UnixAddr};
    use nix::unistd;
    use nix::Result;
    use std::os::unix::prelude::RawFd;

    /// A struct representing one running instance.
    pub struct SingleInstance {
        maybe_sock: Option<RawFd>,
    }

    impl SingleInstance {
        /// Returns a new SingleInstance object.
        pub fn new(name: &str) -> Result<Self> {
            let addr = UnixAddr::new_abstract(name.as_bytes())?;
            let sock = socket::socket(
                socket::AddressFamily::Unix,
                socket::SockType::Stream,
                socket::SockFlag::empty(),
                None,
            )?;

            let maybe_sock = match socket::bind(sock, &socket::SockAddr::Unix(addr)) {
                Ok(()) => Some(sock),
                Err(nix::errno::Errno::EADDRINUSE) => None,
                Err(e) => return Err(e),
            };

            Ok(Self { maybe_sock })
        }

        /// Returns whether this instance is single.
        pub fn is_single(&self) -> bool {
            self.maybe_sock.is_some()
        }
    }

    impl Drop for SingleInstance {
        fn drop(&mut self) {
            if let Some(sock) = self.maybe_sock {
                // Intentionally discard any close errors.
                let _ = unistd::close(sock);
            }
        }
    }
}

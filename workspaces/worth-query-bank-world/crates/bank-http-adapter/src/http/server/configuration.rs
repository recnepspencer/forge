use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::num::NonZeroUsize;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct BankHttpServerConfiguration {
    bind_address: SocketAddr,
    queue_capacity: NonZeroUsize,
    maximum_concurrency: NonZeroUsize,
    maximum_live_streams: NonZeroUsize,
    stream_queue_capacity: NonZeroUsize,
    opaque_handle_capacity: NonZeroUsize,
    maximum_body_bytes: usize,
    maximum_deadline: Duration,
    opaque_handle_lifetime: Duration,
}

impl BankHttpServerConfiguration {
    pub fn local_ephemeral() -> Self {
        Self {
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            queue_capacity: NonZeroUsize::new(64).expect("constant is nonzero"),
            maximum_concurrency: NonZeroUsize::new(8).expect("constant is nonzero"),
            maximum_live_streams: NonZeroUsize::new(64).expect("constant is nonzero"),
            stream_queue_capacity: NonZeroUsize::new(16).expect("constant is nonzero"),
            opaque_handle_capacity: NonZeroUsize::new(1_024).expect("constant is nonzero"),
            maximum_body_bytes: 64 * 1_024,
            maximum_deadline: Duration::from_secs(30),
            opaque_handle_lifetime: Duration::from_secs(300),
        }
    }

    pub const fn bind_address(self) -> SocketAddr {
        self.bind_address
    }

    pub const fn queue_capacity(self) -> NonZeroUsize {
        self.queue_capacity
    }

    pub const fn maximum_concurrency(self) -> NonZeroUsize {
        self.maximum_concurrency
    }

    pub const fn maximum_live_streams(self) -> NonZeroUsize {
        self.maximum_live_streams
    }

    pub const fn maximum_body_bytes(self) -> usize {
        self.maximum_body_bytes
    }

    pub const fn stream_queue_capacity(self) -> NonZeroUsize {
        self.stream_queue_capacity
    }

    pub const fn opaque_handle_capacity(self) -> NonZeroUsize {
        self.opaque_handle_capacity
    }

    pub const fn maximum_deadline(self) -> Duration {
        self.maximum_deadline
    }

    pub const fn opaque_handle_lifetime(self) -> Duration {
        self.opaque_handle_lifetime
    }

    pub fn with_queue_capacity(mut self, capacity: NonZeroUsize) -> Self {
        self.queue_capacity = capacity;
        self
    }

    pub fn with_maximum_concurrency(mut self, concurrency: NonZeroUsize) -> Self {
        self.maximum_concurrency = concurrency;
        self
    }

    pub fn with_maximum_live_streams(mut self, concurrency: NonZeroUsize) -> Self {
        self.maximum_live_streams = concurrency;
        self
    }

    pub fn with_stream_queue_capacity(mut self, capacity: NonZeroUsize) -> Self {
        self.stream_queue_capacity = capacity;
        self
    }

    pub fn with_opaque_handle_capacity(mut self, capacity: NonZeroUsize) -> Self {
        self.opaque_handle_capacity = capacity;
        self
    }

    pub const fn with_opaque_handle_lifetime(mut self, lifetime: Duration) -> Self {
        self.opaque_handle_lifetime = lifetime;
        self
    }
}

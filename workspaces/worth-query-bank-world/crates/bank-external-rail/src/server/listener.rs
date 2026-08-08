//! TCP listener lifecycle for the Bank external rail.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use super::completed_effects::CompletedEffects;
use super::dispatch::handle_connection;
use super::ledger::Ledger;
use crate::protocol::support_profile::RailProtocolSupportProfile;

/// A bound external rail, ready to accept connections.
pub struct RailServer {
    listener: TcpListener,
    ledger: Arc<Ledger>,
    completed_effects: Arc<CompletedEffects>,
    protocol_support: RailProtocolSupportProfile,
}

impl RailServer {
    /// Binds the rail to `addr`. Pass a port of `0` to let the OS assign
    /// one; read it back from [`RailServer::local_addr`].
    pub async fn bind(addr: SocketAddr) -> std::io::Result<Self> {
        Self::bind_with_protocol_support(addr, RailProtocolSupportProfile::Current).await
    }

    pub async fn bind_with_protocol_support(
        addr: SocketAddr,
        protocol_support: RailProtocolSupportProfile,
    ) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self {
            listener,
            ledger: Arc::new(Ledger::new()),
            completed_effects: Arc::new(CompletedEffects::default()),
            protocol_support,
        })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    /// Accepts connections forever, handling each one on its own task.
    ///
    /// Returns only if the listener itself fails; individual connection
    /// failures are contained to that connection.
    pub async fn serve(self) -> std::io::Result<Infallible> {
        loop {
            let (stream, _peer) = self.listener.accept().await?;
            let ledger = Arc::clone(&self.ledger);
            let completed_effects = Arc::clone(&self.completed_effects);
            tokio::spawn(handle_connection(
                stream,
                ledger,
                completed_effects,
                self.protocol_support,
            ));
        }
    }
}

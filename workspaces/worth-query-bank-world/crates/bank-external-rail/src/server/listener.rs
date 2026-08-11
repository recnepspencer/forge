//! TCP listener lifecycle for the Bank external rail.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;

use super::dispatch::{handle_connection, RailDispatchState};
use crate::protocol::support_profile::RailProtocolSupportProfile;
use crate::test_control::{handle_test_control_connection, FaultSelection};

/// A bound external rail, ready to accept connections.
pub struct RailServer {
    listener: TcpListener,
    test_control_listener: TcpListener,
    dispatch_state: Arc<RailDispatchState>,
    fault_selection: Arc<FaultSelection>,
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
        let test_control_listener = TcpListener::bind(SocketAddr::new(addr.ip(), 0)).await?;
        let fault_selection = Arc::new(FaultSelection::new());
        Ok(Self {
            listener,
            test_control_listener,
            dispatch_state: Arc::new(RailDispatchState::new(
                Arc::clone(&fault_selection),
                protocol_support,
            )),
            fault_selection,
        })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub fn test_control_addr(&self) -> std::io::Result<SocketAddr> {
        self.test_control_listener.local_addr()
    }

    /// Accepts connections forever, handling each one on its own task.
    ///
    /// Returns only if the listener itself fails; individual connection
    /// failures are contained to that connection.
    pub async fn serve(self) -> std::io::Result<Infallible> {
        loop {
            tokio::select! {
                accepted = self.listener.accept() => {
                    let (stream, _peer) = accepted?;
                    tokio::spawn(handle_connection(stream, Arc::clone(&self.dispatch_state)));
                }
                accepted = self.test_control_listener.accept() => {
                    let (stream, _peer) = accepted?;
                    tokio::spawn(handle_test_control_connection(
                        stream,
                        Arc::clone(&self.fault_selection),
                    ));
                }
            }
        }
    }
}

//! Client for the physically separate rail test-control listener.

use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;

use crate::protocol::wire::{read_frame, write_frame, FrameRead};

use super::protocol::{RailTestControlRequest, RailTestControlResponse};
use super::FaultScript;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailTestControlFailure {
    Disconnected,
    TimedOut,
}

pub async fn select_fault(
    addr: SocketAddr,
    script: FaultScript,
    frame_timeout: Duration,
) -> Result<(), RailTestControlFailure> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|_| RailTestControlFailure::Disconnected)?;
    write_frame(&mut stream, &RailTestControlRequest::SelectFault(script))
        .await
        .map_err(|_| RailTestControlFailure::Disconnected)?;
    let response = tokio::time::timeout(
        frame_timeout,
        read_frame::<_, RailTestControlResponse>(&mut stream),
    )
    .await
    .map_err(|_| RailTestControlFailure::TimedOut)?
    .map_err(|_| RailTestControlFailure::Disconnected)?;
    match response {
        FrameRead::Frame(RailTestControlResponse::Selected) => Ok(()),
        FrameRead::Disconnected => Err(RailTestControlFailure::Disconnected),
    }
}

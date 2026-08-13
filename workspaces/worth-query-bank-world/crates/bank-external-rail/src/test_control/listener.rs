//! One-shot connection handling for the separate test-control listener.

use std::sync::Arc;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::protocol::wire::{read_frame, write_frame, FrameRead};

use super::protocol::{RailTestControlRequest, RailTestControlResponse};
use super::FaultSelection;

pub(crate) async fn handle_test_control_connection(
    mut stream: TcpStream,
    selection: Arc<FaultSelection>,
) {
    let request = match read_frame::<_, RailTestControlRequest>(&mut stream).await {
        Ok(FrameRead::Frame(request)) => request,
        Ok(FrameRead::Disconnected) | Err(_) => return,
    };
    match request {
        RailTestControlRequest::SelectFault(script) => selection.select(script),
    }
    let _ = write_frame(&mut stream, &RailTestControlResponse::Selected).await;
    let _ = stream.shutdown().await;
}

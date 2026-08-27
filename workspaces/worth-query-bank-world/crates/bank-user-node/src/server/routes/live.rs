use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::body::{Body, Bytes};
use axum::extract::{rejection::JsonRejection, State};
use axum::http::{header, HeaderValue};
use axum::response::Response;
use axum::routing::post;
use axum::{Json, Router};
use bank_http_adapter::BankHttpAccountActivityEvent;
use tokio::sync::{mpsc, oneshot};
use tokio_stream::{Stream, StreamExt};

use crate::protocol::BankUserNodeAccountActivityStreamRequest;

use super::{malformed, node_denial_response, saturated, UserNodeState};

mod sse_framing;

use sse_framing::SseEventFramer;

pub(super) fn router() -> Router<UserNodeState> {
    Router::new().route("/v1/live/account-activity", post(account_activity))
}

async fn account_activity(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeAccountActivityStreamRequest>, JsonRejection>,
) -> Response {
    let Ok(Json(request)) = request else {
        return node_denial_response(malformed());
    };
    let Ok(_request_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return node_denial_response(saturated());
    };
    let Ok(stream_permit) = Arc::clone(&state.live_streams).try_acquire_owned() else {
        return node_denial_response(saturated());
    };
    let request_id = request.request_id.clone();
    let stream = match state.session.open_account_activity(request).await {
        Ok(stream) => stream,
        Err(denial) => return node_denial_response(denial),
    };
    let (upstream, revocation) = stream.into_transport();
    let content_type = upstream
        .headers()
        .get(header::CONTENT_TYPE)
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_static("text/event-stream"));
    let body = proxy_stream(upstream, revocation, request_id, stream_permit);
    let mut response = Response::new(Body::from_stream(body));
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, content_type);
    response
}

fn proxy_stream(
    upstream: reqwest::Response,
    mut revocation: tokio::sync::watch::Receiver<u64>,
    request_id: String,
    stream_permit: tokio::sync::OwnedSemaphorePermit,
) -> BankUserNodeLiveStream {
    let (sender, receiver) = mpsc::channel(1);
    let (terminal, terminal_receiver) = oneshot::channel();
    tokio::spawn(async move {
        let _stream_permit = stream_permit;
        let mut bytes = upstream.bytes_stream();
        let mut framer = SseEventFramer::new();
        let mut terminal_sender = Some(terminal);
        loop {
            let chunk = tokio::select! {
                biased;
                _ = sender.closed() => break,
                changed = revocation.changed() => match changed {
                    Ok(()) => {
                        send_terminal(&mut terminal_sender, typed_event(
                            BankHttpAccountActivityEvent::Cancelled {
                                request_id: request_id.clone(),
                            },
                        ));
                        break;
                    }
                    Err(_) => break,
                },
                chunk = bytes.next() => match chunk {
                    Some(Ok(bytes)) => bytes,
                    Some(Err(error)) => {
                        send_terminal(&mut terminal_sender, terminal_stream_event(&request_id, error));
                        break;
                    }
                    None => {
                        if framer.finish().is_err() {
                            send_terminal(&mut terminal_sender, typed_event(
                                BankHttpAccountActivityEvent::Unavailable {
                                    request_id: request_id.clone(),
                                },
                            ));
                        }
                        break;
                    }
                },
            };
            let Ok(events) = framer.push(&chunk) else {
                send_terminal(
                    &mut terminal_sender,
                    typed_event(BankHttpAccountActivityEvent::Unavailable {
                        request_id: request_id.clone(),
                    }),
                );
                break;
            };
            for event in events {
                if !send_proxy_event(&sender, &mut terminal_sender, event, &request_id) {
                    return;
                }
            }
        }
    });
    BankUserNodeLiveStream {
        chunks: receiver,
        terminal: terminal_receiver,
        chunks_closed: false,
        terminal_closed: false,
    }
}

fn send_proxy_event(
    sender: &mpsc::Sender<Result<Bytes, Infallible>>,
    terminal: &mut Option<oneshot::Sender<Bytes>>,
    event: Bytes,
    request_id: &str,
) -> bool {
    let posture = match activity_event(&event) {
        Some(BankHttpAccountActivityEvent::Opened { .. }) => ProxyEventPosture::Opened,
        Some(BankHttpAccountActivityEvent::Update { .. }) => ProxyEventPosture::Update,
        Some(
            BankHttpAccountActivityEvent::Overflow { .. }
            | BankHttpAccountActivityEvent::Denied { .. }
            | BankHttpAccountActivityEvent::Cancelled { .. }
            | BankHttpAccountActivityEvent::DeadlineExceeded { .. }
            | BankHttpAccountActivityEvent::Closed { .. }
            | BankHttpAccountActivityEvent::Unavailable { .. },
        ) => {
            send_terminal(terminal, event);
            return false;
        }
        None if frame_is_keepalive(&event) => ProxyEventPosture::Keepalive,
        None => {
            send_terminal(
                terminal,
                typed_event(BankHttpAccountActivityEvent::Unavailable {
                    request_id: request_id.to_owned(),
                }),
            );
            return false;
        }
    };
    match sender.try_send(Ok(event)) {
        Ok(()) => true,
        Err(mpsc::error::TrySendError::Full(_))
            if matches!(posture, ProxyEventPosture::Keepalive) =>
        {
            true
        }
        Err(mpsc::error::TrySendError::Full(_)) => {
            let terminal_event = match posture {
                ProxyEventPosture::Update => BankHttpAccountActivityEvent::Overflow {
                    request_id: request_id.to_owned(),
                    missed_commit_batches: 1,
                },
                ProxyEventPosture::Opened => BankHttpAccountActivityEvent::Unavailable {
                    request_id: request_id.to_owned(),
                },
                ProxyEventPosture::Keepalive => unreachable!("full keepalive handled above"),
            };
            send_terminal(terminal, typed_event(terminal_event));
            false
        }
        Err(mpsc::error::TrySendError::Closed(_)) => false,
    }
}

#[derive(Clone, Copy)]
enum ProxyEventPosture {
    Opened,
    Update,
    Keepalive,
}

fn activity_event(frame: &[u8]) -> Option<BankHttpAccountActivityEvent> {
    let frame = std::str::from_utf8(frame).ok()?;
    let data = frame
        .lines()
        .find_map(|line| line.strip_prefix("data:"))?
        .trim_start();
    serde_json::from_str(data).ok()
}

fn frame_is_keepalive(frame: &[u8]) -> bool {
    let Ok(frame) = std::str::from_utf8(frame) else {
        return false;
    };
    let mut saw_comment = false;
    for line in frame.lines().filter(|line| !line.is_empty()) {
        if !line.starts_with(':') {
            return false;
        }
        saw_comment = true;
    }
    saw_comment
}

struct BankUserNodeLiveStream {
    chunks: mpsc::Receiver<Result<Bytes, Infallible>>,
    terminal: oneshot::Receiver<Bytes>,
    chunks_closed: bool,
    terminal_closed: bool,
}

impl Stream for BankUserNodeLiveStream {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.chunks_closed {
            match self.chunks.poll_recv(context) {
                Poll::Ready(Some(chunk)) => return Poll::Ready(Some(chunk)),
                Poll::Ready(None) => self.chunks_closed = true,
                Poll::Pending => return Poll::Pending,
            }
        }
        if self.terminal_closed {
            return Poll::Ready(None);
        }
        match Pin::new(&mut self.terminal).poll(context) {
            Poll::Ready(result) => {
                self.terminal_closed = true;
                Poll::Ready(result.ok().map(Ok))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

fn send_terminal(terminal: &mut Option<oneshot::Sender<Bytes>>, event: Bytes) {
    if let Some(terminal) = terminal.take() {
        let _ = terminal.send(event);
    }
}

fn terminal_stream_event(request_id: &str, error: reqwest::Error) -> Bytes {
    let event = if error.is_timeout() {
        BankHttpAccountActivityEvent::DeadlineExceeded {
            request_id: request_id.to_owned(),
        }
    } else {
        BankHttpAccountActivityEvent::Unavailable {
            request_id: request_id.to_owned(),
        }
    };
    typed_event(event)
}

fn typed_event(event: BankHttpAccountActivityEvent) -> Bytes {
    let data = serde_json::to_string(&event).expect("typed stream event must serialize");
    Bytes::from(format!("event: bank_account_activity\ndata: {data}\n\n"))
}

#[cfg(test)]
#[path = "live/exact_basis_shape_tests.rs"]
mod exact_basis_shape_tests;

#[cfg(test)]
mod tests {
    use super::exact_basis_shape_tests::update_event;
    use super::*;

    #[tokio::test]
    async fn terminal_side_channel_is_fused_after_one_explicit_event() {
        let (sender, receiver) = mpsc::channel(1);
        drop(sender);
        let (terminal, terminal_receiver) = oneshot::channel();
        terminal
            .send(Bytes::from_static(b"terminal"))
            .expect("terminal receiver should remain open");
        let mut stream = BankUserNodeLiveStream {
            chunks: receiver,
            terminal: terminal_receiver,
            chunks_closed: false,
            terminal_closed: false,
        };
        assert_eq!(stream.next().await.unwrap().unwrap(), "terminal");
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn full_node_proxy_queue_preserves_an_explicit_overflow_terminal() {
        let (sender, receiver) = mpsc::channel(1);
        let (terminal, terminal_receiver) = oneshot::channel();
        let mut terminal = Some(terminal);
        assert!(send_proxy_event(
            &sender,
            &mut terminal,
            typed_event(BankHttpAccountActivityEvent::Opened {
                request_id: "slow-node-consumer".to_owned(),
            }),
            "slow-node-consumer",
        ));
        assert!(!send_proxy_event(
            &sender,
            &mut terminal,
            update_event("slow-node-consumer"),
            "slow-node-consumer",
        ));
        drop(sender);
        let mut stream = BankUserNodeLiveStream {
            chunks: receiver,
            terminal: terminal_receiver,
            chunks_closed: false,
            terminal_closed: false,
        };
        assert!(std::str::from_utf8(&stream.next().await.unwrap().unwrap())
            .unwrap()
            .contains("opened"));
        let terminal = stream.next().await.unwrap().unwrap();
        assert!(std::str::from_utf8(&terminal)
            .expect("typed terminal should be UTF-8")
            .contains("overflow"));
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn split_keepalive_does_not_close_the_following_domain_event() {
        let mut framer = SseEventFramer::new();
        assert!(framer.push(b": keep-").unwrap().is_empty());
        let keepalive = framer.push(b"alive\n\n").unwrap().pop().unwrap();
        let (sender, mut receiver) = mpsc::channel(2);
        let (terminal, mut terminal_receiver) = oneshot::channel();
        let mut terminal = Some(terminal);
        assert!(send_proxy_event(
            &sender,
            &mut terminal,
            keepalive,
            "quiet-stream",
        ));
        assert!(send_proxy_event(
            &sender,
            &mut terminal,
            typed_event(BankHttpAccountActivityEvent::Opened {
                request_id: "quiet-stream".to_owned(),
            }),
            "quiet-stream",
        ));
        assert!(
            std::str::from_utf8(&receiver.recv().await.unwrap().unwrap())
                .unwrap()
                .starts_with(':')
        );
        assert!(
            std::str::from_utf8(&receiver.recv().await.unwrap().unwrap())
                .unwrap()
                .contains("opened")
        );
        assert!(matches!(
            terminal_receiver.try_recv(),
            Err(oneshot::error::TryRecvError::Empty)
        ));
    }
}

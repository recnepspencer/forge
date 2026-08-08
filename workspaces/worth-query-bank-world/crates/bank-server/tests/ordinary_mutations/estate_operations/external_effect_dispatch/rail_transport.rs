//! The Bank's outbound transport over a real, separate-process external rail.
//!
//! This is a genuine network adapter, not a stand-in: every call opens a TCP
//! connection to a rail process that shares no memory, runtime, or truth with
//! the bank under test. The only thing the test controls is which fault the
//! rail is instructed to exhibit and how long the bank waits for a frame.

use std::net::SocketAddr;
use std::sync::Mutex;
use std::time::Duration;

use bank_external_rail::{
    dispatch, inquire_admission_count, inquire_notice, inquire_status, EstateDeathNotice,
    FaultScript, LedgerStatus, RailCorrelation, RailDispatch, RailEffectPayload,
    RailExchangeOutcome, RailProcessHandle, RailProtocolSupportProfile, RailRejection,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryExternalDispatchRequest, WorthQueryExternalEffectTransport,
    WorthQueryExternalTransportOutcome,
};

/// Starts a rail in its own OS process, listening on an OS-assigned port.
///
/// The binary lives beside this test binary in the shared workspace target
/// directory; `CARGO_BIN_EXE_*` is only published to the crate that owns the
/// binary, and the rail deliberately belongs to another crate.
pub fn spawn_rail() -> RailProcessHandle {
    spawn_rail_with_protocol_support(RailProtocolSupportProfile::Current)
}

pub fn spawn_rail_with_protocol_support(
    protocol_support: RailProtocolSupportProfile,
) -> RailProcessHandle {
    RailProcessHandle::spawn_with_protocol_support(
        rail_binary_path(),
        "127.0.0.1:0",
        protocol_support,
    )
    .expect("the external rail process should start and report its address")
}

fn rail_binary_path() -> std::path::PathBuf {
    let test_binary =
        std::env::current_exe().expect("a running test binary has a discoverable path");
    let mut directory = test_binary.parent();
    let file_name = format!("bank-external-rail{}", std::env::consts::EXE_SUFFIX);
    while let Some(candidate) = directory {
        let path = candidate.join(&file_name);
        if path.is_file() {
            return path;
        }
        directory = candidate.parent();
    }
    panic!(
        "{file_name} was not found above {}; build it with \
         `cargo build -p bank-external-rail`",
        test_binary.display()
    )
}

pub struct BankEstateRailTransport {
    address: SocketAddr,
    blocking: tokio::runtime::Runtime,
    control: Mutex<RailControl>,
    attempts: Mutex<Vec<RailCorrelation>>,
}

struct RailControl {
    script: FaultScript,
    frame_timeout: Duration,
}

impl BankEstateRailTransport {
    pub fn connected_to(address: SocketAddr) -> Self {
        Self {
            address,
            blocking: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("the bank rail adapter should build its blocking runtime"),
            control: Mutex::new(RailControl {
                script: FaultScript::Succeed,
                frame_timeout: Duration::from_secs(5),
            }),
            attempts: Mutex::new(Vec::new()),
        }
    }

    /// Instructs the rail how to behave on the next dispatch, and how long the
    /// bank will wait for each frame before calling the attempt timed out.
    pub fn under(&self, script: FaultScript, frame_timeout: Duration) {
        let mut control = self.lock_control();
        control.script = script;
        control.frame_timeout = frame_timeout;
    }

    /// Correlations that actually reached the rail, in dispatch order.
    pub fn attempts(&self) -> Vec<RailCorrelation> {
        self.attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Asks the rail's own ledger what became of a correlation.
    ///
    /// This is the late reconciliation read: it never dispatches again, so a
    /// completion it discovers cannot have been caused by asking.
    pub fn ledger_status(&self, correlation: &RailCorrelation) -> LedgerStatus {
        self.blocking
            .block_on(inquire_status(
                self.address,
                correlation.clone(),
                Duration::from_secs(5),
            ))
            .expect("the rail should answer a status inquiry")
    }

    /// Asks the rail which death notice it decoded for a correlation.
    ///
    /// This is the rail's own domain reading, produced by its own decoder in
    /// its own process. A correlation-only protocol could not answer it.
    pub fn ledger_notice(&self, correlation: &RailCorrelation) -> Option<EstateDeathNotice> {
        self.blocking
            .block_on(inquire_notice(
                self.address,
                correlation.clone(),
                Duration::from_secs(5),
            ))
            .expect("the rail should answer a notice inquiry")
    }

    /// Rail-side attempt count: distinct correlations ever admitted to the ledger.
    pub fn admission_count(&self) -> u64 {
        self.blocking
            .block_on(inquire_admission_count(
                self.address,
                Duration::from_secs(5),
            ))
            .expect("the rail should answer an admission-count inquiry")
    }

    fn lock_control(&self) -> std::sync::MutexGuard<'_, RailControl> {
        self.control
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl WorthQueryExternalEffectTransport for BankEstateRailTransport {
    /// Projects one co-committed effect onto the wire.
    ///
    /// Every field of the payload comes off the request, which Query built
    /// from the durable outbox record. The adapter re-derives nothing and
    /// supplies nothing of its own: if it invented the effect name, wire
    /// identity, or bytes, the rail would decode the adapter's opinion rather than
    /// what actually committed (Q8.25-C3).
    fn dispatch(
        &self,
        request: WorthQueryExternalDispatchRequest<'_>,
    ) -> WorthQueryExternalTransportOutcome {
        let correlation = RailCorrelation::new(
            request.correlation_family(),
            request.correlation_token().to_vec(),
        );
        let payload = RailEffectPayload::new(
            request.effect(),
            request.protocol_identity().clone(),
            request.protocol_version(),
            request.maximum_payload_bytes(),
            request.payload(),
        );
        let (script, frame_timeout) = {
            let control = self.lock_control();
            (control.script, control.frame_timeout)
        };
        self.attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(correlation.clone());
        let observed = self.blocking.block_on(dispatch(
            self.address,
            RailDispatch {
                correlation: correlation.clone(),
                payload,
                fault_script: script,
            },
            frame_timeout,
        ));
        self.classify(observed, &correlation)
    }
}

impl BankEstateRailTransport {
    /// Turns one rail exchange into the outcome Query classifies.
    ///
    /// A bare disconnect is ambiguous on the wire, so the adapter asks the
    /// rail's ledger which of the two honest stories it was: a request that
    /// never landed, or a landed request whose answer was lost.
    fn classify(
        &self,
        observed: RailExchangeOutcome,
        correlation: &RailCorrelation,
    ) -> WorthQueryExternalTransportOutcome {
        match observed {
            RailExchangeOutcome::Completed => WorthQueryExternalTransportOutcome::Completed,
            RailExchangeOutcome::Acknowledged => WorthQueryExternalTransportOutcome::Acknowledged,
            RailExchangeOutcome::Rejected(RailRejection::UnsupportedProtocolVersion(
                unsupported,
            )) => WorthQueryExternalTransportOutcome::UnsupportedProtocolVersion(unsupported),
            RailExchangeOutcome::Rejected(_) => WorthQueryExternalTransportOutcome::Rejected,
            RailExchangeOutcome::DuplicateAcknowledgement => {
                WorthQueryExternalTransportOutcome::DuplicateAcknowledgement
            }
            RailExchangeOutcome::TimedOut => WorthQueryExternalTransportOutcome::TimedOut,
            RailExchangeOutcome::Disconnected => match self.ledger_status(correlation) {
                LedgerStatus::NoRecord => WorthQueryExternalTransportOutcome::Disconnected,
                LedgerStatus::Acknowledged | LedgerStatus::Completed => {
                    WorthQueryExternalTransportOutcome::LostResponse
                }
            },
        }
    }
}

//! Per-connection request handling: one connection carries exactly one
//! request, then the rail closes it.

use std::sync::Arc;
use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::protocol::fault_script::FaultScript;
use crate::protocol::notice::decode_notice_for_profile;
use crate::protocol::request::{RailDispatch, RailRequest};
use crate::protocol::response::{LedgerStatus, RailResponseFrame};
use crate::protocol::support_profile::RailProtocolSupportProfile;
use crate::protocol::wire::{read_frame, write_frame, FrameRead};

use super::completed_effects::CompletedEffects;
use super::fault_behavior;
use super::ledger::Ledger;
use super::ledger::RailAdmission;

#[derive(Clone, Copy)]
struct DispatchOwners<'a> {
    ledger: &'a Ledger,
    completed_effects: &'a CompletedEffects,
}

impl<'a> DispatchOwners<'a> {
    const fn completion(self) -> fault_behavior::CompletionOwners<'a> {
        fault_behavior::CompletionOwners::new(self.ledger, self.completed_effects)
    }
}

/// Handles one accepted connection to completion, then closes it.
///
/// A malformed or absent request is itself a legitimate boundary event, not
/// a server bug: a real network peer can always fail to send anything
/// coherent, and the rail must not panic in response.
pub async fn handle_connection(
    mut stream: TcpStream,
    ledger: Arc<Ledger>,
    completed_effects: Arc<CompletedEffects>,
    protocol_support: RailProtocolSupportProfile,
) {
    let request = match read_frame::<_, RailRequest>(&mut stream).await {
        Ok(FrameRead::Frame(request)) => request,
        Ok(FrameRead::Disconnected) | Err(_) => return,
    };

    let _ = match request {
        RailRequest::Dispatch(dispatch) => {
            serve_dispatch(
                &mut stream,
                &dispatch,
                DispatchOwners {
                    ledger: &ledger,
                    completed_effects: &completed_effects,
                },
                protocol_support,
            )
            .await
        }
        RailRequest::InquireStatus { correlation } => {
            fault_behavior::report_status(&mut stream, &correlation, &ledger).await
        }
        RailRequest::InquireNotice { correlation } => {
            fault_behavior::report_notice(&mut stream, &correlation, &ledger).await
        }
        RailRequest::InquireAdmissionCount => {
            write_frame(
                &mut stream,
                &RailResponseFrame::AdmissionCount(ledger.admission_count()),
            )
            .await
        }
        RailRequest::InquireCompletedEffectCount => {
            fault_behavior::report_completed_effect_count(&mut stream, &completed_effects).await
        }
        RailRequest::InquireCompletedNotice { correlation } => {
            fault_behavior::report_completed_notice(&mut stream, &correlation, &completed_effects)
                .await
        }
    };

    let _ = stream.shutdown().await;
}

/// Decodes the payload first, then runs the fault script it earned.
///
/// Decoding is the gate: a payload the rail cannot read is refused before any
/// fault script runs and before any ledger admission, so no script can produce
/// a record whose meaning the rail never established.
async fn serve_dispatch(
    stream: &mut TcpStream,
    dispatch: &RailDispatch,
    owners: DispatchOwners<'_>,
    protocol_support: RailProtocolSupportProfile,
) -> std::io::Result<()> {
    let correlation = &dispatch.correlation;
    let notice = match decode_notice_for_profile(&dispatch.payload, protocol_support) {
        Ok(notice) => notice,
        Err(rejection) => return fault_behavior::reject(stream, rejection).await,
    };
    let reserve_new = dispatch.fault_script != FaultScript::DisappearMidDispatch;
    match owners
        .ledger
        .admit(correlation, &dispatch.payload, notice, reserve_new)
    {
        RailAdmission::Reserved(reservation) => {
            apply_fault_script(stream, reservation, dispatch.fault_script, owners).await
        }
        RailAdmission::Replay(status) => replay_status(stream, status).await,
        RailAdmission::MeaningDrift => {
            fault_behavior::reject(
                stream,
                crate::protocol::notice::RailRejection::CorrelationPayloadMismatch,
            )
            .await
        }
        RailAdmission::DisappearedBeforeAdmission => Ok(()),
    }
}

async fn apply_fault_script(
    stream: &mut TcpStream,
    reservation: super::ledger::RailReservation,
    fault_script: FaultScript,
    owners: DispatchOwners<'_>,
) -> std::io::Result<()> {
    match fault_script {
        FaultScript::Succeed => {
            fault_behavior::succeed(stream, reservation, owners.completion()).await
        }
        FaultScript::CommitThenLoseResponse => {
            fault_behavior::commit_then_lose_response(reservation, owners.completion()).await;
            Ok(())
        }
        FaultScript::AcknowledgeWithoutCompleting => {
            fault_behavior::acknowledge_without_completing(stream, reservation).await
        }
        FaultScript::CompleteAfterDelay { delay_millis } => {
            fault_behavior::complete_after_delay(
                stream,
                reservation,
                owners.completion(),
                Duration::from_millis(delay_millis),
            )
            .await
        }
        FaultScript::DuplicateAcknowledgement => {
            fault_behavior::duplicate_acknowledgement(stream, reservation).await
        }
        FaultScript::DisappearMidDispatch => {
            unreachable!("the disappear fault never receives an admitted reservation")
        }
    }
}

async fn replay_status(stream: &mut TcpStream, status: LedgerStatus) -> std::io::Result<()> {
    write_frame(stream, &RailResponseFrame::Ack).await?;
    if status == LedgerStatus::Completed {
        write_frame(stream, &RailResponseFrame::Completed).await?;
    }
    Ok(())
}

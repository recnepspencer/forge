use worth_ui::facade::app::{WorthUiNativeApplicationShell, WorthUiNativeProjectionRebindDenial};
use worth_ui::facade::query_binding::UiProjectionObservation;
use worth_ui::facade::rebind::{UiProjectionRebindRequest, UiRebindOutcome, UiRebindReceipt};

#[derive(Debug)]
pub(crate) enum PlatformPulseProjectionRebindDenial {
    Native(WorthUiNativeProjectionRebindDenial),
    Nonpublication(String),
}

impl std::fmt::Display for PlatformPulseProjectionRebindDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(denial) => write!(formatter, "{denial:?}"),
            Self::Nonpublication(detail) => {
                write!(formatter, "projection did not publish: {detail}")
            }
        }
    }
}

pub(crate) fn publish_projection(
    shell: &mut WorthUiNativeApplicationShell,
    observation: UiProjectionObservation,
    tick: u64,
) -> Result<UiRebindReceipt, PlatformPulseProjectionRebindDenial> {
    let outcome = shell
        .begin_projection_rebind(UiProjectionRebindRequest::new(observation).observed_at_tick(tick))
        .map_err(PlatformPulseProjectionRebindDenial::Native)?;
    match outcome {
        UiRebindOutcome::Published(receipt) => Ok(receipt),
        UiRebindOutcome::InFlight(completion) => match completion.complete(tick) {
            UiRebindOutcome::Published(receipt) => Ok(receipt),
            outcome => Err(nonpublication(outcome)),
        },
        outcome => Err(nonpublication(outcome)),
    }
}

fn nonpublication(outcome: UiRebindOutcome<'_>) -> PlatformPulseProjectionRebindDenial {
    let detail = match outcome {
        UiRebindOutcome::Duplicate(_) => "duplicate".to_owned(),
        UiRebindOutcome::ObservedNoChange(_) => "observed-no-change".to_owned(),
        UiRebindOutcome::RejectedBeforeEffects(denial) => {
            let host_rejections = denial
                .host_rejections()
                .iter()
                .map(|rejection| format!("{:?}", (*rejection).denial()))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "rejected-before-effects:{:?}:{:?}:[{host_rejections}]",
                denial.stopped_phase(),
                denial.cause()
            )
        }
        UiRebindOutcome::CancelledBeforeEffects(receipt) => {
            format!("cancelled-before-effects:{:?}", receipt.stopped_phase())
        }
        UiRebindOutcome::TimedOutBeforeEffects(receipt) => {
            format!("timed-out-before-effects:{:?}", receipt.stopped_phase())
        }
        UiRebindOutcome::SupersededBeforeEffects(receipt) => {
            format!("superseded-before-effects:{:?}", receipt.stopped_phase())
        }
        UiRebindOutcome::Indeterminate(_) => "indeterminate".to_owned(),
        UiRebindOutcome::InternalDefect(defect) => {
            format!("internal-defect:{:?}", defect.kind())
        }
        UiRebindOutcome::Published(_) | UiRebindOutcome::InFlight(_) => {
            unreachable!("publication and completion outcomes are normalized before denial")
        }
    };
    PlatformPulseProjectionRebindDenial::Nonpublication(detail)
}

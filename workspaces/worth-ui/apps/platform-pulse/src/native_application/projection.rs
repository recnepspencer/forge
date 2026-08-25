use worth_ui::facade::app::{
    WorthUiNativeApplicationShell, WorthUiNativeManagedProjectionRebindOutcome,
    WorthUiNativeManagedRebindStop, WorthUiNativeProjectionRebindDenial,
};
use worth_ui::facade::query_binding::UiProjectionObservation;
use worth_ui::facade::rebind::{UiProjectionRebindRequest, UiRebindReceipt};

#[derive(Debug)]
pub(crate) enum PlatformPulseProjectionRebindDenial {
    NativeProjection(WorthUiNativeProjectionRebindDenial),
    Nonpublication(WorthUiNativeManagedRebindStop),
    ReceiptContract(&'static str),
}

pub(crate) enum PlatformPulseProjectionPublication {
    Published(UiRebindReceipt),
    Pending,
}

impl std::fmt::Display for PlatformPulseProjectionRebindDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NativeProjection(denial) => write!(formatter, "{denial:?}"),
            Self::Nonpublication(stop) => {
                write!(formatter, "projection did not publish: {stop:?}")
            }
            Self::ReceiptContract(detail) => write!(formatter, "projection receipt: {detail}"),
        }
    }
}

pub(crate) fn begin_projection(
    shell: &mut WorthUiNativeApplicationShell,
    observation: UiProjectionObservation,
    tick: u64,
) -> Result<PlatformPulseProjectionPublication, PlatformPulseProjectionRebindDenial> {
    let outcome = shell
        .begin_managed_projection_rebind(
            UiProjectionRebindRequest::new(observation).observed_at_tick(tick),
        )
        .map_err(PlatformPulseProjectionRebindDenial::NativeProjection)?;
    match outcome {
        WorthUiNativeManagedProjectionRebindOutcome::Published(receipt) => {
            Ok(PlatformPulseProjectionPublication::Published(receipt))
        }
        WorthUiNativeManagedProjectionRebindOutcome::Pending => {
            Ok(PlatformPulseProjectionPublication::Pending)
        }
        WorthUiNativeManagedProjectionRebindOutcome::Stopped(stop) => {
            Err(PlatformPulseProjectionRebindDenial::Nonpublication(stop))
        }
    }
}

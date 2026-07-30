use super::{
    fields, number, BoundedCancellationCaseObservation, BoundedCancellationDispatch,
    BoundedCancellationObligation, BoundedCancellationObservation, BoundedCancellationRecovery,
    BoundedCancellationSeam, BoundedCancellationSignal, BoundedCancellationTerminal,
};

pub(super) fn parse(lines: &[String]) -> Result<BoundedCancellationObservation, String> {
    let mut pre_dispatch = None;
    let mut post_dispatch = None;
    for line in lines
        .iter()
        .filter(|line| line.starts_with("BOUNDED_RESIDENCY_CANCELLATION "))
    {
        let case = parse_case(line)?;
        let destination = match case.seam {
            BoundedCancellationSeam::PreDispatch => &mut pre_dispatch,
            BoundedCancellationSeam::PostDispatch => &mut post_dispatch,
        };
        if destination.replace(case).is_some() {
            return Err(format!(
                "duplicate bounded-residency {} cancellation seam",
                seam(case.seam)
            ));
        }
    }
    Ok(BoundedCancellationObservation {
        pre_dispatch: pre_dispatch
            .ok_or_else(|| "missing bounded-residency pre-dispatch cancellation seam".to_owned())?,
        post_dispatch: post_dispatch.ok_or_else(|| {
            "missing bounded-residency post-dispatch cancellation seam".to_owned()
        })?,
    })
}

fn parse_case(line: &str) -> Result<BoundedCancellationCaseObservation, String> {
    let lines = [line.to_owned()];
    let value = fields(&lines, "BOUNDED_RESIDENCY_CANCELLATION ", 15)?;
    let seam = parse_seam(value[1])?;
    let backend_receipt = number(value[14], "cancellation backend receipt")?;
    Ok(BoundedCancellationCaseObservation {
        seam,
        store: store(value[2])?,
        runtime: number(value[3], "cancellation runtime")?,
        generation: number(value[4], "cancellation generation")?,
        operation: number(value[5], "cancellation operation")?,
        obligation: obligation(value[6])?,
        signal: signal(value[7])?,
        dispatch: dispatch(value[8])?,
        recovery: recovery(value[9])?,
        terminal: terminal(value[10])?,
        media_before_cancellation: number(value[11], "media before cancellation")?,
        cancellation_media_effects: number(value[12], "cancellation media effects")?,
        terminal_media_effects: number(value[13], "terminal cancellation media effects")?,
        backend_receipt: (backend_receipt != 0).then_some(backend_receipt),
    })
}

fn parse_seam(value: &str) -> Result<BoundedCancellationSeam, String> {
    match value {
        "pre-dispatch" => Ok(BoundedCancellationSeam::PreDispatch),
        "post-dispatch" => Ok(BoundedCancellationSeam::PostDispatch),
        _ => Err(format!(
            "unknown bounded-residency cancellation seam {value}"
        )),
    }
}

fn obligation(value: &str) -> Result<BoundedCancellationObligation, String> {
    match value {
        "not-dispatched" => Ok(BoundedCancellationObligation::NotDispatched),
        "settlement-continues" => Ok(BoundedCancellationObligation::SettlementContinues),
        _ => Err(format!("unknown cancellation obligation {value}")),
    }
}

fn signal(value: &str) -> Result<BoundedCancellationSignal, String> {
    match value {
        "request-cancelled" => Ok(BoundedCancellationSignal::RequestCancelled),
        "reconciled-from-physical-truth" => {
            Ok(BoundedCancellationSignal::ReconciledFromPhysicalTruth)
        }
        _ => Err(format!("unknown cancellation Signal outcome {value}")),
    }
}

fn dispatch(value: &str) -> Result<BoundedCancellationDispatch, String> {
    match value {
        "denied-consumer-cancelled" => Ok(BoundedCancellationDispatch::DeniedConsumerCancelled),
        "write-completed" => Ok(BoundedCancellationDispatch::WriteCompleted),
        _ => Err(format!("unknown cancellation dispatch outcome {value}")),
    }
}

fn recovery(value: &str) -> Result<BoundedCancellationRecovery, String> {
    match value {
        "no-settlement" => Ok(BoundedCancellationRecovery::NoSettlement),
        "continue-settlement" => Ok(BoundedCancellationRecovery::ContinueSettlement),
        _ => Err(format!("unknown cancellation recovery outcome {value}")),
    }
}

fn terminal(value: &str) -> Result<BoundedCancellationTerminal, String> {
    match value {
        "cancelled-before-dispatch" => Ok(BoundedCancellationTerminal::CancelledBeforeDispatch),
        "continued-after-consumer-cancellation" => {
            Ok(BoundedCancellationTerminal::ContinuedAfterConsumerCancellation)
        }
        _ => Err(format!("unknown cancellation terminal fate {value}")),
    }
}

fn seam(value: BoundedCancellationSeam) -> &'static str {
    match value {
        BoundedCancellationSeam::PreDispatch => "pre-dispatch",
        BoundedCancellationSeam::PostDispatch => "post-dispatch",
    }
}

fn store(encoded: &str) -> Result<[u8; 16], String> {
    if encoded.len() != 32 {
        return Err("cancellation Store identity must contain 32 hex characters".to_owned());
    }
    let mut bytes = [0_u8; 16];
    for (index, destination) in bytes.iter_mut().enumerate() {
        *destination = u8::from_str_radix(&encoded[index * 2..index * 2 + 2], 16)
            .map_err(|_| "cancellation Store identity is not hexadecimal".to_owned())?;
    }
    Ok(bytes)
}

#[cfg(test)]
#[path = "cancellation/tests.rs"]
mod tests;

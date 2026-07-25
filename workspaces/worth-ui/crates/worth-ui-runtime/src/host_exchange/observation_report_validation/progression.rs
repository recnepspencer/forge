use worth_ui_host_contract::{UiHostObservationSequence, UiHostObservationSequenceRange};

use super::UiHostObservationReportDenial;

pub(super) fn validate_sequence_progression(
    previous: Option<UiHostObservationSequence>,
    incoming: UiHostObservationSequenceRange,
) -> Result<(), UiHostObservationReportDenial> {
    let expected = match previous {
        Some(sequence) => sequence
            .value()
            .checked_add(1)
            .ok_or(UiHostObservationReportDenial::SequenceExhausted)?,
        None => 1,
    };
    if incoming.first().value() < expected {
        return Err(UiHostObservationReportDenial::SequenceReordered);
    }
    if incoming.first().value() > expected {
        return Err(UiHostObservationReportDenial::SequenceGap);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_partition_cannot_progress_beyond_the_sequence_domain() {
        assert_eq!(
            validate_sequence_progression(
                Some(UiHostObservationSequence::new(u64::MAX)),
                UiHostObservationSequenceRange::new(
                    UiHostObservationSequence::new(u64::MAX),
                    UiHostObservationSequence::new(u64::MAX),
                ),
            ),
            Err(UiHostObservationReportDenial::SequenceExhausted)
        );
    }
}

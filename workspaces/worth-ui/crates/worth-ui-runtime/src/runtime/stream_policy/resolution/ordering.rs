use super::*;

#[rustfmt::skip]
impl UiAllocationSourceOrderLedger {
    pub(super) fn evaluate_and_stage(
        &mut self,
        ingress: &crate::runtime::UiAdmittedAllocationStreamIngress,
        family: UiAllocationStreamFamily,
    ) -> Result<UiAllocationSourceOrderVerdict, UiAllocationFrameResolutionDenial> {
        let lease = ingress.source_lease();
        let record = &mut self.records[usize::from(lease.slot())];
        let observed = ingress.source_order().as_u64();
        let previous = record.and_then(|(lease_generation, source_generation, order)| {
            (lease_generation == lease.generation()
                && source_generation == ingress.source_generation()).then_some(order)
        });
        let verdict = match previous {
            None => UiAllocationSourceOrderVerdict::FirstObserved,
            Some(previous) if observed == previous => {
                return Err(UiAllocationFrameResolutionDenial::SourceSequenceDuplicate { order: observed });
            }
            Some(previous) if observed < previous => {
                return Err(UiAllocationFrameResolutionDenial::SourceSequenceRegression { previous, observed });
            }
            Some(previous) if observed == previous.saturating_add(1) => UiAllocationSourceOrderVerdict::Contiguous,
            Some(previous) if gap_is_illegal(family) => {
                return Err(UiAllocationFrameResolutionDenial::SourceSequenceGap { previous, observed });
            }
            Some(previous) => UiAllocationSourceOrderVerdict::GapAccepted { missing: observed - previous - 1 },
        };
        *record = Some((lease.generation(), ingress.source_generation(), observed));
        Ok(verdict)
    }
}

impl Default for UiAllocationSourceOrderLedger {
    fn default() -> Self {
        Self {
            records: std::array::from_fn(|_| None),
        }
    }
}

fn gap_is_illegal(family: UiAllocationStreamFamily) -> bool {
    matches!(
        family,
        UiAllocationStreamFamily::TextInput | UiAllocationStreamFamily::DurableResize
    )
}

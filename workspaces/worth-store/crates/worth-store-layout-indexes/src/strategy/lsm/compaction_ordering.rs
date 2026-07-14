use super::BaselineLsmCompactionPublicationReceipt;

use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LsmCompactionOrderingLaw;

impl LsmCompactionOrderingLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    /// Checks the complete fixed-shape run set carried by the WAL-owned
    /// receipt. No raw generation list can be admitted into compaction proof.
    pub(crate) fn verify_owner_receipt(
        self,
        receipt: &BaselineLsmCompactionPublicationReceipt,
    ) -> Result<(), StrategyDenial> {
        let inputs = receipt.input_runs();
        let generations_are_strict = inputs
            .windows(2)
            .all(|pair| pair[0].generation() < pair[1].generation());
        let roots_are_distinct = inputs[0].root_record() != inputs[1].root_record()
            && inputs[1].root_record() != inputs[2].root_record()
            && inputs[0].root_record() != inputs[2].root_record();
        let output_follows_inputs = receipt.output_generation() > inputs[2].generation();

        if generations_are_strict
            && roots_are_distinct
            && output_follows_inputs
            && receipt.stale_runs_retired()
        {
            return Ok(());
        }
        Err(StrategyDenial::CompactionOrderingViolation)
    }
}

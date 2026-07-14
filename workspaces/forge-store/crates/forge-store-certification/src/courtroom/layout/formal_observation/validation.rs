use crate::courtroom::layout::owner_scenarios::durable_observation::LayoutDurableObservationSource;
use forge_store_wal::BlobWalRecordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutFormalObservationDenial {
    LsmRoleMismatch,
    LsmSequenceNotStrict,
    TombstoneReplacementFrontierMismatch,
    ActivationCoverageMismatch,
    PhysicalRootDidNotAdvance,
}

#[derive(Debug, Clone, Copy)]
struct LayoutDurableFacts {
    value_kind: BlobWalRecordKind,
    generation_kind: BlobWalRecordKind,
    tombstone_kind: BlobWalRecordKind,
    value_sequence: u64,
    generation_sequence: u64,
    tombstone_sequence: u64,
    output_sequence: u64,
    activation_start: u64,
    activation_end: u64,
    roots_share_authority: bool,
    old_root_epoch: u64,
    new_root_epoch: u64,
}

pub(super) fn validate_layout_durable_observation(
    source: &LayoutDurableObservationSource,
) -> Result<(), LayoutFormalObservationDenial> {
    validate(LayoutDurableFacts::from_source(source))
}

fn validate(facts: LayoutDurableFacts) -> Result<(), LayoutFormalObservationDenial> {
    if facts.value_kind != BlobWalRecordKind::LsmValue
        || facts.generation_kind != BlobWalRecordKind::GenerationPublication
        || facts.tombstone_kind != BlobWalRecordKind::LsmTombstone
    {
        return Err(LayoutFormalObservationDenial::LsmRoleMismatch);
    }
    if facts.value_sequence >= facts.generation_sequence
        || facts.generation_sequence >= facts.tombstone_sequence
    {
        return Err(LayoutFormalObservationDenial::LsmSequenceNotStrict);
    }
    if facts.tombstone_sequence.checked_add(1) != Some(facts.output_sequence) {
        return Err(LayoutFormalObservationDenial::TombstoneReplacementFrontierMismatch);
    }
    if facts.activation_start > facts.value_sequence
        || facts
            .output_sequence
            .checked_add(1)
            .is_none_or(|end| facts.activation_end < end)
    {
        return Err(LayoutFormalObservationDenial::ActivationCoverageMismatch);
    }
    if !facts.roots_share_authority || facts.old_root_epoch >= facts.new_root_epoch {
        return Err(LayoutFormalObservationDenial::PhysicalRootDidNotAdvance);
    }
    Ok(())
}

impl LayoutDurableFacts {
    fn from_source(source: &LayoutDurableObservationSource) -> Self {
        let value = source.lsm_value();
        let generation = source.lsm_generation();
        let tombstone = source.lsm_tombstone();
        let output = source.lsm_output();
        let activation = source.lsm_activation();
        let old_root = source.physical_old_root();
        let new_root = source.physical_new_root();
        Self {
            value_kind: value.kind(),
            generation_kind: generation.kind(),
            tombstone_kind: tombstone.kind(),
            value_sequence: value.sequence(),
            generation_sequence: generation.sequence(),
            tombstone_sequence: tombstone.sequence(),
            output_sequence: output.sequence(),
            activation_start: activation.covered_lsn_start(),
            activation_end: activation.covered_lsn_end(),
            roots_share_authority: old_root.store_authority_identity()
                == new_root.store_authority_identity(),
            old_root_epoch: old_root.epoch().get(),
            new_root_epoch: new_root.epoch().get(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::courtroom::layout::owner_evidence::certify_layout_owner_execution_evidence;
    use crate::courtroom::layout::owner_scenarios::execute_declaration_owner_scenarios;

    fn durable_facts() -> LayoutDurableFacts {
        let execution = execute_declaration_owner_scenarios().unwrap();
        let evidence = certify_layout_owner_execution_evidence(execution).unwrap();
        let (_, _, durable) = evidence.into_parts();
        LayoutDurableFacts::from_source(&durable)
    }

    #[test]
    fn each_scalar_durable_validation_branch_rejects_its_mutant() {
        let valid = durable_facts();
        assert_eq!(validate(valid), Ok(()));

        let mut wrong_role = valid;
        wrong_role.value_kind = BlobWalRecordKind::GenerationPublication;
        assert_eq!(
            validate(wrong_role),
            Err(LayoutFormalObservationDenial::LsmRoleMismatch)
        );

        let mut non_strict = valid;
        non_strict.generation_sequence = non_strict.value_sequence;
        assert_eq!(
            validate(non_strict),
            Err(LayoutFormalObservationDenial::LsmSequenceNotStrict)
        );

        let mut wrong_frontier = valid;
        wrong_frontier.output_sequence = wrong_frontier.tombstone_sequence + 2;
        assert_eq!(
            validate(wrong_frontier),
            Err(LayoutFormalObservationDenial::TombstoneReplacementFrontierMismatch)
        );

        let mut uncovered = valid;
        uncovered.activation_start = uncovered.value_sequence + 1;
        assert_eq!(
            validate(uncovered),
            Err(LayoutFormalObservationDenial::ActivationCoverageMismatch)
        );

        let mut stale_root = valid;
        stale_root.new_root_epoch = stale_root.old_root_epoch;
        assert_eq!(
            validate(stale_root),
            Err(LayoutFormalObservationDenial::PhysicalRootDidNotAdvance)
        );
    }
}

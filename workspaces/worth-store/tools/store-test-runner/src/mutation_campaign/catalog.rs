use std::borrow::Cow;

mod phase_16;
mod physical_reconstruction_c6;
mod physical_reconstruction_c7;
mod physical_reconstruction_c8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ControlledMutation {
    pub(super) id: u8,
    pub(super) predicate: &'static str,
    pub(super) source: &'static str,
    pub(super) needle: &'static str,
    pub(super) replacement: &'static str,
    pub(super) package: &'static str,
    pub(super) target: MutationTarget,
    pub(super) selector: &'static str,
}

impl ControlledMutation {
    pub(super) fn source_occurrences(&self, source: &str) -> usize {
        let lf = source.matches(self.needle).count();
        let crlf_needle = self.needle.replace('\n', "\r\n");
        if crlf_needle == self.needle {
            lf
        } else {
            lf + source.matches(&crlf_needle).count()
        }
    }

    pub(super) fn source_needle(&self, source: &str) -> Cow<'static, str> {
        if source.contains(self.needle) {
            Cow::Borrowed(self.needle)
        } else {
            Cow::Owned(self.needle.replace('\n', "\r\n"))
        }
    }

    pub(super) fn source_replacement(&self, source: &str) -> Cow<'static, str> {
        if self.source_needle(source).contains("\r\n") {
            Cow::Owned(self.replacement.replace('\n', "\r\n"))
        } else {
            Cow::Borrowed(self.replacement)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MutationTarget {
    Library,
    LibraryWithFeatures { features: &'static str },
    Binary(&'static str),
    Integration(&'static str),
    NestedExecutableLibrary { features: &'static str },
}

pub(super) fn mutations() -> &'static [ControlledMutation] {
    static ALL: std::sync::OnceLock<Box<[ControlledMutation]>> = std::sync::OnceLock::new();
    ALL.get_or_init(|| {
        MUTATIONS
            .iter()
            .chain(phase_16::MUTATIONS)
            .chain(physical_reconstruction_c6::MUTATIONS)
            .chain(physical_reconstruction_c7::MUTATIONS)
            .chain(physical_reconstruction_c8::MUTATIONS)
            .chain(physical_reconstruction_c7::CLOSEOUT_COST_MUTATIONS)
            .chain(physical_reconstruction_c7::PROCESS_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_REOPEN_CLEANUP_MUTATIONS)
            .chain(physical_reconstruction_c7::LEDGER_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_LIFECYCLE_EVIDENCE_MUTATIONS)
            .chain(physical_reconstruction_c7::TIMING_GUARD_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_SUCCESSOR_CLEANUP_MUTATIONS)
            .chain(physical_reconstruction_c7::AUTHORITY_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::EVIDENCE_INTEGRITY_MUTATIONS)
            .copied()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

pub(super) const fn physical_work_mutations() -> &'static [ControlledMutation] {
    phase_16::MUTATIONS
}

#[cfg(test)]
pub(super) const fn physical_reconstruction_c6_mutations() -> &'static [ControlledMutation] {
    physical_reconstruction_c6::MUTATIONS
}

#[cfg(test)]
pub(super) fn physical_reconstruction_c7_mutations(
) -> impl Iterator<Item = &'static ControlledMutation> {
    physical_reconstruction_c7::MUTATIONS
        .iter()
        .chain(physical_reconstruction_c7::CLOSEOUT_COST_MUTATIONS)
        .chain(physical_reconstruction_c7::PROCESS_ACCOUNTING_MUTATIONS)
        .chain(physical_reconstruction_c7::WAL_REOPEN_CLEANUP_MUTATIONS)
        .chain(physical_reconstruction_c7::LEDGER_ACCOUNTING_MUTATIONS)
        .chain(physical_reconstruction_c7::WAL_LIFECYCLE_EVIDENCE_MUTATIONS)
        .chain(physical_reconstruction_c7::TIMING_GUARD_MUTATIONS)
        .chain(physical_reconstruction_c7::WAL_SUCCESSOR_CLEANUP_MUTATIONS)
        .chain(physical_reconstruction_c7::AUTHORITY_ACCOUNTING_MUTATIONS)
        .chain(physical_reconstruction_c7::EVIDENCE_INTEGRITY_MUTATIONS)
}

#[cfg(test)]
pub(super) const fn physical_reconstruction_c8_mutations() -> &'static [ControlledMutation] {
    physical_reconstruction_c8::MUTATIONS
}

pub(super) fn c8_closure_mutations() -> &'static [ControlledMutation] {
    physical_reconstruction_c8::MUTATIONS
}

pub(super) fn bounded_residency_mutations() -> &'static [ControlledMutation] {
    static BOUNDED: std::sync::OnceLock<Box<[ControlledMutation]>> = std::sync::OnceLock::new();
    BOUNDED.get_or_init(|| {
        phase_16::MUTATIONS
            .iter()
            .filter(|mutation| matches!(mutation.id, 42..=44))
            .chain(physical_reconstruction_c6::MUTATIONS)
            .chain(physical_reconstruction_c7::MUTATIONS)
            .chain(physical_reconstruction_c7::CLOSEOUT_COST_MUTATIONS)
            .chain(physical_reconstruction_c7::PROCESS_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_REOPEN_CLEANUP_MUTATIONS)
            .chain(physical_reconstruction_c7::LEDGER_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_LIFECYCLE_EVIDENCE_MUTATIONS)
            .chain(physical_reconstruction_c7::TIMING_GUARD_MUTATIONS)
            .chain(physical_reconstruction_c7::WAL_SUCCESSOR_CLEANUP_MUTATIONS)
            .chain(physical_reconstruction_c7::AUTHORITY_ACCOUNTING_MUTATIONS)
            .chain(physical_reconstruction_c7::EVIDENCE_INTEGRITY_MUTATIONS)
            .chain(physical_reconstruction_c8::MUTATIONS)
            .copied()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

macro_rules! mutation {
    ($id:literal, $predicate:literal, $source:literal, $needle:literal, $replacement:literal,
     $package:literal, $target:expr, $selector:literal) => {
        ControlledMutation {
            id: $id,
            predicate: $predicate,
            source: $source,
            needle: $needle,
            replacement: $replacement,
            package: $package,
            target: $target,
            selector: $selector,
        }
    };
}

const MUTATIONS: &[ControlledMutation] = &[
    mutation!(
        1,
        "publication-durability",
        "crates/worth-store/src/physical_runtime/record_serving/publication/director/root_candidate_execution.rs",
        "for index in 0..candidate.candidate().artifacts().len() {",
        "for index in 0..candidate.candidate().artifacts().len().saturating_sub(1) {",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "durability_admission::data_durability::root_projection_carriage::exact_settled_group_advances_only_after_replacement_and_namespace_durability"
    ),
    mutation!(
        2,
        "outcome-order",
        "crates/worth-store/src/physical_runtime/durability/publication/namespace_durability.rs",
        "PhysicalRootPublicationWorkAction::SynchronizeParentNamespace,",
        "PhysicalRootPublicationWorkAction::ReplaceBootstrapCatalog,",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "durability_admission::data_durability::root_signal_progression::root_candidate_signal_completion_precedes_replacement_and_current_root_advance"
    ),
    mutation!(
        3,
        "batch-atomicity",
        "crates/worth-store/src/physical_runtime/record_serving/planning/batch_placement.rs",
        "identities.iter().copied().zip(batch.records)",
        "identities.iter().copied().zip(batch.records).take(1)",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "page_packing_oracle::batch_packing_matches_an_independent_page_oracle"
    ),
    mutation!(
        4,
        "identity-authority",
        "crates/worth-store/src/physical_runtime/record_serving/identity.rs",
        "let mut allocation_epoch = [0_u8; 16];\n    getrandom::fill(&mut allocation_epoch)\n        .map_err(|_| RecordAppendDenial::IdentityEntropyUnavailable)?;",
        "let allocation_epoch = [1_u8; 16];",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "identity_process::admission_denials_have_no_effect_and_successors_receive_fresh_identity"
    ),
    mutation!(
        5,
        "identity-placement-seam",
        "crates/worth-store/src/physical_runtime/record_serving/planning/inline_segment_plan/page_allocation.rs",
        "DurableInlineRecordPlacement::new(\n                        input.record,",
        "DurableInlineRecordPlacement::new(\n                        PersistedRecordIdentity::new([slot.get() as u8; 16], u64::from(slot.get())).unwrap(),",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "segment_journeys::one_batch_rolls_across_four_segments_and_routes_without_scans"
    ),
    mutation!(
        6,
        "page-layout",
        "crates/worth-store/src/physical_runtime/record_serving/planning/inline_segment_plan/page_allocation.rs",
        "descriptor.slot(),\n            )\n            .with_slot_generation",
        "PhysicalRecordSlot::from_raw(descriptor.slot().get().saturating_add(1)).unwrap(),\n            )\n            .with_slot_generation",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "segment_journeys::cross_batch_page_reuse_is_cow_and_does_not_rebase_old_slots"
    ),
    mutation!(
        7,
        "lifecycle",
        "crates/worth-store/src/physical_runtime/record_serving/admission/transition.rs",
        "        Err(failure) => return open_failure(runtime, failure),\n    };\n    match record_open::load_current_root(",
        "        Err(BootstrapTransitionFailure::Denied(\n            super::super::RecordBootstrapDenial::RecordFamilyAbsent,\n        )) => {\n            let placement = crate::physical_runtime::PhysicalRecordPlacementPolicy::builder()\n                .admit(format)\n                .unwrap();\n            match initialization::initialize(\n                runtime.record_serving_media(),\n                format,\n                placement,\n                access,\n            ) {\n                Ok(bootstrap) => bootstrap,\n                Err(failure) => return open_failure(runtime, failure),\n            }\n        }\n        Err(failure) => return open_failure(runtime, failure),\n    };\n    match record_open::load_current_root(",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "baseline_admission::initialize_and_open_never_substitute_for_each_other"
    ),
    mutation!(
        8,
        "current-truth",
        "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        "RecordFamilyInventory::Residue => {\n            return Err(BootstrapTransitionFailure::Denied(\n                RecordBootstrapDenial::AmbiguousRecordFamilyResidue,\n            ));\n        }",
        "RecordFamilyInventory::Residue => {\n            let generation = worth_store_physical_format::CurrentRootCatalogGeneration::new(2).unwrap();\n            return Ok(PhysicalRecordBootstrapOwner { format, access, current_root: worth_store_physical_format::CurrentRootCatalogEntry::new(generation), observed_staging_residue: true });\n        }",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "manifest_scale::bounded_scale_identity_format_and_policy_courtroom"
    ),
    mutation!(
        9,
        "independent-decision-path",
        "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        "let generation = bootstrap.current_root.generation().get();",
        "let catalog_generation = bootstrap.current_root.generation().get();\n    let successor = catalog_generation.saturating_add(1);\n    let generation = if ServingRecordArtifacts::new(media, loader).load_bounded(allocation, RecordArtifactFile::RootManifest { generation: successor }, limits.current_root_bytes().get()).is_ok() { successor } else { catalog_generation };",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "baseline_admission::namespace_residue_cannot_elect_current_truth"
    ),
    mutation!(
        10,
        "minimum-integrity",
        "crates/worth-store-physical-format/src/record_framing/durable_frame.rs",
        "if stored != actual {",
        "if false && stored != actual {",
        "worth-store-physical-format",
        MutationTarget::Library,
        "binary_format::record_golden_bytes::checksum_covers_identity_header_and_full_payload"
    ),
    mutation!(
        11,
        "placement-generation",
        "crates/worth-store-physical-format/src/extent_record/durable_extent.rs",
        "if u64::from_le_bytes(frame.payload[32..40].try_into().unwrap())\n        != expected.extent.generation().get()\n    {",
        "if false && u64::from_le_bytes(frame.payload[32..40].try_into().unwrap())\n        != expected.extent.generation().get()\n    {",
        "worth-store-physical-format",
        MutationTarget::Library,
        "extent_record::tests::durable_extent_decode_rejects_a_stale_placement_generation"
    ),
    mutation!(
        12,
        "locate-open-scale",
        "crates/worth-store/src/physical_runtime/record_serving/access/manifest_routing/reader.rs",
        "if !reference.contains(record) {",
        "let mut pending = vec![reference];\n        while let Some(candidate) = pending.pop() {\n            if let PhysicalRootRoutingBlock::Branch { children, .. } = self.read_block(allocation, candidate, counters)? { pending.extend(children); }\n        }\n        if !reference.contains(record) {",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "manifest_scale::bounded_scale_identity_format_and_policy_courtroom"
    ),
    mutation!(
        13,
        "transfer-allocation-slope",
        "crates/worth-store/src/physical_runtime/record_serving/access/extent_read_session.rs",
        "let payload_bytes = (self.manifest.logical_bytes() - self.logical_offset)\n            .min(u64::from(self.manifest.chunk_payload_capacity()))\n            as usize;",
        "let payload_bytes = (self.manifest.logical_bytes() - self.logical_offset) as usize;",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "extent_streaming::extent_allocation_peak_is_independent_of_logical_record_length"
    ),
];

#[cfg(test)]
#[path = "source_binding_tests.rs"]
mod source_binding_tests;

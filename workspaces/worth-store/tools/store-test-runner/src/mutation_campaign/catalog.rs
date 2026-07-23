use std::borrow::Cow;

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
    pub(super) fn source_needle(&self, source: &str) -> Cow<'static, str> {
        source_line_ending(self.needle, source)
    }

    pub(super) fn source_replacement(&self, source: &str) -> Cow<'static, str> {
        source_line_ending(self.replacement, source)
    }
}

fn source_line_ending(template: &'static str, source: &str) -> Cow<'static, str> {
    if source.contains("\r\n") {
        Cow::Owned(template.replace('\n', "\r\n"))
    } else {
        Cow::Borrowed(template)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MutationTarget {
    Library,
    Integration(&'static str),
}

pub(super) fn mutations() -> &'static [ControlledMutation] {
    MUTATIONS
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
        "crates/worth-store/src/physical_runtime/record_serving/publication/publication_progression.rs",
        ".synchronize_artifact(artifact)\n            .and_then(|()| artifacts.synchronize_artifact_parent(artifact))",
        ".synchronize_artifact_parent(artifact)",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "publication_faults::publication_barrier_omission_is_observable"
    ),
    mutation!(
        2,
        "outcome-order",
        "crates/worth-store/src/physical_runtime/record_serving/publication/publication_progression.rs",
        "let namespace_synchronized =\n        synchronize_namespace(media, &artifacts, catalog_replaced, counters_before)?;",
        "let namespace_synchronized = NamespaceSynchronized(catalog_replaced.0);",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "publication_faults::possible_catalog_cutover_is_typed_indeterminate_and_close_adds_no_publication_effect"
    ),
    mutation!(
        3,
        "batch-atomicity",
        "crates/worth-store/src/physical_runtime/record_serving/planning/batch_placement.rs",
        "identities.iter().copied().zip(batch.records)",
        "identities.iter().copied().zip(batch.records).take(1)",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "publication_mutants::premature_identity_subset_and_success_mutants_fail_causally"
    ),
    mutation!(
        4,
        "identity-authority",
        "crates/worth-store/src/physical_runtime/record_serving/identity.rs",
        "let mut allocation_epoch = [0_u8; 16];\n    getrandom::fill(&mut allocation_epoch)\n        .map_err(|_| RecordAppendDenial::IdentityEntropyUnavailable)?;",
        "let allocation_epoch = [1_u8; 16];",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "extent_streaming::abandoned_candidate_identity_is_never_reused_by_a_later_publication"
    ),
    mutation!(
        5,
        "identity-placement-seam",
        "crates/worth-store/src/physical_runtime/record_serving/planning/inline_segment_plan.rs",
        "DurableInlineRecordPlacement::new(\n                        input.record,",
        "DurableInlineRecordPlacement::new(\n                        PersistedRecordIdentity::new([slot.get() as u8; 16], u64::from(slot.get())).unwrap(),",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "segment_journeys::one_batch_rolls_across_four_segments_and_routes_without_scans"
    ),
    mutation!(
        6,
        "page-layout",
        "crates/worth-store/src/physical_runtime/record_serving/planning/inline_segment_plan.rs",
        "descriptor.slot(),\n            )\n            .with_slot_generation",
        "PhysicalRecordSlot::from_raw(descriptor.slot().get().saturating_add(1)).unwrap(),\n            )\n            .with_slot_generation",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "segment_journeys::cross_batch_page_reuse_is_cow_and_does_not_rebase_old_slots"
    ),
    mutation!(
        7,
        "lifecycle",
        "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        "RecordFamilyInventory::ProvenAbsent => {\n            return Err(BootstrapTransitionFailure::Denied(\n                RecordBootstrapDenial::RecordFamilyAbsent,\n            ));\n        }",
        "RecordFamilyInventory::ProvenAbsent => {\n            let placement = super::PhysicalRecordPlacementPolicy::builder().admit(request.format).unwrap();\n            return super::initialization::initialize(media, super::PhysicalRecordInitialization::new(request.format, placement, request.access));\n        }",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "initialize_and_open_never_substitute_for_each_other"
    ),
    mutation!(
        8,
        "current-truth",
        "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        "RecordFamilyInventory::Residue => {\n            return Err(BootstrapTransitionFailure::Denied(\n                RecordBootstrapDenial::AmbiguousRecordFamilyResidue,\n            ));\n        }",
        "RecordFamilyInventory::Residue => {\n            let generation = worth_store_physical_format::CurrentRootCatalogGeneration::new(2).unwrap();\n            return Ok(PhysicalRecordBootstrapOwner { format: request.format, access: request.access, current_root: worth_store_physical_format::CurrentRootCatalogEntry::new(generation), observed_staging_residue: true });\n        }",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "manifest_scale::bounded_scale_identity_format_and_policy_courtroom"
    ),
    mutation!(
        9,
        "independent-decision-path",
        "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        "let generation = bootstrap.current_root.generation().get();",
        "let catalog_generation = bootstrap.current_root.generation().get();\n    let successor = catalog_generation.saturating_add(1);\n    let generation = if PhysicalRecordArtifacts::new(media).read_bounded(RecordArtifactFile::RootManifest { generation: successor }, limits.current_root_bytes().get()).is_ok() { successor } else { catalog_generation };",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "publication_failure_topology::publication_cutover_never_invents_current_truth"
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
        "let mut pending = vec![reference];\n        while let Some(candidate) = pending.pop() {\n            if let PhysicalRootRoutingBlock::Branch { children, .. } = self.read_block(candidate, counters)? { pending.extend(children); }\n        }\n        if !reference.contains(record) {",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "manifest_scale::bounded_scale_identity_format_and_policy_courtroom"
    ),
    mutation!(
        13,
        "transfer-allocation-slope",
        "crates/worth-store/src/physical_runtime/record_serving/access/extent_read_session.rs",
        "let chunk_bytes = (self.manifest.logical_bytes() - self.logical_offset)\n            .min(u64::from(self.manifest.chunk_payload_capacity()))\n            as usize;",
        "let chunk_bytes = (self.manifest.logical_bytes() - self.logical_offset) as usize;",
        "worth-store",
        MutationTarget::Integration("physical_record_journeys"),
        "extent_streaming::extent_allocation_peak_is_independent_of_logical_record_length"
    ),
    mutation!(
        14,
        "publication-ownership",
        "crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency.rs",
        "let physical =\n            store_write(resident.bytes()).map_err(CandidateFrameWriteFailure::Backend)?;\n        let completion = resident\n            .publish_clean(&physical)\n            .map_err(CandidateFrameWriteFailure::Residency)?;",
        "let detached = resident.bytes().to_vec();\n        let physical = CandidateFramePhysicalWrite::for_contract_test();\n        let completion = resident\n            .publish_clean(&physical)\n            .map_err(CandidateFrameWriteFailure::Residency)?;\n        store_write(&detached).map_err(CandidateFrameWriteFailure::Backend)?;",
        "worth-store",
        MutationTarget::Library,
        "physical_runtime::record_serving::residency::candidate_frame_residency::tests::publication_ownership::residency_covers_store_write"
    ),
];

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::mutations;

    #[test]
    fn every_mutant_is_bound_to_one_current_source_seam() {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .unwrap();
        let mut identities = BTreeSet::new();
        for mutation in mutations() {
            assert!(
                identities.insert(mutation.id),
                "duplicate mutant {}",
                mutation.id
            );
            let source = workspace.join(mutation.source);
            let contents = std::fs::read_to_string(&source)
                .unwrap_or_else(|error| panic!("cannot read {}: {error}", source.display()));
            let needle = mutation.source_needle(&contents);
            assert_eq!(
                contents.matches(needle.as_ref()).count(),
                1,
                "mutant {} must bind exactly once in {}",
                mutation.id,
                source.display()
            );
        }
    }

    #[test]
    fn mutation_source_binding_follows_lf_and_crlf_without_changing_the_seam() {
        let mutation = &mutations()[0];
        let lf_source = mutation.needle.to_owned();
        let crlf_source = mutation.needle.replace('\n', "\r\n");

        assert_eq!(
            lf_source
                .matches(mutation.source_needle(&lf_source).as_ref())
                .count(),
            1
        );
        assert_eq!(
            crlf_source
                .matches(mutation.source_needle(&crlf_source).as_ref())
                .count(),
            1
        );
        let crlf_replacement = mutation.source_replacement(&crlf_source);
        assert_eq!(
            crlf_replacement.matches("\r\n").count(),
            mutation.replacement.matches('\n').count()
        );
        assert!(!crlf_replacement.replace("\r\n", "").contains('\n'));
    }
}

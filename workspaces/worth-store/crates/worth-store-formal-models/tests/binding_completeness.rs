use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use worth_store_formal_models::{
    classify_owner_observation_omission, current_compaction_visibility_mappings,
    current_compaction_visibility_owner_cases, current_protocol_binding_manifest,
    require_compaction_visibility_refinement_coverage, CompactionVisibilityRefinementCoverageIssue,
    OwnerCrashSurvivalPosture, OwnerEvidenceClass, OwnerObservationOmissionCause,
    OwnerObservationOmissionVerdict, ProtocolFamily,
};

#[test]
fn every_protocol_is_directly_bound_gapped_or_composed() {
    let manifest = current_protocol_binding_manifest();
    let directly_bound = manifest
        .bindings()
        .map(|binding| binding.protocol())
        .collect::<BTreeSet<_>>();
    let gapped = manifest
        .gaps()
        .map(|gap| gap.protocol())
        .collect::<BTreeSet<_>>();
    let composed = manifest.composed_protocols().collect::<BTreeSet<_>>();

    for protocol in ProtocolFamily::all() {
        assert!(
            directly_bound.contains(&protocol)
                || gapped.contains(&protocol)
                || composed.contains(&protocol),
            "missing boundary for {protocol:?}"
        );
        if composed.contains(&protocol) {
            assert!(!directly_bound.contains(&protocol));
            assert!(!gapped.contains(&protocol));
        }
    }
}

#[test]
fn replication_model_replaces_the_last_staged_binding_gap() {
    let manifest = current_protocol_binding_manifest();
    let gaps = manifest
        .gaps()
        .map(|gap| (gap.protocol(), gap.reason()))
        .collect::<BTreeSet<_>>();
    assert!(gaps.is_empty());
    assert!(!gaps
        .iter()
        .any(|(protocol, _)| *protocol == ProtocolFamily::ImportPublication));
    assert!(manifest
        .bindings()
        .any(|binding| binding.protocol() == ProtocolFamily::ImportPublication));
    assert!(manifest
        .bindings()
        .any(|binding| binding.protocol() == ProtocolFamily::ReplicationAdmission));
}

#[test]
fn staged_models_are_not_exported_before_their_checked_artifacts_exist() {
    let manifest = current_protocol_binding_manifest();
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocols_root = crate_root.join("src/protocols");
    let protocol_exports = fs::read_to_string(protocols_root.join("mod.rs"))
        .expect("formal protocol facade is committed");

    for gap in manifest.gaps() {
        let module_name = match gap.protocol() {
            ProtocolFamily::ImportPublication => "import_publication",
            ProtocolFamily::ReplicationAdmission => "replication_admission",
            protocol => panic!("unexpected current capability gap for {protocol:?}"),
        };
        assert!(
            !protocol_exports.contains(&format!("mod {module_name}")),
            "gapped protocol {module_name} must not be exported"
        );
        assert!(
            !protocols_root.join(module_name).join("model.tla").exists(),
            "gapped protocol {module_name} must not contain a checked model"
        );
        assert!(
            !protocols_root.join(module_name).join("model.cfg").exists(),
            "gapped protocol {module_name} must not contain a model configuration"
        );
    }
}

#[test]
fn current_owner_inventories_map_exactly_once_without_strings() {
    let declared = current_compaction_visibility_owner_cases().collect::<Vec<_>>();
    let mapped = current_compaction_visibility_mappings().collect::<Vec<_>>();

    let receipt = require_compaction_visibility_refinement_coverage(
        declared.iter().copied(),
        declared.iter().copied(),
        mapped,
    )
    .expect("every concrete owner case has one exhaustive model mapping");

    assert_eq!(receipt.declared_owner_cases(), declared.len());
    assert_eq!(receipt.ordinary_executed_cases(), declared.len());
    assert_eq!(receipt.mapped_model_actions(), declared.len());
}

#[test]
fn changed_owner_case_sets_fail_until_mapping_is_updated() {
    let declared = current_compaction_visibility_owner_cases().collect::<Vec<_>>();
    let mut mapped = current_compaction_visibility_mappings().collect::<Vec<_>>();
    let omitted = mapped.pop().expect("current manifest has owner cases");

    let denial = require_compaction_visibility_refinement_coverage(
        declared.iter().copied(),
        declared.iter().copied(),
        mapped,
    )
    .expect_err("a missing abstraction edge must block conformance");

    assert!(denial.issues().contains(
        &CompactionVisibilityRefinementCoverageIssue::MissingModelMapping(omitted.owner_case())
    ));
}

#[test]
fn every_binding_names_its_crash_survival_posture() {
    let manifest = current_protocol_binding_manifest();
    assert!(manifest.bindings().all(|binding| {
        matches!(
            binding.source().crash_survival_posture(),
            OwnerCrashSurvivalPosture::DurableAcrossProcessLoss
                | OwnerCrashSurvivalPosture::ReconstructedAfterReopen
                | OwnerCrashSurvivalPosture::LostWithProcess
                | OwnerCrashSurvivalPosture::ForbiddenAsProtocolEvidence
        )
    }));
}

#[test]
fn every_binding_names_a_typed_model_action_family() {
    let manifest = current_protocol_binding_manifest();
    assert!(manifest.bindings().all(|binding| {
        binding.model_action_family() == binding.operation().model_action_family()
            && binding
                .protocol()
                .admits_model_action_family(binding.model_action_family())
    }));
}

#[test]
fn crash_loss_classification_distinguishes_durable_reopened_and_ephemeral_evidence() {
    let cause = OwnerObservationOmissionCause::LostAcrossCrash;
    let durable =
        classify_owner_observation_omission(OwnerEvidenceClass::DurableAuthoritativeReceipt, cause);
    let reopened =
        classify_owner_observation_omission(OwnerEvidenceClass::ReopenedObservedReceipt, cause);
    let ephemeral =
        classify_owner_observation_omission(OwnerEvidenceClass::EphemeralDiagnosticTrace, cause);

    assert_eq!(
        durable,
        OwnerObservationOmissionVerdict::IllegalProtocolHole
    );
    assert_eq!(
        reopened,
        OwnerObservationOmissionVerdict::InstrumentationDefect
    );
    assert_eq!(
        ephemeral,
        OwnerObservationOmissionVerdict::CrashLostEphemeralDiagnostic
    );
    assert_ne!(durable, reopened);
    assert_ne!(reopened, ephemeral);
    assert_ne!(durable, ephemeral);
}

#[test]
fn source_manifest_is_compiler_bound_to_owner_types() {
    let manifest = current_protocol_binding_manifest();
    assert!(manifest.bindings().all(|binding| {
        let source = binding.source().rust_type();
        source.starts_with("worth_store_") && !source.contains("formal_models")
    }));
}

#[test]
fn current_boundary_roles_are_unique_and_all_evidence_classes_are_visible() {
    let manifest = current_protocol_binding_manifest();
    let bindings = manifest.bindings().collect::<Vec<_>>();
    let unique = bindings
        .iter()
        .map(|binding| {
            (
                binding.protocol(),
                binding.owner(),
                binding.operation(),
                binding.source().rust_type(),
            )
        })
        .collect::<BTreeSet<_>>();
    let evidence_classes = bindings
        .iter()
        .map(|binding| binding.source().evidence_class())
        .collect::<BTreeSet<_>>();

    assert_eq!(unique.len(), bindings.len());
    assert_eq!(
        evidence_classes,
        BTreeSet::from([
            OwnerEvidenceClass::DurableAuthoritativeReceipt,
            OwnerEvidenceClass::ReopenedObservedReceipt,
            OwnerEvidenceClass::EphemeralDiagnosticTrace,
            OwnerEvidenceClass::ForbiddenAuthoritySubstitute,
        ])
    );
}

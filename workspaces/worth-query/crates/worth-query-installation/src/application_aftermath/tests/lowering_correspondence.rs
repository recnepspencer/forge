//! Pre-image coverage, residue, owner identity, and R8.9 lowering denials.

use worth_query_declaration::facade::application_aftermath::DeclaredApplicationAftermathContract;

use super::super::{PublishedAftermathPosture, WorthQueryAftermathInstallationDenialKind};
use super::{binding, digest, geometry_catalog, protocol, recorded_inverse, AftermathInstall};

#[test]
fn preimage_demand_must_be_covered_by_declared_reads() {
    let denial = AftermathInstall::new(binding(digest(17), digest(18), 1), "freeze-account")
        .reads(["balance"])
        .catalog(geometry_catalog(1, digest(18)))
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("secret-field"),
        ))
        .expect_err("uncovered pre-image demand must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::PreImageDemandNotCoveredByDeclaredReads
    );
}

#[test]
fn an_operation_declaring_no_reads_denies_distinctly_from_a_missed_slot() {
    // Distinct from the case above on purpose. "You declared reads and missed
    // one" and "you declared no reads at all" are different installation
    // mistakes, and only the second one tells the installer the operation is
    // missing its `graph_reads` entirely. Behind the per-slot loop this arm was
    // unreachable, so the denial kind existed without ever being able to fire.
    let denial = AftermathInstall::new(binding(digest(21), digest(22), 1), "freeze-account")
        .catalog(geometry_catalog(1, digest(22)))
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect_err("an operation with no declared reads cannot cover a demand");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::MissingDeclaredReadsCoverage
    );
}

#[test]
fn preimage_covered_demand_installs() {
    let installed = AftermathInstall::new(binding(digest(19), digest(20), 1), "freeze-account")
        .reads(["balance"])
        .catalog(geometry_catalog(1, digest(20)))
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect("covered pre-image demand must install");
    assert_eq!(
        installed.published_posture(),
        PublishedAftermathPosture::Reversible
    );
}

/// Q8.25-C1: the reversibility guard reads the operation's escaping lane.
///
/// The identical declaration one line above installs as `Reversible`. The only
/// delta here is that the operation definition declares an external-effect slot on the
/// schema — the lane that actually co-commits an outbox record and dispatches.
/// While the aftermath carried its own escaping posture, an author who declared
/// the real lane and left that posture at `None` got an undoable operation over
/// an effect that had already left the process, and no check ever compared the
/// two declarations.
#[test]
fn an_operation_that_escapes_cannot_install_as_reversible() {
    let denial = AftermathInstall::new(binding(digest(19), digest(20), 1), "freeze-account")
        .reads(["balance"])
        .catalog(geometry_catalog(1, digest(20)))
        .escaping()
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect_err("an escaping operation cannot be reversible");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::ExternalEffectRejectsReversible
    );
    assert_eq!(
        denial.subject(),
        "escaped-rail",
        "the denial names the rail the operation escapes through"
    );
}

/// Q8.25-C1: the installed posture is the operation's lane, not a second claim.
///
/// Two installs of the *same* declared aftermath contract differ only in whether
/// the operation declared the escaping lane, and the installed posture follows
/// the lane both ways. There is no aftermath vocabulary that could disagree.
#[test]
fn the_installed_external_posture_follows_the_operation_lane() {
    use super::super::InstalledExternalEffectPosture;

    let declared = DeclaredApplicationAftermathContract::not_correctable();
    let quiet = AftermathInstall::new(binding(digest(31), digest(32), 1), "release-estate")
        .install(&declared)
        .expect("a non-escaping operation installs");
    let escaping = AftermathInstall::new(binding(digest(31), digest(32), 1), "release-estate")
        .escaping()
        .install(&declared)
        .expect("an escaping operation installs when it is not reversible");

    assert_eq!(
        quiet.external_effect(),
        &InstalledExternalEffectPosture::None
    );
    assert_eq!(
        escaping.external_effect(),
        &InstalledExternalEffectPosture::Declared {
            correlation_family: "escaped-rail".to_owned()
        }
    );
    assert_ne!(
        quiet.identity().bytes(),
        escaping.identity().bytes(),
        "the installed aftermath identity must move when the escaping lane does"
    );
}

#[test]
fn stable_wire_type_drift_changes_installed_aftermath_identity() {
    let declared = DeclaredApplicationAftermathContract::not_correctable();
    let owner = binding(digest(33), digest(34), 1);
    let left = AftermathInstall::new(owner.clone(), "same-operation")
        .escaping_with_protocol(protocol(1))
        .install(&declared)
        .expect("the first protocol installs");
    let right = AftermathInstall::new(owner, "same-operation")
        .escaping_with_protocol(protocol(2))
        .install(&declared)
        .expect("the second protocol installs");

    assert_ne!(left.identity().bytes(), right.identity().bytes());
}

#[test]
fn rust_payload_type_drift_changes_only_the_in_process_installed_axis() {
    let declared = DeclaredApplicationAftermathContract::not_correctable();
    let owner = binding(digest(35), digest(36), 1);
    let protocol = protocol(1);
    let left = AftermathInstall::new(owner.clone(), "same-operation")
        .escaping_with_contract("original::EscapingPayload", protocol.clone())
        .install(&declared)
        .expect("the original Rust type installs");
    let right = AftermathInstall::new(owner, "same-operation")
        .escaping_with_contract("moved::EscapingPayload", protocol)
        .install(&declared)
        .expect("the moved Rust type installs");

    assert_ne!(left.identity().bytes(), right.identity().bytes());
}

#[test]
fn identical_declared_different_operation_slots_produce_distinct_identities() {
    let package = digest(27);
    let schema = digest(28);
    let declared = DeclaredApplicationAftermathContract::not_correctable();
    let left = AftermathInstall::new(binding(package, schema, 1), "operation-alpha")
        .install(&declared)
        .unwrap();
    let right = AftermathInstall::new(binding(package, schema, 1), "operation-beta")
        .install(&declared)
        .unwrap();
    assert_ne!(left.identity().bytes(), right.identity().bytes());
}

#[test]
fn residue_forbids_predecessor_aftermath_vocabularies() {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/application_aftermath/mod.rs"
    ));
    for forbidden in [
        "ProvisionalDiscard",
        "NoMutation",
        "RebuildRequired",
        "DeclarationIncomplete",
        "WorthQueryOperationReversalContract",
        "hash_parts",
        "install_irreversible_aftermath",
        "[0x11; 32]",
    ] {
        assert!(
            !source.contains(forbidden),
            "application_aftermath must not retain predecessor vocabulary {forbidden}"
        );
    }
}

#[test]
fn owner_identity_digest_binds_classification_to_domain_identity() {
    let left_owner = super::super::aftermath_owner_identity_digest("WORTH.tests", "geometry", 1, 0)
        .expect("left owner");
    let right_owner = super::super::aftermath_owner_identity_digest("WORTH.ui", "runtime", 1, 0)
        .expect("right owner");
    assert_ne!(left_owner.bytes(), right_owner.bytes());
    let declared = DeclaredApplicationAftermathContract::not_correctable();
    let left = AftermathInstall::new(binding(left_owner, left_owner, 1), "same-operation")
        .install(&declared)
        .unwrap();
    let right = AftermathInstall::new(binding(right_owner, right_owner, 1), "same-operation")
        .install(&declared)
        .unwrap();
    assert_ne!(left.identity().bytes(), right.identity().bytes());
}

#[test]
fn published_posture_exhaustive_match_has_no_provisional_or_nomutation() {
    let postures = [
        PublishedAftermathPosture::Reversible,
        PublishedAftermathPosture::Compensatable,
        PublishedAftermathPosture::Reconcilable,
        PublishedAftermathPosture::Irreversible,
    ];
    for posture in postures {
        match posture {
            PublishedAftermathPosture::Reversible
            | PublishedAftermathPosture::Compensatable
            | PublishedAftermathPosture::Reconcilable
            | PublishedAftermathPosture::Irreversible => {}
        }
    }
}

/// Positive twin for the irreversible compile-fail UI case: irreversible next
/// actions are constructible and expose no correction method names in source.
#[test]
fn irreversible_next_action_contract_has_no_undo_method_in_source() {
    let source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/application_aftermath/next_action_contract.rs"
    ));
    let irreversible_impl = source
        .split("impl IrreversibleNextActionContract")
        .nth(1)
        .expect("irreversible impl");
    let irreversible_impl = irreversible_impl
        .split("impl InstalledAftermathNextActionContract")
        .next()
        .expect("irreversible impl body");
    for method in ["fn undo", "fn compensate", "fn reconcile"] {
        assert!(
            !irreversible_impl.contains(method),
            "IrreversibleNextActionContract must not expose {method}"
        );
    }
}

#[test]
fn lowering_correspondence_unresolved_denies_at_install() {
    let denial = AftermathInstall::new(binding(digest(21), digest(22), 1), "freeze-account")
        .reads(["balance"])
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect_err("unresolved catalog must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::LoweringCorrespondenceUnresolved
    );
}

#[test]
fn lowering_correspondence_wrong_generation_denies_at_install() {
    let denial = AftermathInstall::new(binding(digest(23), digest(24), 1), "freeze-account")
        .reads(["balance"])
        .catalog(geometry_catalog(9, digest(24)))
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect_err("wrong generation must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::LoweringCorrespondenceWrongGeneration
    );
}

#[test]
fn lowering_correspondence_mismatched_graph_participation_denies_at_install() {
    let denial = AftermathInstall::new(binding(digest(25), digest(26), 1), "freeze-account")
        .reads(["balance"])
        .catalog(geometry_catalog(1, digest(99)))
        .install(&DeclaredApplicationAftermathContract::runtime_alone(
            recorded_inverse("balance"),
        ))
        .expect_err("mismatched graph participation must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryAftermathInstallationDenialKind::LoweringCorrespondenceMismatchedGraphParticipation
    );
}

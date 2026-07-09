#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
mod support;

use worth_foundational::{
    CanonicalBasisEntry, CanonicalBasisLocus, CanonicalBasisValue, CanonicalIntegerWidth,
    InternedString,
};
use worth_proof::BasisPostureKind;
use worth_store_physical_isolation::{
    compare_physical_epoch_vectors_with_evidence, EpochComparisonScope, EpochRetryDecision,
    PhysicalEpochDriftKind, PhysicalEpochVector,
};
use support::{
    current_generation_extent_reference, current_generation_page_reference,
    current_generation_segment_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

#[test]
fn stale_epoch_comparison_lowers_exact_foundational_and_proof_basis() {
    let authority = physical_authority_from_complete_closeout();
    let current_root = current_root_from_authority(&authority);
    let page_one = current_root
        .admit_page_publication_epoch(current_generation_page_reference(1))
        .unwrap();
    let page_two = current_root
        .admit_page_publication_epoch(current_generation_page_reference(2))
        .unwrap();
    let expected = PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(
        current_root.scope(),
    ))
    .with_root(current_root.epoch())
    .with_manifest(current_root.manifest_epoch())
    .with_page(page_one.epoch())
    .seal()
    .unwrap();
    let observed = PhysicalEpochVector::for_scope(expected.scope())
        .with_root(current_root.epoch())
        .with_manifest(current_root.manifest_epoch())
        .with_page(page_two.epoch())
        .seal()
        .unwrap();

    let evidence = compare_physical_epoch_vectors_with_evidence(expected, observed).unwrap();
    let entries = evidence.foundational_basis().payload().entries();

    assert_eq!(evidence.freshness().decision(), EpochRetryDecision::Retry);
    assert_eq!(
        evidence.freshness().drift(),
        Some(PhysicalEpochDriftKind::PageEpoch)
    );
    assert_eq!(evidence.proof_evidence().freshness(), &evidence.freshness());
    assert_eq!(
        evidence.proof_evidence().basis().freshness(),
        evidence.freshness()
    );
    assert_eq!(
        evidence.proof_evidence().basis_posture(),
        BasisPostureKind::StaleReadable
    );
    assert_text_entry(entries, "decision", "retry");
    assert_text_entry(entries, "drift", "page-epoch");
    assert_u64_entry(entries, "expected.page", page_one.epoch().get());
    assert_u64_entry(entries, "observed.page", page_two.epoch().get());
}

#[test]
fn scope_mismatch_lowers_exact_rebind_foundational_and_proof_basis() {
    let authority = physical_authority_from_complete_closeout();
    let current_root = current_root_from_authority(&authority);
    let expected = PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(
        current_root.scope(),
    ))
    .with_root(current_root.epoch())
    .with_manifest(current_root.manifest_epoch())
    .seal()
    .unwrap();
    let observed = PhysicalEpochVector::for_scope(EpochComparisonScope::root_readmission(
        current_root.scope(),
    ))
    .with_root(current_root.epoch())
    .with_manifest(current_root.manifest_epoch())
    .seal()
    .unwrap();

    let evidence = compare_physical_epoch_vectors_with_evidence(expected, observed).unwrap();
    let entries = evidence.foundational_basis().payload().entries();

    assert_eq!(
        evidence.proof_evidence().basis_posture(),
        BasisPostureKind::RebindRequired
    );
    assert_text_entry(entries, "decision", "rebind-required");
    assert_text_entry(entries, "drift", "scope-mismatch");
    assert_text_entry(entries, "expected.scope.kind", "read-plan-admission");
    assert_text_entry(entries, "observed.scope.kind", "root-readmission");
}

#[test]
fn full_scoped_epoch_vector_equivalence_lowers_exact_current_foundational_basis() {
    let authority = physical_authority_from_complete_closeout();
    let current_root = current_root_from_authority(&authority);
    let segment_epoch = current_root
        .admit_segment_publication_epoch(current_generation_segment_reference(101))
        .unwrap()
        .epoch();
    let extent_epoch = current_root
        .admit_extent_publication_epoch(current_generation_extent_reference(103))
        .unwrap()
        .epoch();
    let page_epoch = current_root
        .admit_page_publication_epoch(current_generation_page_reference(107))
        .unwrap()
        .epoch();
    let expected = PhysicalEpochVector::for_scope(EpochComparisonScope::read_plan_admission(
        current_root.scope(),
    ))
    .with_root(current_root.epoch())
    .with_manifest(current_root.manifest_epoch())
    .with_segment(segment_epoch)
    .with_extent(extent_epoch)
    .with_page(page_epoch)
    .seal()
    .unwrap();
    let observed = PhysicalEpochVector::for_scope(expected.scope())
        .with_root(current_root.epoch())
        .with_manifest(current_root.manifest_epoch())
        .with_segment(segment_epoch)
        .with_extent(extent_epoch)
        .with_page(page_epoch)
        .seal()
        .unwrap();

    let evidence = compare_physical_epoch_vectors_with_evidence(expected, observed).unwrap();
    let entries = evidence.foundational_basis().payload().entries();

    assert_eq!(evidence.freshness().decision(), EpochRetryDecision::Current);
    assert_eq!(evidence.freshness().drift(), None);
    assert_eq!(
        evidence.proof_evidence().basis_posture(),
        BasisPostureKind::CurrentValidity
    );
    assert_eq!(entries.len(), 18);
    assert_text_entry(entries, "decision", "current");
    assert_text_entry(entries, "drift", "none");
    assert_text_entry(entries, "expected.scope.kind", "read-plan-admission");
    assert_u64_entry(entries, "expected.scope.root", current_root.scope());
    assert_u64_entry(entries, "expected.root", current_root.epoch().get());
    assert_u64_entry(
        entries,
        "expected.manifest",
        current_root.manifest_epoch().get(),
    );
    assert_u64_entry(entries, "expected.segment", segment_epoch.get());
    assert_u64_entry(entries, "expected.extent", extent_epoch.get());
    assert_u64_entry(entries, "expected.page", page_epoch.get());
    assert_text_entry(entries, "expected.chunk", "none");
    assert_text_entry(entries, "observed.scope.kind", "read-plan-admission");
    assert_u64_entry(entries, "observed.scope.root", current_root.scope());
    assert_u64_entry(entries, "observed.root", current_root.epoch().get());
    assert_u64_entry(
        entries,
        "observed.manifest",
        current_root.manifest_epoch().get(),
    );
    assert_u64_entry(entries, "observed.segment", segment_epoch.get());
    assert_u64_entry(entries, "observed.extent", extent_epoch.get());
    assert_u64_entry(entries, "observed.page", page_epoch.get());
    assert_text_entry(entries, "observed.chunk", "none");
}

fn assert_u64_entry(entries: &[CanonicalBasisEntry], locus: &'static str, expected: u64) {
    assert_eq!(
        entry_value(entries, locus),
        &CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: expected.into(),
        }
    );
}

fn assert_text_entry(entries: &[CanonicalBasisEntry], locus: &'static str, expected: &'static str) {
    assert_eq!(
        entry_value(entries, locus),
        &CanonicalBasisValue::ExactText(expected.into())
    );
}

fn entry_value<'a>(
    entries: &'a [CanonicalBasisEntry],
    locus: &'static str,
) -> &'a CanonicalBasisValue {
    let expected_locus = CanonicalBasisLocus::Named(InternedString::from(locus));
    entries
        .iter()
        .find(|entry| entry.locus() == &expected_locus)
        .unwrap_or_else(|| panic!("missing canonical basis entry for {locus}"))
        .value()
}

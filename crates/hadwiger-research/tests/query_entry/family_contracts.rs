use forge_query::facade::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationLegalityClass,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryGroupedDeclarationPosture, ForgeQueryLowerAuthorityRouteFamily,
    ForgeQuerySignalCompatibilityPosture,
};
use hadwiger_research::facade::*;

#[test]
fn relational_declaration_families_encode_phase1_authority_contracts() {
    assert_relational_contract::<CandidateGraphDeclarationFamily>("hadwiger.candidate_graph");
    assert_relational_contract::<EmbeddingDeclarationFamily>("hadwiger.embedding");
    assert_relational_contract::<ColorabilityDeclarationFamily>("hadwiger.colorability");
    assert_relational_contract::<LowerBoundWitnessDeclarationFamily>(
        "hadwiger.lower_bound_witness",
    );
}

#[test]
fn descriptive_declaration_families_encode_deferred_support_contracts() {
    assert_descriptive_bound_support_contract::<AdvisoryNoteDeclarationFamily>(
        "hadwiger.advisory_note",
    );
    assert_descriptive_contract::<RejectionExplanationDeclarationFamily>(
        "hadwiger.rejection_explanation",
    );
    assert_descriptive_contract::<PartialAdmissionExplanationDeclarationFamily>(
        "hadwiger.partial_admission_explanation",
    );
}

fn assert_descriptive_bound_support_contract<F>(family_key: &'static str)
where
    F: ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        ForgeQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        ForgeQueryDeclarationLegalityClass::DescriptiveDeferredSupport
    );
    assert_eq!(
        F::route_contract().allowed_route_families(),
        &[ForgeQueryLowerAuthorityRouteFamily::Relational]
    );
    assert!(!F::route_contract().can_defer());
    assert!(F::relational_truth_contract().is_none());
}

fn assert_relational_contract<F>(family_key: &'static str)
where
    F: ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth
    );
    assert_eq!(
        taxonomy.signal_compatibility(),
        ForgeQuerySignalCompatibilityPosture::NotCompatible
    );
    assert_eq!(
        taxonomy.grouped_posture(),
        ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact
    );
    assert_eq!(
        F::route_contract().allowed_route_families(),
        &[ForgeQueryLowerAuthorityRouteFamily::Relational]
    );
    assert!(!F::route_contract().can_defer());
    assert_eq!(
        F::relational_truth_contract()
            .expect("relational family should declare truth contract")
            .truth_claim(),
        ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth
    );
}

fn assert_descriptive_contract<F>(family_key: &'static str)
where
    F: ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        ForgeQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
    );
    assert_eq!(
        taxonomy.signal_compatibility(),
        ForgeQuerySignalCompatibilityPosture::NotCompatible
    );
    assert_eq!(
        taxonomy.grouped_posture(),
        ForgeQueryGroupedDeclarationPosture::SingleOnly
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        ForgeQueryDeclarationLegalityClass::DescriptiveDeferredSupport
    );
    assert!(F::route_contract().allowed_route_families().is_empty());
    assert!(F::route_contract().can_defer());
    assert!(F::relational_truth_contract().is_none());
}

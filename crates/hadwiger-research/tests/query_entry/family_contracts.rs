use hadwiger_research::facade::*;
use worth_query::facade::{
    WORTHQueryDeclarationFamilyMarker, WORTHQueryDeclarationLegalityClass,
    WORTHQueryDeclarationPrimaryAuthorityFamily, WORTHQueryDeclarationRelationalTruthClaim,
    WORTHQueryGroupedDeclarationPosture, WORTHQueryLowerAuthorityRouteFamily,
    WORTHQuerySignalCompatibilityPosture,
};

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
    F: WORTHQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        WORTHQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        WORTHQueryDeclarationLegalityClass::DescriptiveDeferredSupport
    );
    assert_eq!(
        F::route_contract().allowed_route_families(),
        &[WORTHQueryLowerAuthorityRouteFamily::Relational]
    );
    assert!(!F::route_contract().can_defer());
    assert!(F::relational_truth_contract().is_none());
}

fn assert_relational_contract<F>(family_key: &'static str)
where
    F: WORTHQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        WORTHQueryDeclarationPrimaryAuthorityFamily::RelationalTruth
    );
    assert_eq!(
        taxonomy.signal_compatibility(),
        WORTHQuerySignalCompatibilityPosture::NotCompatible
    );
    assert_eq!(
        taxonomy.grouped_posture(),
        WORTHQueryGroupedDeclarationPosture::NeighborhoodCapable
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        WORTHQueryDeclarationLegalityClass::AuthoritativeHotArtifact
    );
    assert_eq!(
        F::route_contract().allowed_route_families(),
        &[WORTHQueryLowerAuthorityRouteFamily::Relational]
    );
    assert!(!F::route_contract().can_defer());
    assert_eq!(
        F::relational_truth_contract()
            .expect("relational family should declare truth contract")
            .truth_claim(),
        WORTHQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth
    );
}

fn assert_descriptive_contract<F>(family_key: &'static str)
where
    F: WORTHQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        WORTHQueryDeclarationPrimaryAuthorityFamily::DescriptiveOnly
    );
    assert_eq!(
        taxonomy.signal_compatibility(),
        WORTHQuerySignalCompatibilityPosture::NotCompatible
    );
    assert_eq!(
        taxonomy.grouped_posture(),
        WORTHQueryGroupedDeclarationPosture::SingleOnly
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        WORTHQueryDeclarationLegalityClass::DescriptiveDeferredSupport
    );
    assert!(F::route_contract().allowed_route_families().is_empty());
    assert!(F::route_contract().can_defer());
    assert!(F::relational_truth_contract().is_none());
}

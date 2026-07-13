use hadwiger_research::facade::*;
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryCrossingSurface, WorthQueryDeclaredFamilyChecked,
    WorthQueryDomainOperatingContext, WorthQueryOrdinaryOutcome,
};

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("default Hadwiger research handle should admit")
}

#[test]
fn handle_admission_helper_matches_direct_query_path() {
    let context = HadwigerResearchOperatingContext::finite_lower_bound_real();
    let helper = admit_hadwiger_research_handle(context)
        .expect("helper admission should use the runtime-backed Query path");
    let direct = WorthQueryApplicationFacade::runtime_backed_default()
        .domain(HadwigerResearchDomainEntry)
        .with_operating_context(context)
        .validate()
        .expect("direct Query validation should admit")
        .admit()
        .expect("direct Query handle should admit");

    assert_eq!(helper.domain_key(), direct.domain_key());
    assert_eq!(helper.display_name(), direct.display_name());
    assert_eq!(
        helper.handle_identity_digest(),
        direct.handle_identity_digest()
    );
    assert_eq!(
        helper.operating_context_identity_digest(),
        direct.operating_context_identity_digest()
    );
    assert!(!helper.required_capability_families().is_empty());
    assert_eq!(
        helper.required_capability_families(),
        &[
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::HistoricalEvaluation,
        ]
    );
    assert_eq!(
        helper.required_config_sections(),
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational
        ]
    );
    assert_eq!(
        helper.required_config_sections(),
        direct.required_config_sections()
    );
}

#[test]
fn equivalent_candidate_payloads_converge_and_version_changes_digest() {
    let handle = handle();
    let left = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    );
    let same = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    );
    let changed = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v2"),
    );

    let left = admitted_declaration(left);
    let same = admitted_declaration(same);
    let changed = admitted_declaration(changed);

    assert_eq!(left.declaration_digest(), same.declaration_digest());
    assert_ne!(left.declaration_digest(), changed.declaration_digest());
}

#[test]
fn helper_declaration_path_is_thin_over_direct_query_declaration() {
    let handle = handle();
    let request = CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1");
    let helper = declare_research_request_checked(&handle, request.clone());
    let direct = match handle.declare_checked(request) {
        WorthQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("direct Query declaration should admit"),
    };

    let helper = admitted_declaration(helper);

    assert_eq!(helper.declaration_digest(), direct.declaration_digest());
}

#[test]
fn orchestration_helper_returns_query_ordinary_outcome() {
    let handle = handle();
    let outcome = orchestrate_research_request_entry(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    );

    match outcome {
        WorthQueryOrdinaryOutcome::Bound(envelope) => {
            assert_eq!(
                envelope.declaration_family_key(),
                "hadwiger.candidate_graph"
            );
        }
        _ => panic!("expected bound Query ordinary outcome"),
    }
}

#[test]
fn inventory_and_readiness_are_query_seam_projections() {
    let handle = handle();

    let candidate_inventory =
        research_declaration_entry_inventory::<CandidateGraphDeclaration>(&handle);
    let witness_inventory =
        research_declaration_entry_inventory::<LowerBoundWitnessDeclaration>(&handle);
    let advisory_inventory =
        research_declaration_entry_inventory::<AdvisoryNoteDeclaration>(&handle);
    let partial_readiness =
        research_declaration_entry_readiness::<PartialAdmissionExplanationDeclaration>(&handle);

    assert!(!candidate_inventory.rows().is_empty());
    assert!(!witness_inventory.rows().is_empty());
    assert!(!advisory_inventory.rows().is_empty());
    assert!(!partial_readiness.rows().is_empty());
    assert!(candidate_inventory.rows().iter().any(|row| {
        row.surface() == WorthQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
            && row.relational_truth_claim().is_some()
    }));
    assert!(advisory_inventory.rows().iter().any(|row| {
        row.surface() == WorthQueryDeclarationEntryCrossingSurface::Envelope
            && row.envelope_class().is_some()
    }));
}

#[test]
fn operating_context_digest_includes_all_stable_regimes() {
    let context = HadwigerResearchOperatingContext::finite_lower_bound_real();
    let digest = context.context_identity_digest();

    assert!(digest.contains("finite_lower_bound_search"));
    assert!(digest.contains("real_in_process"));
    assert!(digest.contains("conservative"));
}

fn admitted_declaration<I>(
    checked: WorthQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>,
) -> worth_query::facade::foundation::WorthQueryCanonicalDeclarationArtifact<
    HadwigerResearchDomainEntry,
    I,
>
where
    I: HadwigerResearchDeclarationInput,
{
    match checked {
        WorthQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("Hadwiger declaration should admit"),
    }
}

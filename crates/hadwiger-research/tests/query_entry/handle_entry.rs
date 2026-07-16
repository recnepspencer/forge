use hadwiger_research::facade::*;
use worth_query::facade::foundation::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryCrossingSurface, WorthQueryDeclaredFamilyChecked,
    WorthQueryDomainOperatingContext,
};

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("default Hadwiger research handle should admit")
}

#[test]
fn installed_package_drives_the_domain_declaration_context() {
    let installed = handle();

    assert_eq!(installed.domain_key(), "WORTH.hadwiger.research");
    assert_eq!(installed.display_name(), "HadwigerResearchDomainEntry");
    assert_eq!(
        installed.required_capability_families(),
        &[
            WorthQueryCapabilityFamily::QueryRead,
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::HistoricalEvaluation,
        ]
    );
    assert_eq!(
        installed.required_config_sections(),
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational
        ]
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
fn declaration_helper_returns_query_owned_declaration() {
    let handle = handle();
    let declaration = admitted_declaration(declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    ));

    assert_eq!(
        declaration.declaration_family_key(),
        "hadwiger.candidate_graph"
    );
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
fn operating_context_identity_declares_all_stable_regimes() {
    let context = HadwigerResearchOperatingContext::finite_lower_bound_real();
    let identity = context.context_identity();
    let fields = identity.fields().collect::<Vec<_>>();

    assert!(fields.contains(&("assumption_regime", "finite_lower_bound_search")));
    assert!(fields.contains(&("checker_support_regime", "real_in_process")));
    assert!(fields.contains(&("invalidation_regime", "conservative")));
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

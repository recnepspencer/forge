use std::marker::PhantomData;

use forge_proof::{ProofOutcomeKind, RecipeStageKind};

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationProgressionChecked, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDescriptiveOnlyAuthority, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySignalDeferredPosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.progression"
    }

    fn display_name(&self) -> &'static str {
        "GeometryProgressionDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CollaborativeWorld {
    regime: &'static str,
}

impl CollaborativeWorld {
    fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.{}", self.regime)
    }
}

macro_rules! declare_family {
    ($name:ident, $authority:ty, $signal:ty, $grouped:ty, $legality:expr, $progression:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = $grouped;

            fn semantic_family_key() -> &'static str {
                "split-edge"
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                $legality
            }

            fn progression_contract(
                _handle_identity_digest: &str,
                _operating_context_identity_digest: &str,
            ) -> ForgeQueryDeclarationProgressionContract {
                $progression
            }
        }
    };
}

declare_family!(
    AdmittedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    ReceiptFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
declare_family!(
    DeferredFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::deferred_support()
);
declare_family!(
    DeniedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::denied_boundary()
);
declare_family!(
    StaleFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::stale_readable()
);
declare_family!(
    FailedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
    ForgeQueryDeclarationProgressionContract::failed_transition()
);
declare_family!(
    DescriptiveDeferredSignalFamily,
    ForgeQueryDescriptiveOnlyAuthority,
    ForgeQuerySignalDeferredPosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationLegalityContract::receipt_hot_boundary(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorldSensitiveFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for WorldSensitiveFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        if operating_context_identity_digest.contains("restricted") {
            ForgeQueryDeclarationProgressionContract::rebind_required()
        } else {
            ForgeQueryDeclarationProgressionContract::admitted_current()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Declaration<F> {
    edge_ref: &'static str,
    _family: PhantomData<F>,
}

impl<F> Declaration<F> {
    fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _family: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    AdmittedFamily,
    ReceiptFamily,
    DeferredFamily,
    DeniedFamily,
    StaleFamily,
    FailedFamily,
    DescriptiveDeferredSignalFamily,
    WorldSensitiveFamily,
);

fn admitted_handle(
    regime: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, CollaborativeWorld>
{
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld::named(regime))
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}

fn legal<F>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
    declaration: Declaration<F>,
) -> crate::application::ForgeQueryDeclarationLegalityEvidence<GeometryDomain, Declaration<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    Declaration<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_and_review(declaration)
        .unwrap_or_else(|_| panic!("legality review should pass"))
}

fn progressed<F>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
    declaration: Declaration<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, Declaration<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    Declaration<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("progression should admit"))
}

#[test]
fn admitted_progression_yields_proof_bearing_artifact() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .progress_declaration(legal(
            &handle,
            Declaration::<AdmittedFamily>::new("edge:42"),
        ))
        .unwrap_or_else(|_| panic!("progression should admit"));

    assert_eq!(progressed.declaration_family_key(), "split-edge");
    assert_eq!(progressed.outcome().kind(), ProofOutcomeKind::Success);
    assert_eq!(progressed.stage(), RecipeStageKind::Admitted);
    assert_eq!(
        progressed.legality_contract(),
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    );
}

#[test]
fn recipe_lane_matches_convenience_lane() {
    let handle = admitted_handle("collaborative");
    let legal = legal(&handle, Declaration::<AdmittedFamily>::new("edge:42"));
    let recipe = handle.declaration_progression_recipe(legal);
    assert_eq!(recipe.stage(), RecipeStageKind::Unresolved);

    let progressed_from_recipe = handle
        .progress_declaration_recipe(recipe)
        .unwrap_or_else(|_| panic!("recipe progression should admit"));
    let progressed_from_convenience =
        progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));

    assert_eq!(
        progressed_from_recipe.progression_digest(),
        progressed_from_convenience.progression_digest()
    );
}

#[test]
fn progression_exposes_deferred_denied_and_failed_outcomes() {
    let handle = admitted_handle("collaborative");

    match handle.progress_declaration_checked(legal(
        &handle,
        Declaration::<DeferredFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Deferred);
        }
        _ => panic!("expected deferred progression"),
    }

    match handle
        .progress_declaration_checked(legal(&handle, Declaration::<DeniedFamily>::new("edge:42")))
    {
        ForgeQueryDeclarationProgressionChecked::Denied(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Denied);
        }
        _ => panic!("expected denied progression"),
    }

    match handle
        .progress_declaration_checked(legal(&handle, Declaration::<FailedFamily>::new("edge:42")))
    {
        ForgeQueryDeclarationProgressionChecked::Failed(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Failed);
        }
        _ => panic!("expected failed progression"),
    }
}

#[test]
fn checked_recipe_lane_preserves_non_success_outcomes() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    match collaborative.progress_declaration_recipe_checked(
        collaborative.declaration_progression_recipe(legal(
            &collaborative,
            Declaration::<DeferredFamily>::new("edge:42"),
        )),
    ) {
        ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Deferred);
        }
        _ => panic!("expected deferred recipe progression"),
    }

    match collaborative.progress_declaration_recipe_checked(
        collaborative.declaration_progression_recipe(legal(
            &collaborative,
            Declaration::<StaleFamily>::new("edge:42"),
        )),
    ) {
        ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Stale);
            assert_eq!(progress.stage(), RecipeStageKind::Lowered);
        }
        _ => panic!("expected stale recipe progression"),
    }

    match restricted.progress_declaration_recipe_checked(restricted.declaration_progression_recipe(
        legal(
            &restricted,
            Declaration::<WorldSensitiveFamily>::new("edge:42"),
        ),
    )) {
        ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::RebindRequired);
            assert_eq!(progress.stage(), RecipeStageKind::Resolved);
        }
        _ => panic!("expected rebind-required recipe progression"),
    }
}

#[test]
fn progression_preserves_stale_and_rebind_required_separately() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    match collaborative.progress_declaration_checked(legal(
        &collaborative,
        Declaration::<StaleFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Stale);
            assert_eq!(progress.stage(), RecipeStageKind::Lowered);
        }
        _ => panic!("expected stale progression"),
    }

    let collaborative_world_sensitive = legal(
        &collaborative,
        Declaration::<WorldSensitiveFamily>::new("edge:42"),
    );
    assert!(matches!(
        collaborative.progress_declaration_checked(collaborative_world_sensitive),
        ForgeQueryDeclarationProgressionChecked::Admitted(_)
    ));

    match restricted.progress_declaration_checked(legal(
        &restricted,
        Declaration::<WorldSensitiveFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::RebindRequired);
            assert_eq!(progress.stage(), RecipeStageKind::Resolved);
        }
        _ => panic!("expected rebind-required progression"),
    }
}

#[test]
fn descriptive_signal_deferred_families_can_still_progress() {
    let handle = admitted_handle("collaborative");
    let progressed = progressed(
        &handle,
        Declaration::<DescriptiveDeferredSignalFamily>::new("edge:42"),
    );

    assert_eq!(progressed.outcome().kind(), ProofOutcomeKind::Success);
}

#[test]
fn progression_digest_changes_when_legality_truth_changes() {
    let handle = admitted_handle("collaborative");
    let admitted = progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));
    let receipt = progressed(&handle, Declaration::<ReceiptFamily>::new("edge:42"));

    assert_ne!(admitted.progression_digest(), receipt.progression_digest());
}

use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

pub(super) fn core_phase_surface_rows() -> &'static [Row] {
    CORE_PHASE_ROWS
}

#[rustfmt::skip]
const CORE_PHASE_ROWS: &[Row] = &[
    Row::method("src/domain_installation/installed_authority/capabilities/mutation.rs", "WorthQueryInstalledDomainMutationDeclaration", "using", Family::DomainExtension, Phase::Refine, Class::OrdinaryDeclaration, Class::OrdinaryDeclaration, "installed-domain consumer", "installed domain mutation request"),
    Row::method("src/domain_installation/installed_authority/capabilities/mutation.rs", "WorthQueryInstalledDomainMutationRequest", "run", Family::DomainExtension, Phase::Execute, Class::OrdinaryDeclaration, Class::OrdinaryDeclaration, "installed-domain consumer", "installed domain mutation outcome"),
    authoring_method("canonicalize_detail"),
    authoring_method("canonicalize_detail_with_bindings"),
    authoring_method("canonicalize_collection"),
    authoring_method("canonicalize_collection_with_bindings"),
    mechanism("src/binding/runtime.rs", "resolve_bindings", Family::GeneralDeclaration, Phase::Bind, "declarative context admission"),
    mechanism_method("src/canonicalization/pipeline.rs", "QueryCanonicalizer", "canonicalize_request", Family::GeneralDeclaration, Phase::Canonicalize, "ordinary declaration lowering"),
    mechanism("src/canonicalization/pipeline.rs", "canonicalize_request", Family::GeneralDeclaration, Phase::Canonicalize, "ordinary declaration lowering"),
    mechanism("src/validation/pipeline.rs", "validate_canonical_bundle", Family::GeneralDeclaration, Phase::Validate, "Query-owned declaration progression"),
    mechanism("src/planning/mod.rs", "plan_validated_bundle", Family::Read, Phase::Plan, "admitted read execution"),
    mechanism("src/planning/mod.rs", "plan_validated_bundle_for_collection_family", Family::Read, Phase::Plan, "admitted read execution"),
    certification("src/planning/mod.rs", "lower_preflight_to_parallel_admission_route", Phase::Lower),
    certification("src/planning/mod.rs", "lower_preflight_to_serial_fallback_route", Phase::Lower),
    certification("src/planning/mod.rs", "lower_preflight_bundle_to_parallel_admission_routes", Phase::Lower),
    certification("src/planning/mod.rs", "lower_preflight_bundle_to_serial_fallback_routes", Phase::Lower),
    certification("src/planning/mod.rs", "admit_ordered_collection_frontier_preflight", Phase::Admit),
    certification("src/planning/mod.rs", "admit_bounded_materialization_frontier_preflight", Phase::Admit),
    mechanism("src/execution/preflight.rs", "execute_preflight_bundle", Family::Read, Phase::Execute, "admitted read execution"),
    mechanism("src/execution/preflight.rs", "execute_parallel_admission_route", Family::Read, Phase::Execute, "Query-owned route execution"),
    mechanism("src/execution/preflight.rs", "execute_serial_fallback_route", Family::Read, Phase::Execute, "Query-owned route execution"),
    live("src/live/region_scoped.rs", "admit_region_scoped_live_plan", Phase::Admit),
    live("src/live/region_scoped.rs", "execute_region_scoped_live_change", Phase::Execute),
    live("src/live/region_scoped.rs", "lower_region_scoped_execution_to_stream_contract", Phase::Lower),
    live("src/live/mod.rs", "execute_live_change", Phase::Execute),
];

const fn authoring_method(function: &'static str) -> Row {
    mechanism_method(
        "src/authoring/request/guided_path.rs",
        "GuidedAuthoringPath",
        function,
        Family::GeneralDeclaration,
        Phase::Canonicalize,
        "ordinary declaration lowering",
    )
}

const fn live(source: &'static str, function: &'static str, phase: Phase) -> Row {
    mechanism(
        source,
        function,
        Family::Live,
        phase,
        "managed live declaration",
    )
}

const fn certification(source: &'static str, function: &'static str, phase: Phase) -> Row {
    Row::new(
        source,
        function,
        Family::Read,
        phase,
        Class::Certification,
        Class::InternalMechanism,
        "Query certification",
        "internal phase-chain oracle",
    )
}

const fn mechanism(
    source: &'static str,
    function: &'static str,
    family: Family,
    phase: Phase,
    replacement: &'static str,
) -> Row {
    Row::new(
        source,
        function,
        family,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "advanced integration or Query implementation",
        replacement,
    )
}

const fn mechanism_method(
    source: &'static str,
    owner: &'static str,
    function: &'static str,
    family: Family,
    phase: Phase,
    replacement: &'static str,
) -> Row {
    Row::method(
        source,
        owner,
        function,
        family,
        phase,
        Class::Compatibility,
        Class::InternalMechanism,
        "advanced integration or Query implementation",
        replacement,
    )
}

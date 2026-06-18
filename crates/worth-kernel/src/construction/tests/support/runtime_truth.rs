use crate::construction::admitted_scaffold::{
    prepare_primitive_construction_birth_placement_facts, PrimitiveConstructionBirthPlacementFacts,
};
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::outcome::{
    prepare_primitive_construction_rejected_facts, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};
use crate::construction::request::PrimitiveConstructionRequest;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::result::PrimitiveConstructionResultError;
use crate::construction::tests::support::prepared_result::PreparedPrimitiveConstructionResult;
use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionAdmittedRuntimeTruth {
    family: crate::construction::request::PrimitiveConstructionFamily,
    outcome_digest: String,
    birth_truth_digest: String,
    topology_fact_breadth: usize,
    placement_facts: PrimitiveConstructionBirthPlacementFacts,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    mutation_surface: TopologyConstructionQueryMutationSurface,
    read_surface: TopologyConstructionQueryReadSurface,
    inspection_surface: TopologyConstructionQueryInspectionSurface,
    fact_provenance: TopologyConstructionQueryFactProvenance,
    topology_fact_digest: String,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    feature_conditioning_class: PrimitiveFeatureConditioningClass,
    support_normal_class: PrimitiveSupportNormalClass,
    normalization_disposition: PrimitiveNormalizationDisposition,
}

impl PrimitiveConstructionAdmittedRuntimeTruth {
    pub(crate) fn family(&self) -> crate::construction::request::PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub(crate) fn birth_truth_digest(&self) -> &str {
        &self.birth_truth_digest
    }

    pub(crate) fn topology_fact_breadth(&self) -> usize {
        self.topology_fact_breadth
    }

    pub(crate) fn placement_facts(&self) -> PrimitiveConstructionBirthPlacementFacts {
        self.placement_facts
    }

    pub(crate) fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub(crate) fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.read_surface
    }

    pub(crate) fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.inspection_surface
    }

    pub(crate) fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.fact_provenance
    }

    pub(crate) fn topology_fact_digest(&self) -> &str {
        &self.topology_fact_digest
    }

    pub(crate) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_strategy
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub(crate) fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.feature_conditioning_class
    }

    pub(crate) fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.support_normal_class
    }

    pub(crate) fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.normalization_disposition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionRejectedRuntimeTruth {
    family: crate::construction::request::PrimitiveConstructionFamily,
    outcome_digest: String,
    reason: String,
    rejection_class: PrimitiveConstructionRejectionClass,
    rejection_locality: PrimitiveConstructionRejectionLocality,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
}

impl PrimitiveConstructionRejectedRuntimeTruth {
    pub(crate) fn family(&self) -> crate::construction::request::PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub(crate) fn reason(&self) -> &str {
        &self.reason
    }

    pub(crate) fn rejection_class(&self) -> PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub(crate) fn rejection_locality(&self) -> PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub(crate) fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.feature_conditioning_class
    }

    pub(crate) fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }

    pub(crate) fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PrimitiveConstructionCertificationRuntimeTruth {
    Admitted(PrimitiveConstructionAdmittedRuntimeTruth),
    Rejected(PrimitiveConstructionRejectedRuntimeTruth),
}

impl PrimitiveConstructionCertificationRuntimeTruth {
    pub(crate) fn family(&self) -> crate::construction::request::PrimitiveConstructionFamily {
        match self {
            Self::Admitted(outcome) => outcome.family(),
            Self::Rejected(rejected) => rejected.family(),
        }
    }

    pub(crate) fn outcome_digest(&self) -> &str {
        match self {
            Self::Admitted(outcome) => outcome.outcome_digest(),
            Self::Rejected(rejected) => rejected.outcome_digest(),
        }
    }
}

pub(crate) fn prepare_primitive_construction_certification_runtime_truth(
    request: PrimitiveConstructionRequest,
) -> PrimitiveConstructionCertificationRuntimeTruth {
    let intent: PrimitiveConstructionIntent = request.clone().into();
    let family = intent.family();
    match prepare_primitive_construction_result(intent) {
        Ok(prepared) => PrimitiveConstructionCertificationRuntimeTruth::Admitted(
            admitted_runtime_truth_from_prepared_result(family, &request, &prepared),
        ),
        Err(error) => PrimitiveConstructionCertificationRuntimeTruth::Rejected(
            rejected_runtime_truth_from_error(family, &error),
        ),
    }
}

fn admitted_runtime_truth_from_prepared_result(
    family: crate::construction::request::PrimitiveConstructionFamily,
    request: &PrimitiveConstructionRequest,
    prepared: &PreparedPrimitiveConstructionResult,
) -> PrimitiveConstructionAdmittedRuntimeTruth {
    let topology_query_envelope = prepared.topology_query_handoff().topology_query_envelope();
    PrimitiveConstructionAdmittedRuntimeTruth {
        family,
        outcome_digest: prepared.result_digest().to_string(),
        birth_truth_digest: prepared.birth_truth_digest().to_string(),
        topology_fact_breadth: topology_query_envelope
            .fact_rows()
            .iter()
            .map(|row| row.fact_count())
            .sum(),
        placement_facts: prepare_primitive_construction_birth_placement_facts(request)
            .expect("prepared result should retain placement facts"),
        required_query_families: topology_query_envelope.required_query_families().to_vec(),
        mutation_surface: prepared.mutation_surface(),
        read_surface: prepared.read_surface(),
        inspection_surface: topology_query_envelope.inspection_surface(),
        fact_provenance: prepared.fact_provenance(),
        topology_fact_digest: prepared.topology_fact_digest().to_string(),
        realization_strategy: prepared.realization_strategy(),
        attempted_realization_strategies: prepared.attempted_realization_strategies().to_vec(),
        stability_class: prepared.stability_class(),
        feature_conditioning_class: prepared.feature_conditioning_class(),
        support_normal_class: prepared.support_normal_class(),
        normalization_disposition: prepared.normalization_disposition(),
    }
}

fn rejected_runtime_truth_from_error(
    family: crate::construction::request::PrimitiveConstructionFamily,
    error: &PrimitiveConstructionResultError,
) -> PrimitiveConstructionRejectedRuntimeTruth {
    let rejection = prepare_primitive_construction_rejected_facts(family, error);
    PrimitiveConstructionRejectedRuntimeTruth {
        family,
        outcome_digest: rejection.failure_digest().to_string(),
        reason: rejection.reason().to_string(),
        rejection_class: rejection.rejection_class(),
        rejection_locality: rejection.rejection_locality(),
        attempted_realization_strategies: rejection.attempted_realization_strategies().to_vec(),
        stability_class: rejection.stability_class(),
        feature_conditioning_class: rejection.feature_conditioning_class(),
        support_normal_class: rejection.support_normal_class(),
        normalization_disposition: rejection.normalization_disposition(),
        exhaustion_reason: rejection.exhaustion_reason(),
    }
}

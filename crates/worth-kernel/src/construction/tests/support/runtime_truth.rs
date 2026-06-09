use crate::construction::admitted_scaffold::{
    prepare_primitive_construction_birth_placement_facts,
    prepare_primitive_construction_topology_query_admitted_handoff_from_request,
    PrimitiveConstructionBirthPlacementFacts,
};
use crate::construction::outcome::{
    prepare_primitive_construction_rejected_facts, PrimitiveConstructionRejectionClass,
    PrimitiveConstructionRejectionLocality,
};
use crate::construction::realization_snapshot::prepare_realization_snapshot;
use crate::construction::request::{
    primitive_construction_invalid_request_reason, PrimitiveConstructionRequest,
};
use crate::construction::result::PrimitiveConstructionResultError;
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
    if let Some(reason) = primitive_construction_invalid_request_reason(&request) {
        return PrimitiveConstructionCertificationRuntimeTruth::Rejected(
            PrimitiveConstructionRejectedRuntimeTruth {
                family: request.family(),
                outcome_digest: format!("invalid:{}:{reason}", request.family().as_str()),
                reason: format!("invalid {} request: {reason}", request.family().as_str()),
                rejection_class: PrimitiveConstructionRejectionClass::InvalidRequest,
                rejection_locality: PrimitiveConstructionRejectionLocality::Admission,
                attempted_realization_strategies: Vec::new(),
                stability_class: None,
                feature_conditioning_class: None,
                support_normal_class: None,
                normalization_disposition: None,
                exhaustion_reason: None,
            },
        );
    }

    let realization_snapshot = prepare_realization_snapshot(request.clone());
    match prepare_primitive_construction_topology_query_admitted_handoff_from_request(&request) {
        Ok(topology_query_admitted_handoff) => {
            let topology_query_envelope = topology_query_admitted_handoff
                .topology_query_handoff()
                .topology_query_envelope();
            let conditioning_witness = realization_snapshot
                .conditioning_witness()
                .expect("admitted realization snapshot should retain conditioning witness");
            let realization_strategy = realization_snapshot
                .selected_strategy()
                .expect("admitted realization snapshot should retain selected strategy");
            let stability_class = realization_snapshot
                .stability_class()
                .expect("admitted realization snapshot should retain stability class");
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(
                PrimitiveConstructionAdmittedRuntimeTruth {
                    family: request.family(),
                    outcome_digest: topology_query_admitted_handoff
                        .admitted_handoff_digest()
                        .to_string(),
                    birth_truth_digest: topology_query_envelope.source_birth_digest().to_string(),
                    topology_fact_breadth: topology_query_envelope
                        .fact_rows()
                        .iter()
                        .map(|row| row.fact_count())
                        .sum(),
                    placement_facts: prepare_primitive_construction_birth_placement_facts(&request)
                        .expect(
                            "admitted construction runtime truth should retain placement facts",
                        ),
                    required_query_families: topology_query_envelope
                        .required_query_families()
                        .to_vec(),
                    mutation_surface: topology_query_envelope.mutation_surface(),
                    read_surface: topology_query_envelope.read_surface(),
                    inspection_surface: topology_query_envelope.inspection_surface(),
                    fact_provenance: topology_query_envelope.fact_provenance(),
                    topology_fact_digest: topology_query_envelope.fact_digest().to_string(),
                    realization_strategy,
                    attempted_realization_strategies: realization_snapshot
                        .attempted_strategies()
                        .to_vec(),
                    stability_class,
                    feature_conditioning_class: conditioning_witness.feature_conditioning_class(),
                    support_normal_class: conditioning_witness.support_normal_class(),
                    normalization_disposition: conditioning_witness.normalization_disposition(),
                },
            )
        }
        Err(error) => {
            let rejection = prepare_primitive_construction_rejected_facts(
                request.family(),
                &PrimitiveConstructionResultError::Phase(error),
            );
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(
                PrimitiveConstructionRejectedRuntimeTruth {
                    family: request.family(),
                    outcome_digest: rejection.failure_digest().to_string(),
                    reason: rejection.reason().to_string(),
                    rejection_class: rejection.rejection_class(),
                    rejection_locality: rejection.rejection_locality(),
                    attempted_realization_strategies: rejection
                        .attempted_realization_strategies()
                        .to_vec(),
                    stability_class: rejection.stability_class(),
                    feature_conditioning_class: rejection.feature_conditioning_class(),
                    support_normal_class: rejection.support_normal_class(),
                    normalization_disposition: rejection.normalization_disposition(),
                    exhaustion_reason: rejection.exhaustion_reason(),
                },
            )
        }
    }
}

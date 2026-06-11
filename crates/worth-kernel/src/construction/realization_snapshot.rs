use super::admitted_scaffold::prepare_primitive_construction_admitted_realization_posture;
#[cfg(test)]
use super::admitted_scaffold::prepare_primitive_construction_topology_query_admitted_handoff_from_request;
use super::outcome::prepare_primitive_construction_rejected_facts;
use super::request::{
    primitive_construction_invalid_request_reason, PrimitiveConstructionFamily,
    PrimitiveConstructionRequest,
};
use super::result::PrimitiveConstructionResultError;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionRealizationSnapshot {
    family: PrimitiveConstructionFamily,
    admitted: bool,
    selected_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    #[cfg(test)]
    canonical_artifact_digest: Option<String>,
    #[cfg(test)]
    realization_digest: String,
}

impl PrimitiveConstructionRealizationSnapshot {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn admitted(&self) -> bool {
        self.admitted
    }

    pub(crate) fn selected_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.selected_strategy
    }

    pub(crate) fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub(crate) fn conditioning_witness(&self) -> Option<&PrimitiveConditioningWitness> {
        self.conditioning_witness.as_ref()
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    #[cfg(test)]
    pub(crate) fn canonical_artifact_digest(&self) -> Option<&str> {
        self.canonical_artifact_digest.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn realization_digest(&self) -> &str {
        &self.realization_digest
    }
}

pub(crate) fn prepare_realization_snapshot(
    request: PrimitiveConstructionRequest,
) -> PrimitiveConstructionRealizationSnapshot {
    if primitive_construction_invalid_request_reason(&request).is_some() {
        return rejected_snapshot(
            request.family(),
            Vec::new(),
            None,
            None,
            None,
            #[cfg(test)]
            request.request_digest().to_string(),
        );
    }
    match prepare_primitive_construction_admitted_realization_posture(&request) {
        Ok(realization_posture) => accepted_snapshot(&request, realization_posture),
        Err(error) => {
            let rejection = prepare_primitive_construction_rejected_facts(
                request.family(),
                &PrimitiveConstructionResultError::Phase(error),
            );
            rejected_snapshot(
                request.family(),
                rejection.attempted_realization_strategies().to_vec(),
                rejection.conditioning_witness().cloned(),
                rejection.stability_class(),
                rejection.exhaustion_reason(),
                #[cfg(test)]
                rejection
                    .exhaustion_fact_digest()
                    .unwrap_or(rejection.failure_digest())
                    .to_string(),
            )
        }
    }
}

fn accepted_snapshot(
    request: &PrimitiveConstructionRequest,
    realization_posture: super::admitted_scaffold::PrimitiveConstructionAdmittedRealizationPosture,
) -> PrimitiveConstructionRealizationSnapshot {
    #[cfg(test)]
    let canonical_artifact_digest =
        prepare_primitive_construction_topology_query_admitted_handoff_from_request(request)
            .ok()
            .map(
                |handoff: topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff| {
                    handoff.admitted_handoff_digest().to_string()
                },
            );
    PrimitiveConstructionRealizationSnapshot {
        family: request.family(),
        admitted: true,
        selected_strategy: Some(realization_posture.selected_strategy()),
        attempted_strategies: realization_posture.attempted_strategies().to_vec(),
        conditioning_witness: Some(realization_posture.conditioning_witness().clone()),
        stability_class: Some(realization_posture.stability_class()),
        exhaustion_reason: None,
        #[cfg(test)]
        canonical_artifact_digest,
        #[cfg(test)]
        realization_digest: realization_posture.realization_digest().to_string(),
    }
}

fn rejected_snapshot(
    family: PrimitiveConstructionFamily,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    conditioning_witness: Option<PrimitiveConditioningWitness>,
    stability_class: Option<PrimitiveStabilityClass>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    #[cfg(test)] realization_digest: String,
) -> PrimitiveConstructionRealizationSnapshot {
    PrimitiveConstructionRealizationSnapshot {
        family,
        admitted: false,
        selected_strategy: None,
        attempted_strategies,
        conditioning_witness,
        stability_class,
        exhaustion_reason,
        #[cfg(test)]
        canonical_artifact_digest: None,
        #[cfg(test)]
        realization_digest,
    }
}

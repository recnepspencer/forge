use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};
use topology::facade::TopologyConstructionMutationSurface;

use crate::construction::artifact::build_canonical_primitive_construction_artifact;
use crate::construction::digest::digest_owned_parts;
use crate::construction::execution::{
    PreparedPrimitiveConstructionExecution, PrimitiveConstructionExecutionError,
};
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::lower_scaffold_to_topology;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryGraphCompositionParityReport {
    family: PrimitiveConstructionFamily,
    query_contract_digest: String,
    lowering_surface: TopologyConstructionMutationSurface,
    artifact_surface: TopologyConstructionMutationSurface,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryGraphCompositionParityReport {
    fn new(
        family: PrimitiveConstructionFamily,
        query_contract_digest: String,
        lowering_surface: TopologyConstructionMutationSurface,
        artifact_surface: TopologyConstructionMutationSurface,
        required_query_families: &[ForgeQueryRuntimeFacadeFamily],
    ) -> Self {
        let parity_verified = lowering_surface == TopologyConstructionMutationSurface::ComposeGraph
            && artifact_surface == TopologyConstructionMutationSurface::ComposeGraph
            && required_query_families == [ForgeQueryRuntimeFacadeFamily::Write];
        let report_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            query_contract_digest.clone(),
            lowering_surface.as_str().to_string(),
            artifact_surface.as_str().to_string(),
            required_query_families
                .iter()
                .map(|family| format!("{family:?}"))
                .collect::<Vec<_>>()
                .join("|"),
            parity_verified.to_string(),
        ]);
        Self {
            family,
            query_contract_digest,
            lowering_surface,
            artifact_surface,
            required_query_families: required_query_families.to_vec(),
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn lowering_surface(&self) -> TopologyConstructionMutationSurface {
        self.lowering_surface
    }

    pub fn query_contract_digest(&self) -> &str {
        &self.query_contract_digest
    }

    pub fn artifact_surface(&self) -> TopologyConstructionMutationSurface {
        self.artifact_surface
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryGraphCompositionParityError {
    Phase(PrimitiveConstructionPhaseError),
    Execution(PrimitiveConstructionExecutionError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryGraphCompositionParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(error) => write!(f, "{error}"),
            Self::Execution(error) => write!(f, "{error}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryGraphCompositionParityError {}

pub fn prepare_primitive_construction_query_graph_composition_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl Into<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionQueryGraphCompositionParityReport,
    PrimitiveConstructionQueryGraphCompositionParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Write)
        .map_err(PrimitiveConstructionQueryGraphCompositionParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let request_for_chain = intent.into().into_request();
    let intent = request_for_chain
        .clone()
        .admit()
        .map_err(PrimitiveConstructionQueryGraphCompositionParityError::Phase)?;
    let scaffold = intent
        .build_scaffold()
        .map_err(PrimitiveConstructionQueryGraphCompositionParityError::Phase)?;
    let (birth_plan, lowering_plan) = lower_scaffold_to_topology(&scaffold)
        .map_err(PrimitiveConstructionQueryGraphCompositionParityError::Phase)?;
    let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
        &request_for_chain,
        &intent,
        &scaffold,
        &birth_plan,
        &lowering_plan,
    )
    .map_err(PrimitiveConstructionQueryGraphCompositionParityError::Execution)?;
    let certification = execution.plan_topology_certification();
    let artifact = build_canonical_primitive_construction_artifact(
        &request_for_chain,
        &intent,
        &scaffold,
        &birth_plan,
        &lowering_plan,
        &execution,
        &certification,
    )
    .expect("graph composition parity should not fail artifact assembly");
    Ok(PrimitiveConstructionQueryGraphCompositionParityReport::new(
        request_for_chain.family(),
        query_contract_digest,
        lowering_plan.mutation_surface(),
        artifact.mutation_surface(),
        execution.execution_plan().required_query_families(),
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_query_graph_composition_parity_report;
    use crate::construction::{
        OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent,
    };
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyConstructionMutationSurface,
        TopologyRuntimeAdapters,
    };

    #[test]
    fn graph_composition_parity_report_locks_compose_graph_as_the_write_surface() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-graph-composition".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_graph_composition_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
        )
        .expect("graph composition parity report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::Orthotope);
        assert_eq!(
            report.lowering_surface(),
            TopologyConstructionMutationSurface::ComposeGraph
        );
        assert_eq!(
            report.artifact_surface(),
            TopologyConstructionMutationSurface::ComposeGraph
        );
        assert_eq!(
            report.required_query_families(),
            &[ForgeQueryRuntimeFacadeFamily::Write]
        );
        assert!(!report.query_contract_digest().is_empty());
        assert!(report.parity_verified());
        assert!(!report.report_digest().is_empty());
    }
}

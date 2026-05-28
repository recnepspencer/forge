use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::construction::execution::TopologyConstructionExecutionPlan;

const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 1] =
    [ForgeQueryRuntimeFacadeFamily::Inspect];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionCertificationReadSurface {
    ProjectionConsumptionFromInspectionReceipt,
}

impl TopologyConstructionCertificationReadSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectionConsumptionFromInspectionReceipt => {
                "projection consumption from inspection receipt"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionInspectionSurface {
    InspectReceipt,
}

impl TopologyConstructionInspectionSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InspectReceipt => "workspace.inspect(&receipt)",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionCertificationPlan {
    source_execution_digest: String,
    read_surface: TopologyConstructionCertificationReadSurface,
    inspection_surface: TopologyConstructionInspectionSurface,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    certification_scope: &'static str,
    certification_digest: String,
}

impl TopologyConstructionCertificationPlan {
    fn new(source_execution_digest: String) -> Self {
        let read_surface =
            TopologyConstructionCertificationReadSurface::ProjectionConsumptionFromInspectionReceipt;
        let inspection_surface = TopologyConstructionInspectionSurface::InspectReceipt;
        let required_query_families = REQUIRED_QUERY_FAMILIES.to_vec();
        let certification_scope = "worth-topo.construction-certification";
        let mut parts = vec![
            source_execution_digest.clone(),
            read_surface.as_str().to_string(),
            inspection_surface.as_str().to_string(),
            certification_scope.to_string(),
        ];
        parts.extend(
            required_query_families
                .iter()
                .map(|family| format!("required-query-family:{family:?}")),
        );
        Self {
            source_execution_digest,
            read_surface,
            inspection_surface,
            required_query_families,
            certification_scope,
            certification_digest: digest_parts(&parts),
        }
    }

    pub fn source_execution_digest(&self) -> &str {
        &self.source_execution_digest
    }

    pub fn read_surface(&self) -> TopologyConstructionCertificationReadSurface {
        self.read_surface
    }

    pub fn inspection_surface(&self) -> TopologyConstructionInspectionSurface {
        self.inspection_surface
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn certification_scope(&self) -> &str {
        self.certification_scope
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }
}

pub fn prepare_primitive_construction_certification(
    execution: &TopologyConstructionExecutionPlan,
) -> TopologyConstructionCertificationPlan {
    TopologyConstructionCertificationPlan::new(execution.execution_digest().to_string())
}

fn digest_parts(parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_certification, TopologyConstructionCertificationReadSurface,
        TopologyConstructionInspectionSurface,
    };
    use crate::construction::execution::TopologyConstructionExecutionPlan;
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

    #[test]
    fn certification_plan_separates_inspection_and_projection_consumption() {
        let execution = TopologyConstructionExecutionPlan::new_for_tests("simplex-lowering");
        let certification = prepare_primitive_construction_certification(&execution);

        assert_eq!(
            certification.read_surface(),
            TopologyConstructionCertificationReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert_eq!(
            certification.inspection_surface(),
            TopologyConstructionInspectionSurface::InspectReceipt
        );
        assert_eq!(
            certification.required_query_families(),
            &[ForgeQueryRuntimeFacadeFamily::Inspect]
        );
        assert!(!certification.certification_digest().is_empty());
    }
}





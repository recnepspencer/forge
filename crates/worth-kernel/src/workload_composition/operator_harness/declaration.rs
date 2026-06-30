use super::query::query_backed_operator_declaration;
use super::run::{BatchAdmissionExecutionOperatorRun, OperatorRun};
use super::support::{OperatorSupportPosture, OperatorSupportReceipt, OperatorWorkloadError};
use crate::workload_composition::{
    BatchAdmissionExecutionReceipt, WorkloadStageRequirement, WorthWorkload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadOperator {
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    query_intent: String,
}

impl WorkloadOperator {
    pub fn for_family(family: WorkloadOperatorFamily) -> Self {
        Self {
            family,
            requirement: WorkloadStageRequirement::EvidenceLedger,
            query_intent: String::new(),
        }
    }

    pub fn requiring(mut self, requirement: WorkloadStageRequirement) -> Self {
        self.requirement = requirement;
        self
    }

    pub fn declared_by_query(mut self, intent: impl Into<String>) -> Self {
        self.query_intent = intent.into();
        self
    }

    pub fn admit_for(self, workload: &WorthWorkload) -> Result<OperatorRun, OperatorWorkloadError> {
        let declaration =
            OperatorDeclarationReceipt::new(self.family, self.requirement, self.query_intent)?;
        let support = OperatorSupportReceipt::for_declaration(&declaration)?;
        if support.posture() != OperatorSupportPosture::Admitted {
            return Err(OperatorWorkloadError::UnsupportedOperatorFamily {
                family: self.family.unsupported_family(),
                support,
            });
        }
        OperatorRun::from_admitted(workload, declaration, support)
    }

    pub fn admit_for_batch_execution(
        self,
        workload: &WorthWorkload,
        batch_execution: &BatchAdmissionExecutionReceipt,
    ) -> Result<BatchAdmissionExecutionOperatorRun, OperatorWorkloadError> {
        let declaration =
            OperatorDeclarationReceipt::new(self.family, self.requirement, self.query_intent)?;
        let support = OperatorSupportReceipt::for_declaration(&declaration)?;
        if support.posture() != OperatorSupportPosture::Admitted {
            return Err(OperatorWorkloadError::UnsupportedOperatorFamily {
                family: self.family.unsupported_family(),
                support,
            });
        }
        OperatorRun::from_batch_execution_admitted(workload, batch_execution, declaration, support)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorDeclarationReceipt {
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    query_intent: String,
    query_declaration_digest: String,
    query_envelope_digest: String,
    query_handle_digest: String,
}

impl OperatorDeclarationReceipt {
    fn new(
        family: WorkloadOperatorFamily,
        requirement: WorkloadStageRequirement,
        query_intent: String,
    ) -> Result<Self, OperatorWorkloadError> {
        if query_intent.trim().is_empty() {
            return Err(OperatorWorkloadError::MissingQueryDeclaration);
        }
        let query_receipt =
            query_backed_operator_declaration(family, requirement, query_intent.trim())
                .map_err(OperatorWorkloadError::QueryAdmissionFailed)?;
        Ok(Self {
            family,
            requirement,
            query_intent: query_intent.trim().to_string(),
            query_declaration_digest: query_receipt.declaration_digest().to_string(),
            query_envelope_digest: query_receipt.envelope_digest().to_string(),
            query_handle_digest: query_receipt.handle_digest().to_string(),
        })
    }

    pub fn family(&self) -> WorkloadOperatorFamily {
        self.family
    }

    pub fn requirement(&self) -> WorkloadStageRequirement {
        self.requirement
    }

    pub fn query_intent(&self) -> &str {
        &self.query_intent
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn query_envelope_digest(&self) -> &str {
        &self.query_envelope_digest
    }

    pub fn query_handle_digest(&self) -> &str {
        &self.query_handle_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadOperatorFamily {
    CoplanarOverlap,
    PlanarBooleanFoundation,
    Unsupported(UnsupportedOperatorFamily),
}

impl WorkloadOperatorFamily {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::CoplanarOverlap => "coplanar overlap workload operator",
            Self::PlanarBooleanFoundation => "planar boolean workload operator foundation",
            Self::Unsupported(family) => family.human_name(),
        }
    }

    pub fn query_key(self) -> &'static str {
        match self {
            Self::CoplanarOverlap => "worth.operator.coplanar_overlap",
            Self::PlanarBooleanFoundation => "worth.operator.planar_boolean_foundation",
            Self::Unsupported(family) => family.query_key(),
        }
    }

    pub fn is_supported(self) -> bool {
        matches!(self, Self::CoplanarOverlap)
    }

    pub fn unsupported_family(self) -> UnsupportedOperatorFamily {
        match self {
            Self::Unsupported(family) => family,
            Self::CoplanarOverlap => UnsupportedOperatorFamily::NotUnsupported,
            Self::PlanarBooleanFoundation => UnsupportedOperatorFamily::PlanarBooleanFoundation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedOperatorFamily {
    PlanarBooleanFoundation,
    BooleanDifference,
    CurvedSurface,
    Extrusion,
    NotUnsupported,
}

impl UnsupportedOperatorFamily {
    pub fn human_name(self) -> &'static str {
        match self {
            Self::PlanarBooleanFoundation => "planar boolean workload operator foundation",
            Self::BooleanDifference => "boolean difference operator",
            Self::CurvedSurface => "curved surface operator",
            Self::Extrusion => "extrusion operator",
            Self::NotUnsupported => "supported operator",
        }
    }

    pub fn query_key(self) -> &'static str {
        match self {
            Self::PlanarBooleanFoundation => "worth.operator.unsupported.planar_boolean_foundation",
            Self::BooleanDifference => "worth.operator.unsupported.boolean_difference",
            Self::CurvedSurface => "worth.operator.unsupported.curved_surface",
            Self::Extrusion => "worth.operator.unsupported.extrusion",
            Self::NotUnsupported => "worth.operator.supported",
        }
    }
}

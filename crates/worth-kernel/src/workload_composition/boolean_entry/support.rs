use super::declaration::{
    PlanarBooleanDeclarationReceipt, PlanarBooleanExecutionLane, PlanarBooleanFamily,
};
use super::query::query_backed_planar_boolean_support;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSupportReceipt {
    family: PlanarBooleanFamily,
    posture: PlanarBooleanSupportPosture,
    query_support_digest: String,
    human_reason: String,
}

impl PlanarBooleanSupportReceipt {
    pub fn for_declaration(
        declaration: &PlanarBooleanDeclarationReceipt,
    ) -> Result<Self, PlanarBooleanEntryError> {
        let posture = PlanarBooleanSupportPosture::for_lane(declaration.requested_lane());
        let human_reason = match posture {
            PlanarBooleanSupportPosture::Admitted => format!(
                "{} is admitted for {} on the B-rep execution lane",
                declaration.family().human_name(),
                declaration.operation().human_name()
            ),
            PlanarBooleanSupportPosture::VisibleNotAdmitted => {
                "EMBER stays visible on the declaration boundary but is not admitted in milestone 7.0"
                    .to_string()
            }
        };
        let query_receipt = query_backed_planar_boolean_support(
            declaration.family(),
            declaration.operation(),
            declaration.requested_lane(),
            posture,
            declaration.query_declaration_digest(),
            declaration.query_intent(),
        )?;
        Ok(Self {
            family: declaration.family(),
            posture,
            query_support_digest: query_receipt.declaration_digest().to_string(),
            human_reason,
        })
    }

    pub fn family(&self) -> PlanarBooleanFamily {
        self.family
    }

    pub fn posture(&self) -> PlanarBooleanSupportPosture {
        self.posture
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSupportPosture {
    Admitted,
    VisibleNotAdmitted,
}

impl PlanarBooleanSupportPosture {
    pub fn for_lane(lane: PlanarBooleanExecutionLane) -> Self {
        match lane {
            PlanarBooleanExecutionLane::BRepNow => Self::Admitted,
            PlanarBooleanExecutionLane::EmberFuture => Self::VisibleNotAdmitted,
        }
    }

    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }

    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::VisibleNotAdmitted => "visible_not_admitted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEntryError {
    MissingQueryDeclaration,
    MissingEntryBasis,
    InvalidOperandPairIdentity,
    OutcomeProjectionMismatch(String),
    QueryAdmissionFailed(String),
}

impl PlanarBooleanEntryError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingQueryDeclaration => {
                "planar boolean declaration requires a human-readable Query intent".to_string()
            }
            Self::MissingEntryBasis => {
                "planar boolean declaration requires a real planar boolean entry basis".to_string()
            }
            Self::InvalidOperandPairIdentity => {
                "planar boolean declaration requires a non-empty operand-pair identity".to_string()
            }
            Self::OutcomeProjectionMismatch(reason) => reason.clone(),
            Self::QueryAdmissionFailed(reason) => {
                format!("planar boolean declaration could not be admitted by Forge Query: {reason}")
            }
        }
    }
}

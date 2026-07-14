use crate::authorized_projection::{
    derive_authorized_projection, AuthorizedProjectionArtifact, AuthorizedProjectionError,
    PolicyAspectMask, PolicyInfluenceSet,
};
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::projection_consumption::{
    ConsumedProjectionAuthorityDenial, DeferredProjectionConsumption, DeniedProjectionConsumption,
    ProjectionAuthorityContract, ProjectionAuthorityOutcome, ProjectionConsumptionDeclarationError,
    ProjectionConsumptionWarnings, ProjectionFactExtractionError, ProjectionFactFieldPath,
    SourceMismatchedProjectionConsumption, WorthQueryConsumedProjectionAuthority,
};
use crate::runtime::{WorthQueryReadGraph, WorthQueryReadResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProjectionDeclaration {
    contract: ProjectionAuthorityContract,
}

/// Begin a typed fact-extraction declaration for a completed ordinary read.
///
/// The declaration always requires settled consumption, source authority, and
/// matching basis generation. Consumers choose facts, not safety posture.
pub fn project_facts() -> WorthQueryProjectionDeclaration {
    WorthQueryProjectionDeclaration {
        contract: ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_basis_generation(),
    }
}

impl WorthQueryProjectionDeclaration {
    pub fn entity_identities(mut self) -> Self {
        self.contract = self.contract.require_entity_identities();
        self
    }

    pub fn view_local_identities(mut self) -> Self {
        self.contract = self.contract.require_view_local_identities();
        self
    }

    pub fn target_identity(mut self) -> Self {
        self.contract = self.contract.require_target_identity();
        self
    }

    pub fn source_references(mut self) -> Self {
        self.contract = self.contract.require_source_references();
        self
    }

    pub fn effect_continuity_facts(mut self) -> Self {
        self.contract = self.contract.require_effect_continuity_facts();
        self
    }

    pub fn memberships(mut self) -> Self {
        self.contract = self.contract.require_memberships();
        self
    }

    pub fn relation_endpoints(mut self) -> Self {
        self.contract = self.contract.require_relation_endpoints();
        self
    }

    pub fn display_field(mut self, field: ProjectionFactFieldPath) -> Self {
        self.contract = self.contract.require_display_field(field);
        self
    }

    pub fn derived_scalar_field(mut self, field: ProjectionFactFieldPath) -> Self {
        self.contract = self.contract.require_derived_scalar_field(field);
        self
    }
}

#[derive(Debug)]
pub struct WorthQueryProjectionAdvisory {
    authority: Box<WorthQueryConsumedProjectionAuthority>,
    warnings: ProjectionConsumptionWarnings,
}

impl WorthQueryProjectionAdvisory {
    pub fn authority(&self) -> &WorthQueryConsumedProjectionAuthority {
        &self.authority
    }

    pub fn warnings(&self) -> &ProjectionConsumptionWarnings {
        &self.warnings
    }
}

#[derive(Debug)]
pub enum WorthQueryProjectionViolation {
    Authority(ConsumedProjectionAuthorityDenial),
    Consumption(DeniedProjectionConsumption),
    SourceMismatch(SourceMismatchedProjectionConsumption),
    Declaration(ProjectionConsumptionDeclarationError),
}

#[derive(Debug)]
pub enum WorthQueryProjectionUnavailable {
    AuthorityBinding(AuthorizedProjectionError),
    Extraction(ProjectionFactExtractionError),
}

#[derive(Debug)]
pub enum WorthQueryProjectionOutcome {
    Completed(Box<WorthQueryConsumedProjectionAuthority>),
    Advisory(WorthQueryProjectionAdvisory),
    Violation(WorthQueryProjectionViolation),
    Deferred(DeferredProjectionConsumption),
    Unavailable(WorthQueryProjectionUnavailable),
}

impl WorthQueryProjectionOutcome {
    pub fn authority(&self) -> Option<&WorthQueryConsumedProjectionAuthority> {
        match self {
            Self::Completed(authority) => Some(authority),
            Self::Advisory(advisory) => Some(advisory.authority()),
            Self::Violation(_) | Self::Deferred(_) | Self::Unavailable(_) => None,
        }
    }

    pub fn advisory(&self) -> Option<&WorthQueryProjectionAdvisory> {
        match self {
            Self::Advisory(advisory) => Some(advisory),
            _ => None,
        }
    }

    pub fn violation(&self) -> Option<&WorthQueryProjectionViolation> {
        match self {
            Self::Violation(violation) => Some(violation),
            _ => None,
        }
    }

    pub fn deferred(&self) -> Option<&DeferredProjectionConsumption> {
        match self {
            Self::Deferred(deferred) => Some(deferred),
            _ => None,
        }
    }

    pub fn unavailable(&self) -> Option<&WorthQueryProjectionUnavailable> {
        match self {
            Self::Unavailable(unavailable) => Some(unavailable),
            _ => None,
        }
    }

    /// Transfer the sealed projection authority into a downstream consumer.
    ///
    /// This preserves advisory warnings without exposing the decomposed
    /// projection-consumption lifecycle or allowing a receipt/digest to be
    /// promoted back into authority.
    pub fn into_admitted(
        self,
    ) -> Result<
        (
            Box<WorthQueryConsumedProjectionAuthority>,
            Option<ProjectionConsumptionWarnings>,
        ),
        Self,
    > {
        match self {
            Self::Completed(authority) => Ok((authority, None)),
            Self::Advisory(advisory) => Ok((advisory.authority, Some(advisory.warnings))),
            other => Err(other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct WorthQueryReadProjectionBinding {
    result_shape: CanonicalResultShapeArtifact,
    authorized_projection: Result<AuthorizedProjectionArtifact, AuthorizedProjectionError>,
}

impl WorthQueryReadProjectionBinding {
    pub(crate) fn from_graph(graph: &WorthQueryReadGraph) -> Self {
        let result_shape = graph.canonical().result_shape().clone();
        let authorized_projection = graph
            .authorized_projection()
            .map(|authorized| Ok(authorized.clone()))
            .unwrap_or_else(|| unrestricted_projection(graph));
        Self {
            result_shape,
            authorized_projection,
        }
    }

    pub(crate) fn consume(
        &self,
        result: &WorthQueryReadResult,
        declaration: WorthQueryProjectionDeclaration,
    ) -> WorthQueryProjectionOutcome {
        let authorized = match &self.authorized_projection {
            Ok(authorized) => authorized,
            Err(error) => {
                return WorthQueryProjectionOutcome::Unavailable(
                    WorthQueryProjectionUnavailable::AuthorityBinding(error.clone()),
                )
            }
        };
        match result.consume_projection_authority(
            &self.result_shape,
            authorized,
            declaration.contract,
        ) {
            Ok(ProjectionAuthorityOutcome::Admitted(authority)) => {
                WorthQueryProjectionOutcome::Completed(authority)
            }
            Ok(ProjectionAuthorityOutcome::AdmittedWithWarnings(authority, warnings)) => {
                WorthQueryProjectionOutcome::Advisory(WorthQueryProjectionAdvisory {
                    authority,
                    warnings,
                })
            }
            Ok(ProjectionAuthorityOutcome::AuthorityDenied(denial)) => {
                WorthQueryProjectionOutcome::Violation(WorthQueryProjectionViolation::Authority(
                    denial,
                ))
            }
            Ok(ProjectionAuthorityOutcome::ConsumptionDenied(denial)) => {
                WorthQueryProjectionOutcome::Violation(WorthQueryProjectionViolation::Consumption(
                    denial,
                ))
            }
            Ok(ProjectionAuthorityOutcome::Deferred(deferred)) => {
                WorthQueryProjectionOutcome::Deferred(deferred)
            }
            Ok(ProjectionAuthorityOutcome::SourceMismatch(mismatch)) => {
                WorthQueryProjectionOutcome::Violation(
                    WorthQueryProjectionViolation::SourceMismatch(mismatch),
                )
            }
            Err(
                crate::projection_consumption::ProjectionFactConsumptionPathError::Declaration(
                    error,
                ),
            ) => WorthQueryProjectionOutcome::Violation(
                WorthQueryProjectionViolation::Declaration(error),
            ),
            Err(crate::projection_consumption::ProjectionFactConsumptionPathError::Extraction(
                error,
            )) => WorthQueryProjectionOutcome::Unavailable(
                WorthQueryProjectionUnavailable::Extraction(error),
            ),
        }
    }
}

fn unrestricted_projection(
    graph: &WorthQueryReadGraph,
) -> Result<AuthorizedProjectionArtifact, AuthorizedProjectionError> {
    let policy_identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::ProjectionConsumptionIdentity)
            .field_shape(
                WorthQueryEvidenceTag::new("role"),
                "ordinary-unrestricted-read-projection",
            )
            .field_shape(WorthQueryEvidenceTag::new("read_graph"), graph.digest())
            .field_shape(
                WorthQueryEvidenceTag::new("schema_basis"),
                graph.schema_basis().as_str(),
            )
            .seal();
    derive_authorized_projection(
        graph.canonical().query(),
        graph.canonical().result_shape(),
        policy_identity.as_str(),
        graph.schema_basis().as_str(),
        &PolicyAspectMask::allow_all(),
        &PolicyInfluenceSet::none(),
        usize::MAX,
        usize::MAX,
    )
}

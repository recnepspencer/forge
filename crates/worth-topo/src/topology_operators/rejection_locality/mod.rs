use serde::{Deserialize, Serialize};

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::application::TopologyMutationApplicationError;
use crate::topology_operators::{TopologyDeclaredMutationMember, TopologyDeclaredMutationSequence};

use super::{
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationFamily,
    TopologyMutationNamingScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TopologyMutationRejectionClass {
    OutOfClassEdit,
    InvariantBlocked,
    NamingContinuityAmbiguous,
    NamingContinuityRejected,
    ScopeLocalizationUnavailable,
    DerivedFallbackExceeded,
}

impl TopologyMutationRejectionClass {
    pub const ALL: [Self; 6] = [
        Self::OutOfClassEdit,
        Self::InvariantBlocked,
        Self::NamingContinuityAmbiguous,
        Self::NamingContinuityRejected,
        Self::ScopeLocalizationUnavailable,
        Self::DerivedFallbackExceeded,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutOfClassEdit => "OutOfClassEdit",
            Self::InvariantBlocked => "InvariantBlocked",
            Self::NamingContinuityAmbiguous => "NamingContinuityAmbiguous",
            Self::NamingContinuityRejected => "NamingContinuityRejected",
            Self::ScopeLocalizationUnavailable => "ScopeLocalizationUnavailable",
            Self::DerivedFallbackExceeded => "DerivedFallbackExceeded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedMutationScopeRow {
    pub family: TopologyMutationFamily,
    pub rejection_class: TopologyMutationRejectionClass,
    pub changed_scopes: Vec<TopologyMutationChangedScope>,
    pub naming_scopes: Vec<TopologyMutationNamingScope>,
    pub derived_regions: Vec<TopologyDerivedRegion>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedMutationScopeReport {
    pub rows: Vec<RejectedMutationScopeRow>,
}

impl TopologyMutationApplicationError {
    pub fn rejection_class(&self) -> Option<TopologyMutationRejectionClass> {
        match self {
            Self::UnsupportedFamilies(_) => Some(TopologyMutationRejectionClass::OutOfClassEdit),
            Self::DeclarationEntry { .. } => Some(TopologyMutationRejectionClass::OutOfClassEdit),
            Self::MissingCreatedEntityReference(_)
            | Self::MissingExistingEntityBinding(_)
            | Self::MissingExistingRelationBinding(_)
            | Self::ExistingEntityOutgoingRelationCountMismatch { .. }
            | Self::ExistingEntityIncomingRelationCountMismatch { .. } => {
                Some(TopologyMutationRejectionClass::ScopeLocalizationUnavailable)
            }
            Self::CreatedEntityKindMismatch { .. }
            | Self::ExistingEntityKindMismatch { .. }
            | Self::ExistingRelationKindMismatch { .. }
            | Self::ExistingRelationSourceMismatch { .. }
            | Self::ExistingHalfEdgesNotOnSameEdge { .. }
            | Self::ExistingHalfEdgesNotOnSameLoop { .. } => {
                Some(TopologyMutationRejectionClass::InvariantBlocked)
            }
            Self::Query(_)
            | Self::MaterializedDecode(_)
            | Self::RetainedSemanticAftermathMismatch { .. } => None,
            Self::QueryAnchorFamilyMismatch { .. } => {
                Some(TopologyMutationRejectionClass::ScopeLocalizationUnavailable)
            }
        }
    }

    pub(crate) fn rejected_mutation_sequence_scope_report(
        &self,
        sequence: &TopologyDeclaredMutationSequence,
    ) -> Option<RejectedMutationScopeReport> {
        let rejection_class = self.rejection_class()?;
        let detail = self.to_string();
        let rows = rejected_members(self, sequence)
            .into_iter()
            .map(|member| {
                let record = member.record();
                RejectedMutationScopeRow {
                    family: record.family,
                    rejection_class,
                    changed_scopes: record.changed_scopes().to_vec(),
                    naming_scopes: record.naming_scopes().to_vec(),
                    derived_regions: record.derived_regions().to_vec(),
                    detail: detail.clone(),
                }
            })
            .collect();
        Some(RejectedMutationScopeReport { rows })
    }

    pub fn rejected_declaration_scope_report<D>(
        &self,
        declaration: &D,
    ) -> Option<RejectedMutationScopeReport>
    where
        D: TopologyDeclarationMutationPayload,
    {
        let sequence = declaration.clone().into_mutation_sequence();
        self.rejected_mutation_sequence_scope_report(&sequence)
    }
}

fn rejected_members<'a>(
    error: &TopologyMutationApplicationError,
    sequence: &'a TopologyDeclaredMutationSequence,
) -> Vec<TopologyDeclaredMutationMember<'a>> {
    match error {
        TopologyMutationApplicationError::UnsupportedFamilies(families) => sequence
            .members()
            .filter(|member| families.contains(&member.record().family))
            .collect(),
        _ => sequence.members().collect(),
    }
}

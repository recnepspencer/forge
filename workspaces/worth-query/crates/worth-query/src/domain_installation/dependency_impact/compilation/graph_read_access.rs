use super::super::compiled::WorthQuerySemanticDependencyRole;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) enum WorthQueryCompiledGraphReadAccess {
    Observe,
    Project,
}

impl WorthQueryCompiledGraphReadAccess {
    pub(super) const fn from_declared(
        access: worth_query_installation::facade::WorthQueryOperationGraphAccess,
    ) -> Self {
        match access {
            worth_query_installation::facade::WorthQueryOperationGraphAccess::Observe => {
                Self::Observe
            }
            worth_query_installation::facade::WorthQueryOperationGraphAccess::Project => {
                Self::Project
            }
        }
    }

    pub(super) const fn from_realized(
        kind: crate::domain_installation::WorthQueryGraphProviderCallKind,
    ) -> Option<Self> {
        match kind {
            crate::domain_installation::WorthQueryGraphProviderCallKind::Observe => {
                Some(Self::Observe)
            }
            crate::domain_installation::WorthQueryGraphProviderCallKind::Project => {
                Some(Self::Project)
            }
            crate::domain_installation::WorthQueryGraphProviderCallKind::TouchEffect
            | crate::domain_installation::WorthQueryGraphProviderCallKind::CommitAdmission => None,
        }
    }

    pub(super) const fn dependency_role(self) -> WorthQuerySemanticDependencyRole {
        match self {
            Self::Observe => WorthQuerySemanticDependencyRole::AdvisoryOnlyContext,
            Self::Project => WorthQuerySemanticDependencyRole::ProjectedValue,
        }
    }
}

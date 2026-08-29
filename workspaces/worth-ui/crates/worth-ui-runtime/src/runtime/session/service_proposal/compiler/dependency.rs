#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum UiServiceProposalStage {
    ValidatePreState,
    FamilyOwnedStaging,
    AssembleSuccessor,
    ResolveFocusAndReveal,
    DeriveMotion,
    SubmitToExistingPublication,
    SettleFamilyOwners,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg(test)]
pub(in crate::runtime) struct UiServiceProposalDependencyEdge {
    prerequisite: UiServiceProposalStage,
    dependent: UiServiceProposalStage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg(test)]
pub(in crate::runtime) enum UiServiceProposalDependencyDenial {
    WrongEdgeCount,
    DuplicateEdge,
    BackEdgeOrCycle,
    MissingFixedEdge,
}

impl UiServiceProposalStage {
    pub(in crate::runtime) const ORDER: [Self; 7] = [
        Self::ValidatePreState,
        Self::FamilyOwnedStaging,
        Self::AssembleSuccessor,
        Self::ResolveFocusAndReveal,
        Self::DeriveMotion,
        Self::SubmitToExistingPublication,
        Self::SettleFamilyOwners,
    ];

    #[cfg(test)]
    pub(in crate::runtime) const fn dependencies() -> [UiServiceProposalDependencyEdge; 6] {
        [
            edge(Self::ValidatePreState, Self::FamilyOwnedStaging),
            edge(Self::FamilyOwnedStaging, Self::AssembleSuccessor),
            edge(Self::AssembleSuccessor, Self::ResolveFocusAndReveal),
            edge(Self::ResolveFocusAndReveal, Self::DeriveMotion),
            edge(Self::DeriveMotion, Self::SubmitToExistingPublication),
            edge(Self::SubmitToExistingPublication, Self::SettleFamilyOwners),
        ]
    }

    pub(super) const fn ordinal(self) -> usize {
        match self {
            Self::ValidatePreState => 0,
            Self::FamilyOwnedStaging => 1,
            Self::AssembleSuccessor => 2,
            Self::ResolveFocusAndReveal => 3,
            Self::DeriveMotion => 4,
            Self::SubmitToExistingPublication => 5,
            Self::SettleFamilyOwners => 6,
        }
    }
}

#[cfg(test)]
impl UiServiceProposalDependencyEdge {
    #[cfg(test)]
    const fn recorded_fixture(
        prerequisite: UiServiceProposalStage,
        dependent: UiServiceProposalStage,
    ) -> Self {
        Self {
            prerequisite,
            dependent,
        }
    }

    pub(in crate::runtime) const fn prerequisite(self) -> UiServiceProposalStage {
        self.prerequisite
    }

    pub(in crate::runtime) const fn dependent(self) -> UiServiceProposalStage {
        self.dependent
    }
}

#[cfg(test)]
pub(in crate::runtime) fn validate_dependency_graph(
    edges: &[UiServiceProposalDependencyEdge],
) -> Result<(), UiServiceProposalDependencyDenial> {
    if edges.len() != UiServiceProposalStage::ORDER.len() - 1 {
        return Err(UiServiceProposalDependencyDenial::WrongEdgeCount);
    }
    for (index, candidate) in edges.iter().enumerate() {
        if edges[..index].contains(candidate) {
            return Err(UiServiceProposalDependencyDenial::DuplicateEdge);
        }
        if candidate.prerequisite.ordinal() >= candidate.dependent.ordinal() {
            return Err(UiServiceProposalDependencyDenial::BackEdgeOrCycle);
        }
    }
    for expected in UiServiceProposalStage::dependencies() {
        if !edges.contains(&expected) {
            return Err(UiServiceProposalDependencyDenial::MissingFixedEdge);
        }
    }
    Ok(())
}

#[cfg(test)]
const fn edge(
    prerequisite: UiServiceProposalStage,
    dependent: UiServiceProposalStage,
) -> UiServiceProposalDependencyEdge {
    UiServiceProposalDependencyEdge {
        prerequisite,
        dependent,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        validate_dependency_graph, UiServiceProposalDependencyDenial,
        UiServiceProposalDependencyEdge, UiServiceProposalStage,
    };

    #[test]
    fn dependency_graph_is_fixed_acyclic_and_complete() {
        let edges = UiServiceProposalStage::dependencies();
        assert_eq!(edges.len(), UiServiceProposalStage::ORDER.len() - 1);
        for (ordinal, edge) in edges.into_iter().enumerate() {
            assert_eq!(edge.prerequisite(), UiServiceProposalStage::ORDER[ordinal]);
            assert_eq!(edge.dependent(), UiServiceProposalStage::ORDER[ordinal + 1]);
            assert!(edge.prerequisite().ordinal() < edge.dependent().ordinal());
        }
        assert_eq!(validate_dependency_graph(&edges), Ok(()));
    }

    #[test]
    fn duplicate_missing_and_back_edges_cannot_mutate_the_fixed_graph() {
        let mut duplicate = UiServiceProposalStage::dependencies();
        duplicate[1] = duplicate[0];
        assert_eq!(
            validate_dependency_graph(&duplicate),
            Err(UiServiceProposalDependencyDenial::DuplicateEdge)
        );

        let mut missing = UiServiceProposalStage::dependencies();
        missing[2] = UiServiceProposalDependencyEdge::recorded_fixture(
            UiServiceProposalStage::ValidatePreState,
            UiServiceProposalStage::ResolveFocusAndReveal,
        );
        assert_eq!(
            validate_dependency_graph(&missing),
            Err(UiServiceProposalDependencyDenial::MissingFixedEdge)
        );

        let mut cycle = UiServiceProposalStage::dependencies();
        cycle[5] = UiServiceProposalDependencyEdge::recorded_fixture(
            UiServiceProposalStage::SettleFamilyOwners,
            UiServiceProposalStage::SubmitToExistingPublication,
        );
        assert_eq!(
            validate_dependency_graph(&cycle),
            Err(UiServiceProposalDependencyDenial::BackEdgeOrCycle)
        );
    }
}

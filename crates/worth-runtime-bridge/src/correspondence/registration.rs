use super::{
    BridgeCorrespondenceDenial, BridgeCorrespondenceDenialKind, BridgeSemanticDependencyCandidate,
    BridgeSignalAspectTargetDeclaration,
};

/// Frozen volatile lowering registration for one exact Query-installed
/// dependency. Signal layout is deliberately absent from portable Query
/// meaning and is selected only at runtime construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSemanticCorrespondenceRegistration {
    pub(crate) dependency: BridgeSemanticDependencyCandidate,
    pub(crate) targets: Vec<BridgeSignalAspectTargetDeclaration>,
}

impl BridgeSemanticCorrespondenceRegistration {
    pub fn new(
        dependency: BridgeSemanticDependencyCandidate,
        mut targets: Vec<BridgeSignalAspectTargetDeclaration>,
    ) -> Result<Self, BridgeCorrespondenceDenial> {
        if targets.is_empty() {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::EmptyTargetSet,
            ));
        }
        let graph = targets[0].graph_instance_id();
        if targets
            .iter()
            .any(|target| target.graph_instance_id() != graph)
        {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::MixedGraphTargetSet,
            ));
        }
        targets.sort_by_key(BridgeSignalAspectTargetDeclaration::canonical_registration_key);
        if targets.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::DuplicateTarget,
            ));
        }
        Ok(Self {
            dependency,
            targets,
        })
    }

    pub fn dependency(&self) -> &BridgeSemanticDependencyCandidate {
        &self.dependency
    }

    pub(crate) fn canonical_key(&self) -> String {
        self.dependency.canonical_registration_key()
    }

    pub(crate) fn signal_graph_instance_id(&self) -> u64 {
        self.targets[0].graph_instance_id()
    }
}

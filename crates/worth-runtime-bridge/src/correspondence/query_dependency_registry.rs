use std::collections::BTreeMap;

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};

use super::{
    BridgeSemanticCorrespondenceRegistration, BridgeSemanticDependencyCandidate,
    BridgeSignalAspectTargetDeclaration,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct AdmittedQueryDependencyRegistry {
    authoritative: Vec<BridgeSemanticCorrespondenceRegistration>,
    index: BTreeMap<String, BridgeSemanticCorrespondenceRegistration>,
    by_authority: BTreeMap<String, BridgeSemanticCorrespondenceRegistration>,
    signal_graph_instance_id: Option<u64>,
}

impl AdmittedQueryDependencyRegistry {
    pub(crate) fn freeze(
        mut registrations: Vec<BridgeSemanticCorrespondenceRegistration>,
    ) -> Result<Self, BridgeBuildError> {
        registrations.sort_by_key(BridgeSemanticCorrespondenceRegistration::canonical_key);
        let mut index = BTreeMap::new();
        let mut authoritative = Vec::new();
        let mut by_authority = BTreeMap::new();
        let mut signal_graph_instance_id = None;
        for registration in registrations {
            let registration_graph = registration.signal_graph_instance_id();
            if signal_graph_instance_id
                .is_some_and(|installed_graph| installed_graph != registration_graph)
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::MixedSemanticCorrespondenceSignalGraphs,
                    "One Bridge runtime cannot own semantic correspondences for multiple Signal graphs. Install an explicit bridge runtime per executable graph.",
                ));
            }
            signal_graph_instance_id = Some(registration_graph);
            let candidate = &registration.dependency;
            let key = candidate.canonical_registration_key();
            let authority_key = candidate.authority_registration_key();
            if let Some(existing) = by_authority.get(&authority_key) {
                if existing != &registration {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::AmbiguousQueryDependencyRegistration,
                        "One installed Query dependency authority resolved to conflicting semantic meaning.",
                    ));
                }
            }
            if let Some(existing) = index.get(&key) {
                if existing == &registration {
                    continue;
                }
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousQueryDependencyRegistration,
                    "Query dependency registration key resolved to conflicting semantic meaning.",
                ));
            }
            index.insert(key, registration.clone());
            by_authority.insert(authority_key, registration.clone());
            authoritative.push(registration);
        }
        Ok(Self {
            authoritative,
            index,
            by_authority,
            signal_graph_instance_id,
        })
    }

    pub(crate) fn resolve(
        &self,
        candidate: &BridgeSemanticDependencyCandidate,
    ) -> Option<Vec<BridgeSignalAspectTargetDeclaration>> {
        self.index
            .get(&candidate.canonical_registration_key())
            .filter(|installed| installed.dependency == *candidate)
            .map(|installed| installed.targets.clone())
    }

    pub(crate) fn rebuild_has_exact_parity(&self) -> bool {
        Self::freeze(self.authoritative.clone()).is_ok_and(|rebuilt| {
            rebuilt.index == self.index
                && rebuilt.by_authority == self.by_authority
                && rebuilt.signal_graph_instance_id == self.signal_graph_instance_id
        })
    }

    pub(crate) fn authoritative_count(&self) -> usize {
        self.authoritative.len()
    }

    pub(crate) fn authoritative_registrations(
        &self,
    ) -> &[BridgeSemanticCorrespondenceRegistration] {
        &self.authoritative
    }

    pub(crate) fn signal_graph_instance_id(&self) -> Option<u64> {
        self.signal_graph_instance_id
    }
}

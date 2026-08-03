use std::collections::BTreeMap;

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};

use super::{
    BridgeSemanticCorrespondenceRegistration, BridgeSemanticDependencyCandidate,
    BridgeSignalAspectTargetDeclaration,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct AdmittedSemanticDependencyRegistry {
    authoritative: Vec<BridgeSemanticCorrespondenceRegistration>,
    index: BTreeMap<String, BridgeSemanticCorrespondenceRegistration>,
    by_authority: BTreeMap<String, BridgeSemanticCorrespondenceRegistration>,
    signal_graph_instance_id: Option<u64>,
}

pub(crate) struct AdmittedSemanticDependencyExtension {
    registrations: Vec<BridgeSemanticCorrespondenceRegistration>,
    new_registrations: Vec<BridgeSemanticCorrespondenceRegistration>,
    counters: SemanticDependencyExtensionCounters,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SemanticDependencyExtensionCounters {
    pub(crate) existing_key_lookups: usize,
    pub(crate) batch_key_lookups: usize,
}

pub(crate) struct SemanticDependencyExtensionDenial {
    pub(crate) error: BridgeBuildError,
    pub(crate) counters: SemanticDependencyExtensionCounters,
}

impl AdmittedSemanticDependencyRegistry {
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
                        BridgeBuildErrorKind::AmbiguousSemanticDependencyRegistration,
                        "One installed source dependency authority resolved to conflicting semantic meaning.",
                    ));
                }
            }
            if let Some(existing) = index.get(&key) {
                if existing == &registration {
                    continue;
                }
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousSemanticDependencyRegistration,
                    "Semantic dependency registration key resolved to conflicting meaning.",
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

    pub(crate) fn signal_graph_instance_id(&self) -> Option<u64> {
        self.signal_graph_instance_id
    }

    pub(crate) fn admit_extension(
        &self,
        registrations: &[BridgeSemanticCorrespondenceRegistration],
    ) -> Result<AdmittedSemanticDependencyExtension, SemanticDependencyExtensionDenial> {
        let mut counters = SemanticDependencyExtensionCounters::default();
        let mut batch_by_key = BTreeMap::new();
        let mut batch_by_authority = BTreeMap::new();
        let mut new_registrations = Vec::new();
        for registration in registrations {
            require_compatible_signal_graph(self, registration, counters)?;
            let key = registration.dependency.canonical_registration_key();
            let authority_key = registration.dependency.authority_registration_key();
            counters.existing_key_lookups += 2;
            require_compatible_registration(
                self.by_authority.get(&authority_key),
                registration,
                counters,
            )?;
            if let Some(existing) = self.index.get(&key) {
                require_compatible_registration(Some(existing), registration, counters)?;
                continue;
            }
            counters.batch_key_lookups += 2;
            require_compatible_registration(
                batch_by_authority.get(&authority_key),
                registration,
                counters,
            )?;
            require_compatible_registration(batch_by_key.get(&key), registration, counters)?;
            batch_by_authority.insert(authority_key, registration.clone());
            batch_by_key.insert(key, registration.clone());
            new_registrations.push(registration.clone());
        }
        Ok(AdmittedSemanticDependencyExtension {
            registrations: registrations.to_vec(),
            new_registrations,
            counters,
        })
    }
}

impl AdmittedSemanticDependencyExtension {
    pub(crate) fn registrations(&self) -> &[BridgeSemanticCorrespondenceRegistration] {
        &self.registrations
    }

    pub(crate) const fn counters(&self) -> SemanticDependencyExtensionCounters {
        self.counters
    }

    pub(crate) fn commit(self, registry: &mut AdmittedSemanticDependencyRegistry) -> usize {
        let committed = self.new_registrations.len();
        for registration in self.new_registrations {
            let key = registration.dependency.canonical_registration_key();
            let authority_key = registration.dependency.authority_registration_key();
            registry.index.insert(key, registration.clone());
            registry
                .by_authority
                .insert(authority_key, registration.clone());
            registry.authoritative.push(registration);
        }
        if let Some(first) = self.registrations.first() {
            registry.signal_graph_instance_id = Some(first.signal_graph_instance_id());
        }
        committed
    }
}

fn require_compatible_signal_graph(
    registry: &AdmittedSemanticDependencyRegistry,
    registration: &BridgeSemanticCorrespondenceRegistration,
    counters: SemanticDependencyExtensionCounters,
) -> Result<(), SemanticDependencyExtensionDenial> {
    if registry
        .signal_graph_instance_id
        .is_some_and(|installed| installed != registration.signal_graph_instance_id())
    {
        return Err(extension_denial(
            BridgeBuildErrorKind::MixedSemanticCorrespondenceSignalGraphs,
            "One Bridge runtime cannot own semantic correspondences for multiple Signal graphs. Install an explicit bridge runtime per executable graph.",
            counters,
        ));
    }
    Ok(())
}

fn require_compatible_registration(
    existing: Option<&BridgeSemanticCorrespondenceRegistration>,
    candidate: &BridgeSemanticCorrespondenceRegistration,
    counters: SemanticDependencyExtensionCounters,
) -> Result<(), SemanticDependencyExtensionDenial> {
    if existing.is_some_and(|existing| existing != candidate) {
        return Err(extension_denial(
            BridgeBuildErrorKind::AmbiguousSemanticDependencyRegistration,
            "An installed semantic dependency registration conflicts with the admitted extension.",
            counters,
        ));
    }
    Ok(())
}

fn extension_denial(
    kind: BridgeBuildErrorKind,
    detail: &str,
    counters: SemanticDependencyExtensionCounters,
) -> SemanticDependencyExtensionDenial {
    SemanticDependencyExtensionDenial {
        error: BridgeBuildError::new(kind, detail),
        counters,
    }
}

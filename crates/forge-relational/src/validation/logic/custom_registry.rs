use std::collections::BTreeMap;
use std::sync::Arc;

use crate::validation::data::{CustomInvariantRegistration, CustomInvariantSemanticIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DuplicateCustomInvariantRegistration {
    pub(crate) identity: CustomInvariantSemanticIdentity,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FrozenCustomInvariantRegistry {
    registrations: Arc<[CustomInvariantRegistration]>,
    #[cfg_attr(not(test), allow(dead_code))]
    index_by_identity: Arc<BTreeMap<CustomInvariantSemanticIdentity, usize>>,
}

impl FrozenCustomInvariantRegistry {
    pub(crate) fn from_registrations(
        registrations: Vec<CustomInvariantRegistration>,
    ) -> Result<Self, DuplicateCustomInvariantRegistration> {
        let mut index_by_identity = BTreeMap::new();
        for (index, registration) in registrations.iter().enumerate() {
            let identity = registration.descriptor().identity.clone();
            if index_by_identity.insert(identity.clone(), index).is_some() {
                return Err(DuplicateCustomInvariantRegistration { identity });
            }
        }
        Ok(Self {
            registrations: registrations.into(),
            index_by_identity: Arc::new(index_by_identity),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.registrations.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.registrations.len()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &CustomInvariantRegistration> {
        self.registrations.iter()
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn get(
        &self,
        identity: &CustomInvariantSemanticIdentity,
    ) -> Option<&CustomInvariantRegistration> {
        self.index_by_identity
            .get(identity)
            .and_then(|index| self.registrations.get(*index))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::FrozenCustomInvariantRegistry;
    use crate::validation::data::{
        CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
        CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
        CustomInvariantRegistration, CustomInvariantRule, CustomInvariantRuleId,
        CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
        CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCostClass,
        InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
    };

    #[derive(Clone, Copy)]
    struct RegistryRule(&'static str);

    impl CustomInvariantRule for RegistryRule {
        type Scope = ();

        fn descriptor(&self) -> CustomInvariantDescriptor {
            CustomInvariantDescriptor {
                identity: CustomInvariantSemanticIdentity {
                    rule_id: CustomInvariantRuleId::new(self.0),
                    semantic_version: CustomInvariantSemanticVersion::new(1, 0),
                },
                display_name: Arc::from(self.0),
                operational: CustomInvariantOperationalMetadata {
                    execution_point: InvariantExecutionPoint::CommitBoundary,
                    groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                    cost_class: InvariantCostClass::Touched,
                    failure_effect: InvariantFailureEffect::BlockCommit,
                },
            }
        }

        fn prepare_scope(
            &self,
            _planner: &mut CustomInvariantScopePlanner<'_>,
        ) -> Result<Self::Scope, CustomInvariantPreparationError> {
            Ok(())
        }

        fn evaluate(
            &self,
            _context: &CustomInvariantExecutionContext<'_>,
            _scope: &Self::Scope,
        ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
            Ok(CustomInvariantVerdict::Pass)
        }
    }

    #[test]
    fn frozen_registry_rejects_duplicate_semantic_identity() {
        let left = CustomInvariantRegistration::new(RegistryRule("dup.rule")).unwrap();
        let right = CustomInvariantRegistration::new(RegistryRule("dup.rule")).unwrap();

        let error =
            FrozenCustomInvariantRegistry::from_registrations(vec![left, right]).unwrap_err();
        assert_eq!(error.identity.rule_id.as_str(), "dup.rule");
    }

    #[test]
    fn frozen_registry_supports_stable_lookup() {
        let registration = CustomInvariantRegistration::new(RegistryRule("lookup.rule")).unwrap();
        let identity = registration.descriptor().identity.clone();
        let registry =
            FrozenCustomInvariantRegistry::from_registrations(vec![registration]).unwrap();

        assert_eq!(registry.len(), 1);
        assert!(registry.get(&identity).is_some());
    }
}

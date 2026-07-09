use serde::{Deserialize, Serialize};

use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::handle::NodeId;
use crate::logic::prepared::PreparedDependencyCapture;

use super::denial::{DeniedHostComputedReadSet, HostComputedDenialClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmittedHostComputedReadSet {
    node: NodeId,
    dependencies: CanonicalDependencies,
}

impl AdmittedHostComputedReadSet {
    pub(crate) fn admit(
        node: NodeId,
        capture: PreparedDependencyCapture,
    ) -> Result<Self, DeniedHostComputedReadSet> {
        let dependencies =
            CanonicalDependencies::from_ordered_unique(capture.as_slice().iter().map(
                |dependency| match dependency.scope.as_ref() {
                    Some(scope) => DependencyEdge::with_partition_scope(
                        dependency.source,
                        dependency.aspect,
                        scope.clone(),
                    ),
                    None => DependencyEdge::new(dependency.source, dependency.aspect),
                },
            ));
        if let Some(self_read) = dependencies
            .as_slice()
            .iter()
            .find(|dependency| dependency.source() == node)
            .cloned()
        {
            return Err(DeniedHostComputedReadSet::new(
                node,
                HostComputedDenialClass::SelfRead,
                self_read,
            ));
        }
        Ok(Self { node, dependencies })
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn dependencies(&self) -> &[DependencyEdge] {
        self.dependencies.as_slice()
    }

    pub fn len(&self) -> usize {
        self.dependencies.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }

    pub(crate) fn to_prepared_capture(&self) -> PreparedDependencyCapture {
        let mut capture = PreparedDependencyCapture::new();
        for dependency in self.dependencies() {
            capture.record(
                dependency.source(),
                dependency.aspect(),
                dependency.scope_ref().cloned(),
            );
        }
        capture
    }

    pub(crate) fn canonical_dependencies(&self) -> &CanonicalDependencies {
        &self.dependencies
    }
}

#[cfg(test)]
mod tests {
    use crate::data::aspect::Aspect;

    use super::*;

    #[test]
    fn denies_self_reads() {
        let node = NodeId::new(4, 0);
        let mut capture = PreparedDependencyCapture::new();
        capture.record(node, Aspect::new(0), None);

        let denial =
            AdmittedHostComputedReadSet::admit(node, capture).expect_err("self-read should deny");

        assert_eq!(denial.node(), node);
        assert_eq!(denial.class(), HostComputedDenialClass::SelfRead);
        assert_eq!(denial.dependency().source(), node);
    }

    #[test]
    fn canonicalizes_duplicates() {
        let node = NodeId::new(1, 0);
        let source = NodeId::new(2, 0);
        let mut capture = PreparedDependencyCapture::new();
        capture.record(source, Aspect::new(0), None);
        capture.record(source, Aspect::new(0), None);

        let admitted = AdmittedHostComputedReadSet::admit(node, capture).unwrap();

        assert_eq!(admitted.dependencies().len(), 1);
        assert_eq!(admitted.dependencies()[0].source(), source);
    }
}

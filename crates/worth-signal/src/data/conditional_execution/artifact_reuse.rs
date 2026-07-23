use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;

use super::{SignalConditionalDecisionClass, SignalConditionalDecisionCounters};

pub(super) struct SignalConditionalArtifactReuseObservation<'a> {
    pub(super) policy: &'a super::SignalConditionalArtifactReusePolicy,
    pub(super) class: SignalConditionalDecisionClass,
    pub(super) dependency_changed: bool,
    pub(super) aspect: crate::data::aspect::Aspect,
    pub(super) before: u64,
    pub(super) after: u64,
}

pub(super) fn resolve_artifact_reuse(
    observation: SignalConditionalArtifactReuseObservation<'_>,
    resolver: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<bool, SignalError> {
    let clean_result = matches!(
        observation.class,
        SignalConditionalDecisionClass::DependencyUnchanged
            | SignalConditionalDecisionClass::ComputedRevertedClean
    );
    if !clean_result {
        return Ok(false);
    }
    counters.reuse_checks += 1;
    Ok(match observation.policy {
        super::SignalConditionalArtifactReusePolicy::NotReusable => false,
        super::SignalConditionalArtifactReusePolicy::DependencyAndOutputEquivalent => {
            !observation.dependency_changed
                && observation.class == SignalConditionalDecisionClass::DependencyUnchanged
        }
        super::SignalConditionalArtifactReusePolicy::OutputEquivalent => true,
        super::SignalConditionalArtifactReusePolicy::Installed(identity) => {
            counters.comparator_checks += 1;
            !resolver.resolve_installed(
                identity,
                observation.aspect,
                observation.before,
                observation.after,
            )?
        }
    })
}

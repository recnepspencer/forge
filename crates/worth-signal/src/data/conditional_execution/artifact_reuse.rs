use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;

use super::{SignalConditionalDecisionClass, SignalConditionalDecisionCounters};

#[allow(clippy::too_many_arguments)]
pub(super) fn resolve_artifact_reuse(
    policy: &super::SignalConditionalArtifactReusePolicy,
    class: SignalConditionalDecisionClass,
    dependency_changed: bool,
    aspect: crate::data::aspect::Aspect,
    before: u64,
    after: u64,
    resolver: &mut impl ComparatorPolicyResolver,
    counters: &mut SignalConditionalDecisionCounters,
) -> Result<bool, SignalError> {
    let clean_result = matches!(
        class,
        SignalConditionalDecisionClass::DependencyUnchanged
            | SignalConditionalDecisionClass::ComputedRevertedClean
    );
    if !clean_result {
        return Ok(false);
    }
    counters.reuse_checks += 1;
    Ok(match policy {
        super::SignalConditionalArtifactReusePolicy::NotReusable => false,
        super::SignalConditionalArtifactReusePolicy::DependencyAndOutputEquivalent => {
            !dependency_changed && class == SignalConditionalDecisionClass::DependencyUnchanged
        }
        super::SignalConditionalArtifactReusePolicy::OutputEquivalent => true,
        super::SignalConditionalArtifactReusePolicy::Installed(identity) => {
            counters.comparator_checks += 1;
            !resolver.resolve_installed(*identity, aspect, before, after)?
        }
    })
}

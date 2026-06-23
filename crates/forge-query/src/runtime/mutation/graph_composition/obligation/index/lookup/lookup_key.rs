use crate::runtime::{
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use forge_relational::facade::identity::KindId;

use super::super::selection::ForgeQueryGraphObligationOperatingWorldDescriptorKind;
use crate::runtime::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchSelector,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum ForgeQueryGraphObligationTouchLookupKey
{
    AnyGraphTouch,
    Collection(String),
    RelationKindId(KindId),
    AspectPath(String),
    DeclaredAspectOperation(String),
    MutationFamily(ForgeQueryMutationFamily),
    LifecycleFamily(ForgeQueryGraphTouchLifecycleFamily),
    ReadVerb(ForgeQueryGraphTouchReadVerb),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) enum ForgeQueryGraphObligationOperatingWorldLookupKey
{
    AnyOperatingWorld,
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
}

impl ForgeQueryGraphObligationTouchLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: &ForgeQueryGraphTouchSelector,
    ) -> Self {
        match selector.selector_kind() {
            "any-graph-touch" => Self::AnyGraphTouch,
            "collection" => Self::Collection(selector_value(selector)),
            "relation-kind-id" => Self::RelationKindId(KindId(selector_u32_value(selector))),
            "aspect-path" => Self::AspectPath(selector_value(selector)),
            "declared-aspect-operation" => Self::DeclaredAspectOperation(selector_value(selector)),
            "declared-mutation-collection" => {
                Self::Collection(declared_mutation_collection_name(selector))
            }
            "mutation-family" => {
                Self::MutationFamily(mutation_family_from_selector_value(selector))
            }
            "lifecycle-family" => {
                Self::LifecycleFamily(lifecycle_family_from_selector_value(selector))
            }
            "read-verb" => Self::ReadVerb(read_verb_from_selector_value(selector)),
            other => unreachable!("unknown graph touch selector kind `{other}`"),
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn as_kind_str(
        &self,
    ) -> &'static str {
        match self {
            Self::AnyGraphTouch => "any-graph-touch",
            Self::Collection(_) => "collection",
            Self::RelationKindId(_) => "relation-kind-id",
            Self::AspectPath(_) => "aspect-path",
            Self::DeclaredAspectOperation(_) => "declared-aspect-operation",
            Self::MutationFamily(_) => "mutation-family",
            Self::LifecycleFamily(_) => "lifecycle-family",
            Self::ReadVerb(_) => "read-verb",
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn value(
        &self,
    ) -> Option<String> {
        match self {
            Self::AnyGraphTouch => None,
            Self::Collection(value)
            | Self::AspectPath(value)
            | Self::DeclaredAspectOperation(value) => Some(value.clone()),
            Self::RelationKindId(value) => Some(value.0.to_string()),
            Self::MutationFamily(value) => Some(value.as_str().to_string()),
            Self::LifecycleFamily(value) => Some(value.as_str().to_string()),
            Self::ReadVerb(value) => Some(value.as_str().to_string()),
        }
    }
}

impl ForgeQueryGraphObligationOperatingWorldLookupKey {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_selector(
        selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        match selector {
            ForgeQueryGraphObligationOperatingWorldSelector::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            ForgeQueryGraphObligationOperatingWorldSelector::Preview => Self::Preview,
            ForgeQueryGraphObligationOperatingWorldSelector::Branch => Self::Branch,
            ForgeQueryGraphObligationOperatingWorldSelector::ConfiguredDomainHandle => {
                Self::ConfiguredDomainHandle
            }
            ForgeQueryGraphObligationOperatingWorldSelector::AnyOperatingWorld => {
                Self::AnyOperatingWorld
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn from_descriptor_kind(
        kind: ForgeQueryGraphObligationOperatingWorldDescriptorKind,
    ) -> Self {
        match kind {
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::AnyCommittedAuthority => {
                Self::AnyCommittedAuthority
            }
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::Preview => Self::Preview,
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::Branch => Self::Branch,
            ForgeQueryGraphObligationOperatingWorldDescriptorKind::ConfiguredDomainHandle => {
                Self::ConfiguredDomainHandle
            }
        }
    }

    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn as_str(
        self,
    ) -> &'static str {
        match self {
            Self::AnyOperatingWorld => "any-operating-world",
            Self::AnyCommittedAuthority => "any-committed-authority",
            Self::Preview => "preview",
            Self::Branch => "branch",
            Self::ConfiguredDomainHandle => "configured-domain-handle",
        }
    }
}

fn selector_value(selector: &ForgeQueryGraphTouchSelector) -> String {
    selector
        .selector_value()
        .expect("non-any graph touch selector has a value")
}

fn selector_u32_value(selector: &ForgeQueryGraphTouchSelector) -> u32 {
    selector_value(selector)
        .parse()
        .expect("relation kind id selector value is numeric")
}

fn declared_mutation_collection_name(selector: &ForgeQueryGraphTouchSelector) -> String {
    selector_value(selector)
        .split('|')
        .next()
        .expect("declared mutation collection selector includes collection")
        .to_string()
}

fn mutation_family_from_selector_value(
    selector: &ForgeQueryGraphTouchSelector,
) -> ForgeQueryMutationFamily {
    match selector_value(selector).as_str() {
        "insert" => ForgeQueryMutationFamily::Insert,
        "update" => ForgeQueryMutationFamily::Update,
        "assertion" => ForgeQueryMutationFamily::Assertion,
        "delete" => ForgeQueryMutationFamily::Delete,
        other => unreachable!("unknown mutation family selector value `{other}`"),
    }
}

fn lifecycle_family_from_selector_value(
    selector: &ForgeQueryGraphTouchSelector,
) -> ForgeQueryGraphTouchLifecycleFamily {
    match selector_value(selector).as_str() {
        "declaration" => ForgeQueryGraphTouchLifecycleFamily::Declaration,
        "same-batch-symbolic-entity-followup" => {
            ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicEntityFollowup
        }
        "same-batch-symbolic-relation-followup" => {
            ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationFollowup
        }
        "same-batch-symbolic-relation-retirement" => {
            ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement
        }
        "existing-target-followup" => ForgeQueryGraphTouchLifecycleFamily::ExistingTargetFollowup,
        "existing-target-retarget" => ForgeQueryGraphTouchLifecycleFamily::ExistingTargetRetarget,
        "existing-target-supersession" => {
            ForgeQueryGraphTouchLifecycleFamily::ExistingTargetSupersession
        }
        "existing-target-retirement" => {
            ForgeQueryGraphTouchLifecycleFamily::ExistingTargetRetirement
        }
        "verified-existing-target-followup" => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetFollowup
        }
        "verified-existing-target-retarget" => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget
        }
        "verified-existing-target-supersession" => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetSupersession
        }
        "verified-existing-target-retirement" => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetirement
        }
        other => unreachable!("unknown graph lifecycle selector value `{other}`"),
    }
}

fn read_verb_from_selector_value(
    selector: &ForgeQueryGraphTouchSelector,
) -> ForgeQueryGraphTouchReadVerb {
    match selector_value(selector).as_str() {
        "observes-collection" => ForgeQueryGraphTouchReadVerb::ObservesCollection,
        "observes-relation-kind" => ForgeQueryGraphTouchReadVerb::ObservesRelationKind,
        "observes-aspect-path" => ForgeQueryGraphTouchReadVerb::ObservesAspectPath,
        "exposes-derived-topology" => ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology,
        "materializes-diagnostic" => ForgeQueryGraphTouchReadVerb::MaterializesDiagnostic,
        "requires-policy-basis" => ForgeQueryGraphTouchReadVerb::RequiresPolicyBasis,
        "retains-live-subscription" => ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription,
        "crosses-operating-world" => ForgeQueryGraphTouchReadVerb::CrossesOperatingWorld,
        "reads-stale-basis-allowed" => ForgeQueryGraphTouchReadVerb::ReadsStaleBasisAllowed,
        other => unreachable!("unknown graph read verb selector value `{other}`"),
    }
}

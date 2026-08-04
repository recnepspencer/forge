use super::super::promotion::LiveQueryFamily;
use super::bridge_change::{BridgeChangeSummary, BridgeRelationDelta};
use super::query_contract::QueryRelevanceContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelevantChangeClass {
    DetailProjectionChange,
    OrderedCollectionMembershipChange,
    OrderedCollectionOrderingChange,
    BoundedMaterializationScopeChange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IrrelevantChangeClass {
    NoProjectedFieldOverlap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChangeRelevance {
    Relevant(RelevantChangeClass),
    Irrelevant(IrrelevantChangeClass),
}

impl QueryRelevanceContract {
    pub fn classify_change(&self, change: &BridgeChangeSummary) -> ChangeRelevance {
        let projected_overlap = change.field_deltas().iter().any(|delta| {
            self.projected_fields()
                .iter()
                .any(|field| field.matches(delta))
        });
        let ordering_overlap = change.field_deltas().iter().any(|delta| {
            self.ordering_fields()
                .iter()
                .any(|field| field.matches(delta))
        });
        let traversal_overlap =
            change
                .relation_deltas()
                .iter()
                .any(|delta: &BridgeRelationDelta| {
                    self.traversal_relations()
                        .iter()
                        .any(|relation| relation == delta.relation())
                });

        match self.family() {
            LiveQueryFamily::Detail => {
                if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
            LiveQueryFamily::OrderedCollection => {
                if change.membership_changed() {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::OrderedCollectionMembershipChange,
                    )
                } else if ordering_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange)
                } else if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
            LiveQueryFamily::BoundedMaterialization => {
                if change.materialization_scope_changed() || traversal_overlap {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::BoundedMaterializationScopeChange,
                    )
                } else if change.membership_changed() {
                    ChangeRelevance::Relevant(
                        RelevantChangeClass::OrderedCollectionMembershipChange,
                    )
                } else if ordering_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::OrderedCollectionOrderingChange)
                } else if projected_overlap {
                    ChangeRelevance::Relevant(RelevantChangeClass::DetailProjectionChange)
                } else {
                    ChangeRelevance::Irrelevant(IrrelevantChangeClass::NoProjectedFieldOverlap)
                }
            }
        }
    }
}

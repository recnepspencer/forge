use std::collections::BTreeSet;

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryLiveRetirement, WorthUiQueryViewIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingSuccessionDenial {
    DuplicateSuccessorView,
    ForeignSuccessorReference,
    StaleSuccessorReference,
}

#[derive(Clone, Debug)]
pub struct WorthUiQueryBindingSuccessionChange {
    predecessor: Option<WorthUiInstalledQueryBindingReference>,
    successor: Option<WorthUiInstalledQueryBindingReference>,
}

impl WorthUiQueryBindingSuccessionChange {
    pub fn new(
        predecessor: Option<WorthUiInstalledQueryBindingReference>,
        successor: Option<WorthUiInstalledQueryBindingReference>,
    ) -> Self {
        Self {
            predecessor,
            successor,
        }
    }
}

#[must_use = "prepared Query succession must be committed or abandoned with its candidate resources"]
pub struct WorthUiPreparedQueryBindingSuccession {
    candidate: super::WorthUiRuntimeQueryBinding,
    scope: WorthUiQueryBindingSuccessionScope,
}

enum WorthUiQueryBindingSuccessionScope {
    Complete(Vec<WorthUiInstalledQueryBindingReference>),
    Regional(Vec<WorthUiQueryBindingSuccessionChange>),
}

impl super::WorthUiRuntimeQueryBinding {
    pub fn prepare_succession(
        self,
        successor_references: impl IntoIterator<Item = WorthUiInstalledQueryBindingReference>,
    ) -> Result<WorthUiPreparedQueryBindingSuccession, WorthUiQueryBindingSuccessionDenial> {
        if !self.installation_is_current() {
            return Err(WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference);
        }
        let successor_references = successor_references.into_iter().collect::<Vec<_>>();
        let mut identities = BTreeSet::<WorthUiQueryViewIdentity>::new();
        for reference in &successor_references {
            if !identities.insert(reference.definition().identity().clone()) {
                return Err(WorthUiQueryBindingSuccessionDenial::DuplicateSuccessorView);
            }
            if !self.admits_reference(reference) {
                return Err(WorthUiQueryBindingSuccessionDenial::ForeignSuccessorReference);
            }
            if !reference.installation_is_current() {
                return Err(WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference);
            }
        }
        Ok(WorthUiPreparedQueryBindingSuccession {
            candidate: self,
            scope: WorthUiQueryBindingSuccessionScope::Complete(successor_references),
        })
    }

    pub fn prepare_regional_succession(
        self,
        changes: impl IntoIterator<Item = WorthUiQueryBindingSuccessionChange>,
    ) -> Result<WorthUiPreparedQueryBindingSuccession, WorthUiQueryBindingSuccessionDenial> {
        if !self.installation_is_current() {
            return Err(WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference);
        }
        let changes = changes.into_iter().collect::<Vec<_>>();
        let mut identities = BTreeSet::<WorthUiQueryViewIdentity>::new();
        for change in &changes {
            for reference in [change.predecessor.as_ref(), change.successor.as_ref()]
                .into_iter()
                .flatten()
            {
                if !self.admits_reference(reference) {
                    return Err(WorthUiQueryBindingSuccessionDenial::ForeignSuccessorReference);
                }
            }
            if change
                .successor
                .as_ref()
                .is_some_and(|reference| !reference.installation_is_current())
            {
                return Err(WorthUiQueryBindingSuccessionDenial::StaleSuccessorReference);
            }
            let identity = change
                .successor
                .as_ref()
                .or(change.predecessor.as_ref())
                .expect("regional Query changes carry one side")
                .definition()
                .identity()
                .clone();
            if !identities.insert(identity) {
                return Err(WorthUiQueryBindingSuccessionDenial::DuplicateSuccessorView);
            }
        }
        Ok(WorthUiPreparedQueryBindingSuccession {
            candidate: self,
            scope: WorthUiQueryBindingSuccessionScope::Regional(changes),
        })
    }
}

impl WorthUiPreparedQueryBindingSuccession {
    pub fn commit_once(
        self,
        active: &mut super::WorthUiRuntimeQueryBinding,
    ) -> WorthUiQueryLiveRetirement {
        let mut predecessor =
            std::mem::replace(active, super::WorthUiRuntimeQueryBinding::QueryFree);
        let mut successor = self.candidate;
        let mut retirement = Vec::new();

        match self.scope {
            WorthUiQueryBindingSuccessionScope::Complete(successor_references) => {
                commit_complete_succession(
                    &mut predecessor,
                    &mut successor,
                    &successor_references,
                    &mut retirement,
                );
            }
            WorthUiQueryBindingSuccessionScope::Regional(changes) => {
                commit_regional_succession(
                    &mut predecessor,
                    &mut successor,
                    &changes,
                    &mut retirement,
                );
            }
        }
        *active = successor;
        WorthUiQueryLiveRetirement::new(retirement)
    }
}

fn commit_regional_succession(
    predecessor: &mut super::WorthUiRuntimeQueryBinding,
    successor: &mut super::WorthUiRuntimeQueryBinding,
    changes: &[WorthUiQueryBindingSuccessionChange],
    retirement: &mut Vec<crate::WorthUiQueryLiveResource>,
) {
    successor.swap_runtime_state_with(predecessor);
    for change in changes {
        if change.predecessor == change.successor {
            if let Some(reference) = &change.successor {
                predecessor.take_settlement(reference);
                predecessor.take_settled_snapshot(reference);
                if let Some(candidate_resource) = predecessor.take_live_resource(reference) {
                    retirement.push(candidate_resource);
                }
            }
            continue;
        }
        if let Some(reference) = &change.predecessor {
            successor.take_settlement(reference);
            successor.take_settled_snapshot(reference);
            if let Some(resource) = successor.take_live_resource(reference) {
                retirement.push(resource);
            }
        }
        if let Some(reference) = &change.successor {
            if let Some(settlement) = predecessor.take_settlement(reference) {
                successor.replace_settlement(reference, settlement);
            }
            if let Some(projection) = predecessor.take_settled_snapshot(reference) {
                successor.replace_settled_snapshot(projection);
            }
            if let Some(resource) = predecessor.take_live_resource(reference) {
                if let Some(displaced) = successor.replace_live_resource(reference, resource) {
                    retirement.push(displaced);
                }
            }
        }
    }
    predecessor.drain_live_resources_into(retirement);
    successor.finish_managed_live_succession(retirement);
}

fn commit_complete_succession(
    predecessor: &mut super::WorthUiRuntimeQueryBinding,
    successor: &mut super::WorthUiRuntimeQueryBinding,
    successor_references: &[WorthUiInstalledQueryBindingReference],
    retirement: &mut Vec<crate::WorthUiQueryLiveResource>,
) {
    for reference in successor_references {
        if !predecessor.admits_reference(reference) {
            continue;
        }
        if let Some(settlement) = predecessor.take_settlement(reference) {
            successor.replace_settlement(reference, settlement);
        }
        if let Some(projection) = predecessor.take_settled_snapshot(reference) {
            successor.take_settled_snapshot(reference);
            successor.replace_settled_snapshot(projection);
        }
        if let Some(resource) = predecessor.take_live_resource(reference) {
            if let Some(candidate_resource) = successor.replace_live_resource(reference, resource) {
                retirement.push(candidate_resource);
            }
        }
    }
    predecessor.drain_live_resources_into(retirement);
    successor.retain_only_live_resources_for(successor_references, retirement);
    successor.retain_only_settlements_for(successor_references);
    successor.finish_managed_live_succession(retirement);
}

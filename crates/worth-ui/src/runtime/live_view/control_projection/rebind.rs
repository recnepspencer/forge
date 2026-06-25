use std::collections::BTreeMap;

use crate::runtime::{WorthUiChangedRuntimeFacts, WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::receipt::WorthUiLiveViewControlProjectionReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlProjectionCompatibilityReceipt {
    Preserved,
    OutOfOptionSetPreserved,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionCompatibilityRow {
    control_id: String,
    prior_kind: Option<String>,
    next_kind: Option<String>,
    compatibility: WorthUiLiveViewControlProjectionCompatibilityReceipt,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionRebindCounters {
    prior_control_count: usize,
    next_control_count: usize,
    changed_control_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionRebindReceipt {
    changed_facts: WorthUiChangedRuntimeFacts,
    compatibility: WorthUiLiveViewControlProjectionCompatibilityReceipt,
    compatibility_rows: Vec<WorthUiLiveViewControlProjectionCompatibilityRow>,
    counters: WorthUiLiveViewControlProjectionRebindCounters,
}

impl WorthUiLiveViewControlProjectionRebindReceipt {
    pub fn from_control_projection_receipts(
        prior: &[WorthUiLiveViewControlProjectionReceipt],
        next: &[WorthUiLiveViewControlProjectionReceipt],
    ) -> Self {
        let changed_facts = changed_control_projection_facts(prior, next);
        let compatibility_rows = compatibility_rows(prior, next);
        let compatibility = aggregate_compatibility(&compatibility_rows);
        let counters = WorthUiLiveViewControlProjectionRebindCounters {
            prior_control_count: prior.len(),
            next_control_count: next.len(),
            changed_control_count: compatibility_rows
                .iter()
                .filter(|row| row.prior_kind != row.next_kind)
                .count(),
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        Self {
            changed_facts,
            compatibility,
            compatibility_rows,
            counters,
        }
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn compatibility(&self) -> WorthUiLiveViewControlProjectionCompatibilityReceipt {
        self.compatibility
    }

    pub fn compatibility_rows(&self) -> &[WorthUiLiveViewControlProjectionCompatibilityRow] {
        &self.compatibility_rows
    }

    pub fn counters(&self) -> WorthUiLiveViewControlProjectionRebindCounters {
        self.counters
    }
}

impl WorthUiLiveViewControlProjectionCompatibilityRow {
    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn prior_kind(&self) -> Option<&str> {
        self.prior_kind.as_deref()
    }

    pub fn next_kind(&self) -> Option<&str> {
        self.next_kind.as_deref()
    }

    pub fn compatibility(&self) -> WorthUiLiveViewControlProjectionCompatibilityReceipt {
        self.compatibility
    }
}

impl WorthUiLiveViewControlProjectionRebindCounters {
    pub fn prior_control_count(self) -> usize {
        self.prior_control_count
    }

    pub fn next_control_count(self) -> usize {
        self.next_control_count
    }

    pub fn changed_control_count(self) -> usize {
        self.changed_control_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

fn compatibility_rows(
    prior: &[WorthUiLiveViewControlProjectionReceipt],
    next: &[WorthUiLiveViewControlProjectionReceipt],
) -> Vec<WorthUiLiveViewControlProjectionCompatibilityRow> {
    let prior_controls = prior
        .iter()
        .map(|control| (control.control_id(), control))
        .collect::<BTreeMap<_, _>>();
    let mut rows = next
        .iter()
        .map(|control| {
            let prior = prior_controls.get(control.control_id()).copied();
            let compatibility = match prior {
                Some(prior)
                    if prior.binding().binding_digest() != control.binding().binding_digest() =>
                {
                    WorthUiLiveViewControlProjectionCompatibilityReceipt::Denied
                }
                _ => WorthUiLiveViewControlProjectionCompatibilityReceipt::Preserved,
            };
            WorthUiLiveViewControlProjectionCompatibilityRow {
                control_id: control.control_id().to_owned(),
                prior_kind: prior.map(|prior| prior.kind().token().to_owned()),
                next_kind: Some(control.kind().token().to_owned()),
                compatibility,
            }
        })
        .collect::<Vec<_>>();
    for control in prior {
        if !next
            .iter()
            .any(|candidate| candidate.control_id() == control.control_id())
        {
            rows.push(WorthUiLiveViewControlProjectionCompatibilityRow {
                control_id: control.control_id().to_owned(),
                prior_kind: Some(control.kind().token().to_owned()),
                next_kind: None,
                compatibility: WorthUiLiveViewControlProjectionCompatibilityReceipt::Denied,
            });
        }
    }
    rows
}

fn aggregate_compatibility(
    rows: &[WorthUiLiveViewControlProjectionCompatibilityRow],
) -> WorthUiLiveViewControlProjectionCompatibilityReceipt {
    if rows.iter().any(|row| {
        row.compatibility == WorthUiLiveViewControlProjectionCompatibilityReceipt::Denied
    }) {
        WorthUiLiveViewControlProjectionCompatibilityReceipt::Denied
    } else if rows.iter().any(|row| {
        row.compatibility
            == WorthUiLiveViewControlProjectionCompatibilityReceipt::OutOfOptionSetPreserved
    }) {
        WorthUiLiveViewControlProjectionCompatibilityReceipt::OutOfOptionSetPreserved
    } else {
        WorthUiLiveViewControlProjectionCompatibilityReceipt::Preserved
    }
}

fn changed_control_projection_facts(
    prior: &[WorthUiLiveViewControlProjectionReceipt],
    next: &[WorthUiLiveViewControlProjectionReceipt],
) -> WorthUiChangedRuntimeFacts {
    let prior_digests = prior
        .iter()
        .map(|control| (control.control_id(), control.control_projection_digest()))
        .collect::<BTreeMap<_, _>>();
    let mut changed = WorthUiRuntimeFactSet::empty();
    for control in next {
        if prior_digests.get(control.control_id()) != Some(&control.control_projection_digest()) {
            changed.insert(control_projection_fact(control));
        }
    }
    for control in prior {
        if !next
            .iter()
            .any(|candidate| candidate.control_id() == control.control_id())
        {
            changed.insert(control_projection_fact(control));
        }
    }
    WorthUiChangedRuntimeFacts::from_runtime(changed)
}

fn control_projection_fact(
    control: &WorthUiLiveViewControlProjectionReceipt,
) -> WorthUiRuntimeFactId {
    WorthUiRuntimeFactId::live_view_control_projection(format!(
        "{}:{}",
        control.live_view_id(),
        control.control_id()
    ))
}

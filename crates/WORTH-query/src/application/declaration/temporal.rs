use super::input::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryTemporalDeclarationSupport {
    Unsupported,
    CanonicalIdentityOnly,
    DeferredDebt,
}

impl WorthQueryTemporalDeclarationSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::CanonicalIdentityOnly => "canonical_identity_only",
            Self::DeferredDebt => "deferred_debt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryTemporalDuration {
    milliseconds: u64,
}

impl WorthQueryTemporalDuration {
    pub fn milliseconds(milliseconds: u64) -> Self {
        Self { milliseconds }
    }

    pub fn seconds(seconds: u64) -> Self {
        Self::milliseconds(seconds.saturating_mul(1_000))
    }

    pub fn minutes(minutes: u64) -> Self {
        Self::seconds(minutes.saturating_mul(60))
    }

    pub fn as_milliseconds(self) -> u64 {
        self.milliseconds
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryTemporalWindowKind {
    Rolling,
    Sliding,
}

impl WorthQueryTemporalWindowKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rolling => "rolling-window",
            Self::Sliding => "sliding-window",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryTemporalDeclarationClause {
    StaleAfter {
        duration: WorthQueryTemporalDuration,
    },
    Interval {
        every: WorthQueryTemporalDuration,
    },
    Deadline {
        within: WorthQueryTemporalDuration,
    },
    Window {
        kind: WorthQueryTemporalWindowKind,
        width: WorthQueryTemporalDuration,
        step: Option<WorthQueryTemporalDuration>,
    },
}

impl WorthQueryTemporalDeclarationClause {
    pub fn stale_after(duration: WorthQueryTemporalDuration) -> Self {
        Self::StaleAfter { duration }
    }

    pub fn interval(every: WorthQueryTemporalDuration) -> Self {
        Self::Interval { every }
    }

    pub fn deadline(within: WorthQueryTemporalDuration) -> Self {
        Self::Deadline { within }
    }

    pub fn rolling_window(width: WorthQueryTemporalDuration) -> Self {
        Self::Window {
            kind: WorthQueryTemporalWindowKind::Rolling,
            width,
            step: None,
        }
    }

    pub fn sliding_window(
        width: WorthQueryTemporalDuration,
        step: WorthQueryTemporalDuration,
    ) -> Self {
        Self::Window {
            kind: WorthQueryTemporalWindowKind::Sliding,
            width,
            step: Some(step),
        }
    }

    fn family_key(&self) -> &'static str {
        match self {
            Self::StaleAfter { .. } => "stale-after",
            Self::Interval { .. } => "interval",
            Self::Deadline { .. } => "deadline",
            Self::Window { kind, .. } => kind.as_str(),
        }
    }

    fn normalized_key(&self) -> String {
        match self {
            Self::StaleAfter { duration } => {
                format!("stale-after:{}", duration.as_milliseconds())
            }
            Self::Interval { every } => format!("interval:{}", every.as_milliseconds()),
            Self::Deadline { within } => format!("deadline:{}", within.as_milliseconds()),
            Self::Window { kind, width, step } => format!(
                "window:{}:{}:{}",
                kind.as_str(),
                width.as_milliseconds(),
                step.map(WorthQueryTemporalDuration::as_milliseconds)
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
        }
    }
}

pub(crate) fn normalize_temporal_clauses(
    clauses: Vec<WorthQueryTemporalDeclarationClause>,
) -> Vec<WorthQueryTemporalDeclarationClause> {
    let mut clauses = clauses;
    clauses.sort_by_cached_key(WorthQueryTemporalDeclarationClause::normalized_key);
    clauses.dedup();
    clauses
}

pub(crate) fn temporal_entries(
    clauses: &[WorthQueryTemporalDeclarationClause],
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| clause_entries(index, clause))
        .collect()
}

fn clause_entries(
    index: usize,
    clause: &WorthQueryTemporalDeclarationClause,
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    let base = format!("declaration.temporal.{index}");
    let mut entries = vec![WorthQueryDeclarationCanonicalEntry::new(
        format!("{base}.family"),
        WorthQueryDeclarationCanonicalEntryKind::Shape,
        WorthQueryDeclarationCanonicalValue::ExactText(clause.family_key().to_string()),
    )];

    match clause {
        WorthQueryTemporalDeclarationClause::StaleAfter { duration } => {
            entries.push(milliseconds_entry(format!("{base}.duration_ms"), *duration));
        }
        WorthQueryTemporalDeclarationClause::Interval { every } => {
            entries.push(milliseconds_entry(format!("{base}.every_ms"), *every));
        }
        WorthQueryTemporalDeclarationClause::Deadline { within } => {
            entries.push(milliseconds_entry(format!("{base}.within_ms"), *within));
        }
        WorthQueryTemporalDeclarationClause::Window { kind, width, step } => {
            entries.push(WorthQueryDeclarationCanonicalEntry::new(
                format!("{base}.window_kind"),
                WorthQueryDeclarationCanonicalEntryKind::Shape,
                WorthQueryDeclarationCanonicalValue::ExactText(kind.as_str().to_string()),
            ));
            entries.push(milliseconds_entry(format!("{base}.width_ms"), *width));
            if let Some(step) = step {
                entries.push(milliseconds_entry(format!("{base}.step_ms"), *step));
            }
        }
    }

    entries
}

fn milliseconds_entry(
    locus: impl Into<String>,
    duration: WorthQueryTemporalDuration,
) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::new(
        locus,
        WorthQueryDeclarationCanonicalEntryKind::Field,
        WorthQueryDeclarationCanonicalValue::UnsignedInteger(duration.as_milliseconds().into()),
    )
}

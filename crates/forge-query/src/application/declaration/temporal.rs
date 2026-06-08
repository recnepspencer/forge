use super::input::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryTemporalDeclarationSupport {
    Unsupported,
    CanonicalIdentityOnly,
    DeferredDebt,
}

impl ForgeQueryTemporalDeclarationSupport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::CanonicalIdentityOnly => "canonical_identity_only",
            Self::DeferredDebt => "deferred_debt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryTemporalDuration {
    milliseconds: u64,
}

impl ForgeQueryTemporalDuration {
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
pub enum ForgeQueryTemporalWindowKind {
    Rolling,
    Sliding,
}

impl ForgeQueryTemporalWindowKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Rolling => "rolling-window",
            Self::Sliding => "sliding-window",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryTemporalDeclarationClause {
    StaleAfter {
        duration: ForgeQueryTemporalDuration,
    },
    Interval {
        every: ForgeQueryTemporalDuration,
    },
    Deadline {
        within: ForgeQueryTemporalDuration,
    },
    Window {
        kind: ForgeQueryTemporalWindowKind,
        width: ForgeQueryTemporalDuration,
        step: Option<ForgeQueryTemporalDuration>,
    },
}

impl ForgeQueryTemporalDeclarationClause {
    pub fn stale_after(duration: ForgeQueryTemporalDuration) -> Self {
        Self::StaleAfter { duration }
    }

    pub fn interval(every: ForgeQueryTemporalDuration) -> Self {
        Self::Interval { every }
    }

    pub fn deadline(within: ForgeQueryTemporalDuration) -> Self {
        Self::Deadline { within }
    }

    pub fn rolling_window(width: ForgeQueryTemporalDuration) -> Self {
        Self::Window {
            kind: ForgeQueryTemporalWindowKind::Rolling,
            width,
            step: None,
        }
    }

    pub fn sliding_window(
        width: ForgeQueryTemporalDuration,
        step: ForgeQueryTemporalDuration,
    ) -> Self {
        Self::Window {
            kind: ForgeQueryTemporalWindowKind::Sliding,
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
                step.map(ForgeQueryTemporalDuration::as_milliseconds)
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
        }
    }
}

pub(crate) fn normalize_temporal_clauses(
    clauses: Vec<ForgeQueryTemporalDeclarationClause>,
) -> Vec<ForgeQueryTemporalDeclarationClause> {
    let mut clauses = clauses;
    clauses.sort_by_cached_key(ForgeQueryTemporalDeclarationClause::normalized_key);
    clauses.dedup();
    clauses
}

pub(crate) fn temporal_entries(
    clauses: &[ForgeQueryTemporalDeclarationClause],
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| clause_entries(index, clause))
        .collect()
}

fn clause_entries(
    index: usize,
    clause: &ForgeQueryTemporalDeclarationClause,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    let base = format!("declaration.temporal.{index}");
    let mut entries = vec![ForgeQueryDeclarationCanonicalEntry::new(
        format!("{base}.family"),
        ForgeQueryDeclarationCanonicalEntryKind::Shape,
        ForgeQueryDeclarationCanonicalValue::ExactText(clause.family_key().to_string()),
    )];

    match clause {
        ForgeQueryTemporalDeclarationClause::StaleAfter { duration } => {
            entries.push(milliseconds_entry(format!("{base}.duration_ms"), *duration));
        }
        ForgeQueryTemporalDeclarationClause::Interval { every } => {
            entries.push(milliseconds_entry(format!("{base}.every_ms"), *every));
        }
        ForgeQueryTemporalDeclarationClause::Deadline { within } => {
            entries.push(milliseconds_entry(format!("{base}.within_ms"), *within));
        }
        ForgeQueryTemporalDeclarationClause::Window { kind, width, step } => {
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{base}.window_kind"),
                ForgeQueryDeclarationCanonicalEntryKind::Shape,
                ForgeQueryDeclarationCanonicalValue::ExactText(kind.as_str().to_string()),
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
    duration: ForgeQueryTemporalDuration,
) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::new(
        locus,
        ForgeQueryDeclarationCanonicalEntryKind::Field,
        ForgeQueryDeclarationCanonicalValue::UnsignedInteger(duration.as_milliseconds().into()),
    )
}

use std::path::Path;

use super::inventory::{ForgeQuerySharedReadPinningInventoryRow, SHARED_READ_PINNING_INVENTORY};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningForbiddenPattern {
    name: &'static str,
    pattern: &'static str,
}

impl ForgeQuerySharedReadPinningForbiddenPattern {
    pub const fn new(name: &'static str, pattern: &'static str) -> Self {
        Self { name, pattern }
    }

    pub fn name(self) -> &'static str {
        self.name
    }

    pub fn pattern(self) -> &'static str {
        self.pattern
    }
}

pub const SHARED_READ_MINT_FORBIDDEN_PATTERNS: &[ForgeQuerySharedReadPinningForbiddenPattern] = &[
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "materialization rows cloned at shared-read mint",
        "runtime_view.materialization.rows().to_vec()",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "materialization binding cloned at shared-read artifact mint",
        "derived_view.materialization.clone()",
    ),
];

pub const SHARED_READ_PIN_HOT_PATH_FORBIDDEN_PATTERNS:
    &[ForgeQuerySharedReadPinningForbiddenPattern] = &[
    ForgeQuerySharedReadPinningForbiddenPattern::new("mutex on shared-read pin hot path", "Mutex<"),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "rwlock on shared-read pin hot path",
        "RwLock<",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "lock acquisition on shared-read pin hot path",
        ".lock().expect",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "formatted snapshot token standing in for generation identity",
        "snapshot_token",
    ),
];

pub const SHARED_READ_PIN_RETIRE_FORBIDDEN_PATTERNS:
    &[ForgeQuerySharedReadPinningForbiddenPattern] = &[
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "hard-coded zero residue assertion",
        "assert_eq!(0, 0)",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "formatted snapshot token standing in for retire identity",
        "snapshot_token",
    ),
];

pub const SHARED_READ_PIN_REQUIRED_PATTERNS: &[ForgeQuerySharedReadPinningForbiddenPattern] = &[
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "lock-free current generation load",
        "self.current_generation.load()",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "explicit generation release surface",
        "fn release_generation(",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "explicit retired-generation drain surface",
        "fn drain_retired_generation(",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "retained generation artifact cleanup",
        "retain_generations(",
    ),
    ForgeQuerySharedReadPinningForbiddenPattern::new(
        "runtime-sourced hot-path lock counter",
        "committed_read_hot_path_lock_count",
    ),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadPinningScanFailure {
    path: &'static str,
    pattern_name: &'static str,
    pattern: &'static str,
}

impl ForgeQuerySharedReadPinningScanFailure {
    fn new(path: &'static str, pattern: ForgeQuerySharedReadPinningForbiddenPattern) -> Self {
        Self {
            path,
            pattern_name: pattern.name(),
            pattern: pattern.pattern(),
        }
    }
}

pub fn scan_shared_read_mint_forbidden_patterns(
    workspace_root: impl AsRef<Path>,
) -> Vec<ForgeQuerySharedReadPinningScanFailure> {
    scan_shared_read_pinning_patterns(
        workspace_root,
        |row| row.role().contains("mint"),
        SHARED_READ_MINT_FORBIDDEN_PATTERNS,
    )
}

pub fn scan_shared_read_pin_hot_path_forbidden_patterns(
    workspace_root: impl AsRef<Path>,
) -> Vec<ForgeQuerySharedReadPinningScanFailure> {
    scan_shared_read_pinning_patterns(
        workspace_root,
        |row| row.role().contains("hot-path"),
        SHARED_READ_PIN_HOT_PATH_FORBIDDEN_PATTERNS,
    )
}

pub fn scan_shared_read_pin_retire_forbidden_patterns(
    workspace_root: impl AsRef<Path>,
) -> Vec<ForgeQuerySharedReadPinningScanFailure> {
    scan_shared_read_pinning_patterns(
        workspace_root,
        |row| row.role().contains("retirement") || row.role().contains("drain"),
        SHARED_READ_PIN_RETIRE_FORBIDDEN_PATTERNS,
    )
}

pub fn scan_shared_read_pin_required_pattern_failures(
    workspace_root: impl AsRef<Path>,
) -> Vec<ForgeQuerySharedReadPinningScanFailure> {
    let workspace_root = workspace_root.as_ref();
    let inventory_source = SHARED_READ_PINNING_INVENTORY
        .iter()
        .map(|row| std::fs::read_to_string(workspace_root.join(row.path())).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");

    SHARED_READ_PIN_REQUIRED_PATTERNS
        .iter()
        .copied()
        .filter(|pattern| !inventory_source.contains(pattern.pattern()))
        .map(|pattern| {
            ForgeQuerySharedReadPinningScanFailure::new("shared-read pinning inventory", pattern)
        })
        .collect()
}

fn scan_shared_read_pinning_patterns(
    workspace_root: impl AsRef<Path>,
    included: impl Fn(ForgeQuerySharedReadPinningInventoryRow) -> bool + Copy,
    patterns: &'static [ForgeQuerySharedReadPinningForbiddenPattern],
) -> Vec<ForgeQuerySharedReadPinningScanFailure> {
    let workspace_root = workspace_root.as_ref();
    SHARED_READ_PINNING_INVENTORY
        .iter()
        .copied()
        .filter(|row| included(*row))
        .flat_map(|row| {
            let source =
                std::fs::read_to_string(workspace_root.join(row.path())).unwrap_or_default();
            patterns
                .iter()
                .copied()
                .filter(move |pattern| source.contains(pattern.pattern()))
                .map(move |pattern| {
                    ForgeQuerySharedReadPinningScanFailure::new(row.path(), pattern)
                })
        })
        .collect()
}

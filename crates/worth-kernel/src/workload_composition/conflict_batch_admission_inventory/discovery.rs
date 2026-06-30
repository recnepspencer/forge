use std::fs;
use std::path::{Path, PathBuf};

use super::closeout::ConflictBatchAdmissionInventory;
use super::error::ConflictBatchAdmissionInventoryError;
use super::scan_pattern::ConflictBatchAdmissionScanPattern;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionDiscoveredSurface {
    path: PathBuf,
    pattern: ConflictBatchAdmissionScanPattern,
    surface_name: String,
}

impl ConflictBatchAdmissionDiscoveredSurface {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub const fn pattern(&self) -> ConflictBatchAdmissionScanPattern {
        self.pattern
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub(crate) fn path_matches(&self, source_path: &str) -> bool {
        let discovered = normalized_path(&self.path);
        let source = source_path.replace('\\', "/");
        discovered == source || discovered.ends_with(&source)
    }

    pub(crate) fn surface_matches(&self, surface_name: &str) -> bool {
        let row_name = surface_name.to_ascii_lowercase();
        let discovered_name = self.surface_name.to_ascii_lowercase();
        row_name == discovered_name
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConflictBatchAdmissionDiscoveryReport {
    scanned_file_count: usize,
    discovered_surfaces: Vec<ConflictBatchAdmissionDiscoveredSurface>,
}

impl ConflictBatchAdmissionDiscoveryReport {
    pub fn scan_roots(
        roots: &[impl AsRef<Path>],
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        let mut report = Self::default();
        for root in roots {
            scan_dir(root.as_ref(), &mut report)?;
        }
        Ok(report)
    }

    pub fn scan_root(root: &Path) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        Self::scan_roots(&[root])
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn discovered_surfaces(&self) -> &[ConflictBatchAdmissionDiscoveredSurface] {
        &self.discovered_surfaces
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionReconciliation {
    unclassified_surfaces: Vec<ConflictBatchAdmissionDiscoveredSurface>,
}

impl ConflictBatchAdmissionReconciliation {
    pub fn from_inventory_and_discovery(
        inventory: &ConflictBatchAdmissionInventory,
        discovery: &ConflictBatchAdmissionDiscoveryReport,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        let unclassified_surfaces = discovery
            .discovered_surfaces()
            .iter()
            .filter(|surface| !inventory.contains_discovered_surface(surface))
            .cloned()
            .collect::<Vec<_>>();
        if let Some(surface) = unclassified_surfaces.first() {
            return Err(
                ConflictBatchAdmissionInventoryError::UnclassifiedDiscoveredSurface(format!(
                    "{} declares `{}` classified as `{}`",
                    surface.path().display(),
                    surface.surface_name(),
                    surface.pattern().pattern()
                )),
            );
        }
        Ok(Self {
            unclassified_surfaces,
        })
    }

    pub fn unclassified_surfaces(&self) -> &[ConflictBatchAdmissionDiscoveredSurface] {
        &self.unclassified_surfaces
    }
}

fn scan_dir(
    dir: &Path,
    report: &mut ConflictBatchAdmissionDiscoveryReport,
) -> Result<(), ConflictBatchAdmissionInventoryError> {
    let entries = fs::read_dir(dir).map_err(|error| {
        ConflictBatchAdmissionInventoryError::SourceFirewallViolation(format!(
            "cannot scan {}: {error}",
            dir.display()
        ))
    })?;
    for entry in entries {
        let path = entry
            .map_err(|error| {
                ConflictBatchAdmissionInventoryError::SourceFirewallViolation(error.to_string())
            })?
            .path();
        if path.is_dir() {
            scan_dir(&path, report)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            scan_file(&path, report)?;
        }
    }
    Ok(())
}

fn scan_file(
    path: &Path,
    report: &mut ConflictBatchAdmissionDiscoveryReport,
) -> Result<(), ConflictBatchAdmissionInventoryError> {
    report.scanned_file_count += 1;
    if is_ignored_closeout_path(path) {
        return Ok(());
    }
    let text = fs::read_to_string(path).map_err(|error| {
        ConflictBatchAdmissionInventoryError::SourceFirewallViolation(format!(
            "cannot read {}: {error}",
            path.display()
        ))
    })?;
    let mut impl_context = ImplContext::default();
    for line in text.lines() {
        impl_context.observe_opening(line);
        let Some(identifier) = declared_identifier(line, impl_context.current_type()) else {
            impl_context.observe_closing(line);
            continue;
        };
        for pattern in ConflictBatchAdmissionScanPattern::all() {
            if pattern.matches_surface(path, &identifier) {
                report
                    .discovered_surfaces
                    .push(ConflictBatchAdmissionDiscoveredSurface {
                        path: path.to_path_buf(),
                        pattern: *pattern,
                        surface_name: identifier.clone(),
                    });
            }
        }
        impl_context.observe_closing(line);
    }
    Ok(())
}

fn declared_identifier(line: &str, impl_type: Option<&str>) -> Option<String> {
    let rest = strip_declaration_prefixes(line.trim_start());
    for keyword in ["fn ", "struct ", "enum ", "mod "] {
        if let Some(name) = rest.strip_prefix(keyword) {
            let identifier = name
                .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
                .next()
                .filter(|value| !value.is_empty())?;
            if keyword == "fn " {
                if let Some(impl_type) = impl_type {
                    return Some(format!("{impl_type}::{identifier}"));
                }
            }
            return Some(identifier.to_string());
        }
    }
    None
}

fn is_ignored_closeout_path(path: &Path) -> bool {
    let normalized = normalized_path(path);
    normalized.contains("/conflict_batch_admission_inventory/")
        || normalized.contains("/compile_fail/")
        || normalized.contains("/test_support/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with("/tests.rs")
}

fn strip_declaration_prefixes(mut rest: &str) -> &str {
    loop {
        let next = rest
            .strip_prefix("pub(crate) ")
            .or_else(|| rest.strip_prefix("pub(super) "))
            .or_else(|| rest.strip_prefix("pub "))
            .or_else(|| strip_pub_in(rest))
            .or_else(|| rest.strip_prefix("async "))
            .or_else(|| rest.strip_prefix("const "))
            .or_else(|| rest.strip_prefix("unsafe "))
            .or_else(|| rest.strip_prefix("default "));
        match next {
            Some(value) => rest = value.trim_start(),
            None => return rest,
        }
    }
}

fn strip_pub_in(rest: &str) -> Option<&str> {
    let rest = rest.strip_prefix("pub(in ")?;
    let closing = rest.find(") ")?;
    Some(&rest[closing + 2..])
}

#[derive(Default)]
struct ImplContext {
    current_type: Option<String>,
    brace_depth: i32,
}

impl ImplContext {
    fn current_type(&self) -> Option<&str> {
        self.current_type.as_deref()
    }

    fn observe_opening(&mut self, line: &str) {
        if self.current_type.is_none() {
            if let Some(type_name) = impl_type_name(line.trim_start()) {
                self.current_type = Some(type_name.to_string());
            }
        }
    }

    fn observe_closing(&mut self, line: &str) {
        if self.current_type.is_none() {
            return;
        }
        self.brace_depth += brace_delta(line);
        if self.brace_depth <= 0 {
            self.current_type = None;
            self.brace_depth = 0;
        }
    }
}

fn impl_type_name(line: &str) -> Option<&str> {
    let rest = impl_header_body(line)?;
    let candidate = rest
        .split(|ch: char| ch.is_whitespace() || ch == '{' || ch == '<')
        .next()?;
    if candidate.is_empty() || candidate.contains("::") {
        return None;
    }
    Some(candidate)
}

fn impl_header_body(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("impl ") {
        return Some(rest);
    }
    let rest = line.strip_prefix("impl<")?;
    let (_, body) = rest.split_once("> ")?;
    Some(body)
}

fn brace_delta(line: &str) -> i32 {
    line.chars().fold(0, |depth, ch| match ch {
        '{' => depth + 1,
        '}' => depth - 1,
        _ => depth,
    })
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

use std::collections::BTreeMap;

use crate::{hard_prohibition_registry, WorthQueryProhibitedSeam};

pub(crate) struct WorthQueryBoundaryAuditCallIndex {
    method_names: BTreeMap<&'static str, WorthQueryProhibitedSeam>,
    associated_path_suffixes: Vec<WorthQueryBoundaryAuditAssociatedPathSuffix>,
}

impl WorthQueryBoundaryAuditCallIndex {
    pub(crate) fn seam_for_method_name(&self, call_name: &str) -> Option<WorthQueryProhibitedSeam> {
        self.method_names.get(call_name).copied()
    }

    pub(crate) fn seam_for_associated_path(
        &self,
        path: &syn::ExprPath,
    ) -> Option<WorthQueryProhibitedSeam> {
        self.associated_path_suffixes
            .iter()
            .find(|suffix| suffix.matches(path))
            .map(|suffix| suffix.seam)
    }
}

struct WorthQueryBoundaryAuditAssociatedPathSuffix {
    segments: Vec<&'static str>,
    seam: WorthQueryProhibitedSeam,
}

impl WorthQueryBoundaryAuditAssociatedPathSuffix {
    fn new(public_symbol: &'static str, seam: WorthQueryProhibitedSeam) -> Self {
        Self {
            segments: public_symbol.split("::").collect(),
            seam,
        }
    }

    fn matches(&self, path: &syn::ExprPath) -> bool {
        if path.path.segments.len() < self.segments.len() {
            return false;
        }
        path.path
            .segments
            .iter()
            .rev()
            .zip(self.segments.iter().rev())
            .all(|(actual, expected)| actual.ident == *expected)
    }
}

pub(crate) fn hard_prohibition_boundary_audit_call_index() -> WorthQueryBoundaryAuditCallIndex {
    let registry = hard_prohibition_registry();
    WorthQueryBoundaryAuditCallIndex {
        method_names: registry
            .rows()
            .iter()
            .map(|row| {
                (
                    method_name_from_public_symbol(row.public_symbol()),
                    row.seam(),
                )
            })
            .collect(),
        associated_path_suffixes: registry
            .rows()
            .iter()
            .map(|row| {
                WorthQueryBoundaryAuditAssociatedPathSuffix::new(row.public_symbol(), row.seam())
            })
            .collect(),
    }
}

fn method_name_from_public_symbol(symbol: &'static str) -> &'static str {
    symbol
        .rsplit("::")
        .next()
        .expect("hard prohibition public symbol should include a method segment")
}

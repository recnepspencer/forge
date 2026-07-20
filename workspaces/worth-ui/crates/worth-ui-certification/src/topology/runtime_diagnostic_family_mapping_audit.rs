use std::collections::BTreeSet;
use std::path::Path;

use syn::visit::Visit;

use super::WorkspaceSourceInventory;

const MAPPING_ROOT: &str = "crates/worth-ui-runtime/src/runtime/diagnostics/mapping";
const EXPECTED_FAMILIES: &[&str] = &[
    "ActivationGate",
    "ActivationStaging",
    "ArtifactEquivalence",
    "CandidateAdmission",
    "CommittedAllocationActivation",
    "DiagnosticsProjection",
    "DurableStateReconciliation",
    "IdentityMatching",
    "ImpactNarrowing",
    "LaneAdmission",
    "PlanInspection",
    "PlanLowering",
    "QueryLiveRebind",
    "Reload",
    "ReplacementImpact",
];

pub fn audit_runtime_diagnostic_family_mapping(
    inventory: &WorkspaceSourceInventory,
) -> Vec<String> {
    let mut observed = BTreeSet::new();
    for source in inventory.rust_files_under(MAPPING_ROOT) {
        collect_family_variants(source.text(), source.relative_path(), &mut observed);
    }
    let expected = EXPECTED_FAMILIES
        .iter()
        .map(|family| (*family).to_string())
        .collect::<BTreeSet<_>>();
    if observed == expected {
        Vec::new()
    } else {
        vec![format!(
            "runtime diagnostic mapping families differ: observed {observed:?}, expected {expected:?}"
        )]
    }
}

fn collect_family_variants(text: &str, path: &Path, output: &mut BTreeSet<String>) {
    let syntax = syn::parse_file(text)
        .unwrap_or_else(|error| panic!("{} should parse: {error}", path.display()));
    DiagnosticFamilyVisitor { output }.visit_file(&syntax);
}

struct DiagnosticFamilyVisitor<'a> {
    output: &'a mut BTreeSet<String>,
}

impl<'ast> Visit<'ast> for DiagnosticFamilyVisitor<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments = &path.segments;
        if segments.len() >= 2
            && segments[segments.len() - 2].ident == "WorthUiRuntimeDiagnosticFamily"
        {
            self.output
                .insert(segments[segments.len() - 1].ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}

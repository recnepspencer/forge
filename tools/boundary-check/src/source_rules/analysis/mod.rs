//! Compiler-shaped source analysis shared by constitutional rule families.
//!
//! These modules form one mutually dependent semantic graph: module discovery,
//! reachability, alias closure, external dependency inspection, and authority
//! sealing. Keeping that graph here makes the cycle explicit and prevents the
//! `source_rules` facade from becoming a flat implementation directory.

pub(super) mod authority_sealing;
mod authority_sealing_surface;
mod authority_value_gate;
mod authority_value_gate_defs;
mod authority_value_gate_scan;
mod authority_value_identity;
mod blanket_launder;
mod callable_surface;
mod compiled_library_surface;
mod crate_modules;
mod dependency_authority;
pub(super) mod exported_ceremony_macro;
pub(super) mod external_public_reexport;
mod external_use_target;
mod forbidden_aliases;
mod forbidden_bound_scan;
mod module_source;
mod opaque_attributes;
mod path_dependencies;
mod public_reachability;
mod query_fence;
mod type_alias_reachability;

use crate::config::{QueryAudienceContract, SubworkspaceConfig};
use crate::diagnostics::Diagnostic;
use crate::snapshots::FacadeVocabularyAuthority;
use std::path::Path;

pub(crate) use compiled_library_surface::observe_compiled_library_surface;

pub(super) fn validate(
    root: &Path,
    subworkspaces: &[SubworkspaceConfig],
    query_audience: &QueryAudienceContract,
    facade_exports: &FacadeVocabularyAuthority<'_>,
) -> Result<Vec<Diagnostic>, String> {
    let mut diagnostics = Vec::new();
    let query_vocabulary = query_fence::QueryVocabulary::load(query_audience, facade_exports);
    let crates = crate_modules::discover_governed_crates(root, subworkspaces)?;
    for governed in crates {
        let module_graph = match crate_modules::parse_crate_modules(&governed) {
            Ok(graph) => graph,
            Err(error) => {
                diagnostics.push(authority_sealing::parse_failure_diagnostic(
                    &governed, error,
                ));
                continue;
            }
        };
        let additional_targets = match crate_modules::parse_additional_source_targets(&governed) {
            Ok(targets) => targets,
            Err(error) => {
                diagnostics.push(authority_sealing::parse_failure_diagnostic(
                    &governed, error,
                ));
                continue;
            }
        };
        let reachable = match public_reachability::externally_reachable_items(
            &module_graph,
            &governed.crate_root,
        ) {
            Ok(reachability) => reachability,
            Err(error) => {
                diagnostics.push(authority_sealing::dependency_authority_failure_diagnostic(
                    &governed, error,
                ));
                continue;
            }
        };
        diagnostics.extend(authority_sealing::enforce_authority_sealing(
            &governed,
            &module_graph,
            &reachable,
        ));
        diagnostics.extend(external_public_reexport::enforce_external_public_reexports(
            &governed,
            &module_graph,
            &reachable,
        ));
        diagnostics.extend(exported_ceremony_macro::enforce_exported_ceremony_macros(
            &governed,
            &module_graph,
            &reachable,
        ));
        diagnostics.extend(query_fence::enforce_query_fence(
            &governed,
            &module_graph,
            &reachable,
            &query_vocabulary,
        ));
        diagnostics.extend(query_fence::enforce_query_target_paths(
            &governed,
            &additional_targets,
            &query_vocabulary,
        ));
    }
    Ok(diagnostics)
}

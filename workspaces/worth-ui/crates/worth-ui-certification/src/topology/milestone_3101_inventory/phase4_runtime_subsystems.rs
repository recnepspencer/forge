use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

mod future_insertions;

const RUNTIME_ROOT: &str = "crates/worth-ui-runtime/src";
const REQUIRED_FAMILIES: &[&str] = &[
    "application",
    "graph",
    "inspection",
    "mounting",
    "observation",
    "planning",
    "session",
];

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    if ledger::text(document, "schema")?
        != "worth-ui.milestone-3.10.1.phase-4-runtime-subsystems.v2"
    {
        return Err("Phase 4 runtime subsystem ledger schema should be v2".to_owned());
    }
    if !ledger::text(document, "canonical_product_transition")?.contains("execute_mounted_frame") {
        return Err(
            "Phase 4 canonical product transition should be mounted-frame execution".into(),
        );
    }
    ledger::text(document, "certification_extension")?;

    let rows = ledger::tables(document, "family")?;
    let families = rows
        .iter()
        .map(|row| ledger::text(row, "name").map(str::to_owned))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let required = REQUIRED_FAMILIES
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<BTreeSet<_>>();
    if families != required {
        return Err(format!(
            "Phase 4 subsystem families differ: actual={families:?}, required={required:?}"
        ));
    }

    let mut dependencies = BTreeMap::new();
    for row in rows {
        audit_family(inventory, row, &families, &mut dependencies)?;
    }
    reject_family_cycles(&dependencies)?;
    future_insertions::audit(document, &families)?;
    audit_crate_decision(document)?;
    audit_owner_privacy(inventory)?;
    audit_source_boundaries(inventory)
}

fn audit_family(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    families: &BTreeSet<String>,
    dependencies: &mut BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    let name = ledger::text(row, "name")?;
    let owner_kind = ledger::text(row, "owner_kind")?;
    if !["state", "stateless-contract", "composition-root"].contains(&owner_kind) {
        return Err(format!("`{name}` has unknown owner kind `{owner_kind}`"));
    }
    for field in [
        "authority_in",
        "authority_out",
        "failure_owner",
        "preservation",
        "cost_lane",
        "future_insertion",
        "topology_rule",
    ] {
        ledger::text(row, field)?;
    }

    let owner_file = ledger::text(row, "owner_file")?;
    let owner_symbol = ledger::text(row, "owner_symbol")?;
    require_file(inventory, name, owner_file)?;
    if !inventory.text(owner_file).contains(owner_symbol) {
        return Err(format!(
            "`{name}` owner `{owner_file}` does not name `{owner_symbol}`"
        ));
    }
    for transition in ledger::strings(row, "transition_files")? {
        require_file(inventory, name, transition)?;
    }

    let allowed = ledger::strings(row, "allowed_family_dependencies")?
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if let Some(unknown) = allowed
        .iter()
        .find(|dependency| !families.contains(*dependency))
    {
        return Err(format!("`{name}` depends on unknown family `{unknown}`"));
    }
    if allowed.contains(name) {
        return Err(format!("`{name}` cannot depend on itself"));
    }
    dependencies.insert(name.to_owned(), allowed);
    Ok(())
}

fn require_file(
    inventory: &WorkspaceSourceInventory,
    family: &str,
    path: &str,
) -> Result<(), String> {
    if inventory.contains(path) {
        Ok(())
    } else {
        Err(format!("`{family}` authority file `{path}` is missing"))
    }
}

fn reject_family_cycles(dependencies: &BTreeMap<String, BTreeSet<String>>) -> Result<(), String> {
    let mut remaining = dependencies.clone();
    loop {
        let removable = remaining
            .iter()
            .filter(|(_, targets)| targets.iter().all(|target| !remaining.contains_key(target)))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        if removable.is_empty() {
            break;
        }
        for name in removable {
            remaining.remove(&name);
        }
    }
    if remaining.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Phase 4 subsystem dependency cycle remains: {:?}",
            remaining.keys().collect::<Vec<_>>()
        ))
    }
}

fn audit_crate_decision(document: &toml::Value) -> Result<(), String> {
    let rows = ledger::tables(document, "crate_decision")?;
    if rows.len() != 1 {
        return Err("Phase 4 should record exactly one all-family crate decision".into());
    }
    for field in [
        "scope",
        "decision",
        "independent_authority",
        "stable_directional_api",
        "dependency_effect",
        "compile_cost_effect",
        "cycle_analysis",
        "why_internal_is_sufficient",
    ] {
        ledger::text(&rows[0], field)?;
    }
    Ok(())
}

fn audit_owner_privacy(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for (path, symbol, composition_root) in [
        (
            "crates/worth-ui-runtime/src/runtime/session/application_state.rs",
            "WorthUiApplicationSessionState",
            false,
        ),
        (
            "crates/worth-ui-runtime/src/graph/snapshot/graph_snapshot.rs",
            "UiGraphSnapshot",
            false,
        ),
        (
            "crates/worth-ui-runtime/src/runtime/planning/allocation_planning/plan.rs",
            "WorthUiAllocationPlanning",
            false,
        ),
        (
            "crates/worth-ui-runtime/src/mounting/session_state.rs",
            "WorthUiMountedSessionState",
            false,
        ),
        (
            "crates/worth-ui-runtime/src/host_exchange/session_state.rs",
            "WorthUiHostExchangeSessionState",
            false,
        ),
        (
            "crates/worth-ui-runtime/src/facade/entry/active_application_session.rs",
            "WorthUiActiveApplicationSession",
            true,
        ),
    ] {
        let syntax = syn::parse_file(inventory.text(path))
            .map_err(|error| format!("`{path}` should parse: {error}"))?;
        let owner = syntax.items.iter().find_map(|item| match item {
            syn::Item::Struct(item) if item.ident == symbol => Some(item),
            _ => None,
        });
        let owner = owner.ok_or_else(|| format!("`{path}` is missing owner `{symbol}`"))?;
        let exposes_field = owner.fields.iter().any(|field| {
            if matches!(field.vis, syn::Visibility::Inherited) {
                return false;
            }
            if composition_root {
                return !matches!(
                    &field.vis,
                    syn::Visibility::Restricted(restricted)
                        if restricted.path.segments.last().is_some_and(|segment| segment.ident == "super")
                );
            }
            true
        });
        if exposes_field {
            return Err(format!("`{symbol}` exposes a subsystem state field"));
        }
    }
    Ok(())
}

fn audit_source_boundaries(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(RUNTIME_ROOT) {
        if is_support_or_test(source.relative_path()) {
            continue;
        }
        reject_source_boundary(source.relative_path(), source.text())?;
    }
    Ok(())
}

pub(super) fn reject_source_boundary(path: &Path, source: &str) -> Result<(), String> {
    let path = path.to_string_lossy().replace('\\', "/");
    if path.starts_with("crates/worth-ui-runtime/src/graph/") && source.contains("crate::mounting")
    {
        return Err(format!("`{path}` creates a graph-to-mounting reverse edge"));
    }
    if !path.contains("/facade/entry/") && whole_session_borrow(source) {
        return Err(format!("`{path}` borrows the whole active session"));
    }
    if path.contains("/facade/entry/") {
        for raw in [
            "UiMountedIdentityState",
            "UiMountedFrameRetentionCoordinator",
            "UiMountedPresentationCoordinator",
            "UiHostObservationReportValidation",
            "UiHostMeasurementAdmission",
        ] {
            if source.contains(raw) {
                return Err(format!("`{path}` reaches through a facade to raw `{raw}`"));
            }
        }
        if source.contains("mounted.identity.")
            || source.contains("mounted.retention.")
            || source.contains("mounted.presentation.")
        {
            return Err(format!("`{path}` mutates a mounted sibling field directly"));
        }
    }
    if path.contains("/inspection/")
        && source.contains("&mut crate::mounting::WorthUiMountedSessionState")
    {
        return Err(format!(
            "`{path}` gives inspection operational mutation authority"
        ));
    }
    Ok(())
}

fn whole_session_borrow(source: &str) -> bool {
    source.contains("session: &crate::facade::WorthUiActiveApplicationSession")
        || source.contains("session: &mut crate::facade::WorthUiActiveApplicationSession")
        || source.contains("session: &WorthUiActiveApplicationSession")
        || source.contains("session: &mut WorthUiActiveApplicationSession")
}

fn is_support_or_test(path: &Path) -> bool {
    let path = path.to_string_lossy().replace('\\', "/");
    path.contains("/certification_support/")
        || path.contains("/tests/")
        || path.ends_with("_tests.rs")
        || path.ends_with("_test_support.rs")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::reject_source_boundary;

    #[test]
    fn sibling_mutation_fixture_is_rejected() {
        let fixture = include_str!(
            "../../../tests/fixtures/topology_negative/milestone_3101_phase4_sibling_mutation.rs"
        );
        let error = reject_source_boundary(
            Path::new("crates/worth-ui-runtime/src/facade/entry/sibling_cheat.rs"),
            fixture,
        )
        .expect_err("sibling field mutation should fail");
        assert!(error.contains("sibling field"));
    }

    #[test]
    fn whole_session_graph_reader_fixture_is_rejected() {
        let fixture = include_str!(
            "../../../tests/fixtures/topology_negative/milestone_3101_phase4_whole_session.rs"
        );
        let error = reject_source_boundary(
            Path::new("crates/worth-ui-runtime/src/graph/whole_session_reader.rs"),
            fixture,
        )
        .expect_err("graph reader should require graph authority, not the whole session");
        assert!(error.contains("whole active session"));
    }

    #[test]
    fn thin_wrapper_fixture_is_rejected() {
        let fixture = include_str!(
            "../../../tests/fixtures/topology_negative/milestone_3101_phase4_thin_wrapper.rs"
        );
        let error = reject_source_boundary(
            Path::new("crates/worth-ui-runtime/src/facade/entry/thin_wrapper.rs"),
            fixture,
        )
        .expect_err("thin wrapper should not reach raw mounted owners");
        assert!(error.contains("reaches through a facade"));
    }

    #[test]
    fn graph_mounting_cycle_fixture_is_rejected() {
        let fixture = include_str!(
            "../../../tests/fixtures/topology_negative/milestone_3101_phase4_graph_mount_cycle.rs"
        );
        let error = reject_source_boundary(
            Path::new("crates/worth-ui-runtime/src/graph/mounting_cycle.rs"),
            fixture,
        )
        .expect_err("graph-to-mounting reverse edge should fail");
        assert!(error.contains("reverse edge"));
    }

    #[test]
    fn inspection_mutation_fixture_is_rejected() {
        let fixture = include_str!(
            "../../../tests/fixtures/topology_negative/milestone_3101_phase4_inspection_mutation.rs"
        );
        let error = reject_source_boundary(
            Path::new("crates/worth-ui-runtime/src/inspection/mounted_mutation.rs"),
            fixture,
        )
        .expect_err("inspection cannot borrow mounted mutation authority");
        assert!(error.contains("inspection operational mutation"));
    }
}

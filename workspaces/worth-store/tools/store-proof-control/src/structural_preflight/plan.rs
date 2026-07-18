use std::path::Path;
use std::process::Command;

use worth_store_test_support::structural_preflight::{
    PreflightInputScope, StructuralPredicate, StructuralPredicatePlan, StructuralPreflightPlan,
    StructuralPreflightRequest, StructuralToolDeclaration,
};

use crate::evidence::sha256_serialized;

use super::inputs::scope;

pub(super) fn build(
    forge_root: &Path,
    request: StructuralPreflightRequest,
) -> Result<StructuralPreflightPlan, String> {
    let toolchain = toolchain_identity(forge_root)?;
    let mut predicates = Vec::with_capacity(request.predicates.len());
    for predicate in &request.predicates {
        predicates.push(predicate_plan(forge_root, *predicate, &toolchain)?);
    }
    let mut plan = StructuralPreflightPlan {
        schema_version: 1,
        request,
        predicates,
        plan_identity: String::new(),
    };
    plan.plan_identity = sha256_serialized(&plan)?;
    Ok(plan)
}

fn predicate_plan(
    root: &Path,
    predicate: StructuralPredicate,
    toolchain: &str,
) -> Result<StructuralPredicatePlan, String> {
    use StructuralPredicate as Predicate;
    let (input_scopes, tool) = match predicate {
        Predicate::Boundary => (
            vec![boundary_scope(root)?],
            Some(boundary_tool(toolchain, "boundary")),
        ),
        Predicate::AgentContext => (
            vec![scope(
                root,
                "agent-context-authority",
                &[
                    "tools/agent-context",
                    "tools/boundary-check/config/road1.toml",
                    "crates",
                    "workspaces/worth-store/crates",
                ],
                &["rs", "toml", "md"],
            )?],
            Some(StructuralToolDeclaration {
                tool_identity: format!("agent-context::{toolchain}"),
                program: "cargo".to_owned(),
                arguments: vec![
                    "run".to_owned(),
                    "--quiet".to_owned(),
                    "--manifest-path".to_owned(),
                    "tools/agent-context/Cargo.toml".to_owned(),
                    "--".to_owned(),
                    "check".to_owned(),
                ],
                source_scope_identity: "agent-context-authority".to_owned(),
            }),
        ),
        Predicate::Inventory => (vec![inventory_scope(root)?], None),
        Predicate::Preservation => (
            vec![scope(
                root,
                "proof-preservation-authority",
                &[
                    "workspaces/worth-store/test-control",
                    "workspaces/worth-store/crates",
                ],
                &["json", "rs", "toml", "md"],
            )?],
            None,
        ),
        Predicate::Feature | Predicate::Dependency => (
            vec![manifest_scope(root)?],
            None,
        ),
        Predicate::LineCap => (
            vec![scope(
                root,
                "workspace-rust-line-cap-authority",
                &[
                    "scripts/ci/check_workspace_rust_line_caps.sh",
                    "scripts/ci/workspace_rust_line_cap_allowlist.txt",
                    "crates",
                    "workspaces/worth-ui/crates",
                    "workspaces/worth-store/crates",
                ],
                &["rs", "sh", "txt"],
            )?],
            Some(StructuralToolDeclaration {
                tool_identity: format!("workspace-rust-line-caps::{toolchain}"),
                program: "bash".to_owned(),
                arguments: vec!["scripts/ci/check_workspace_rust_line_caps.sh".to_owned()],
                source_scope_identity: "workspace-rust-line-cap-authority".to_owned(),
            }),
        ),
        Predicate::Naming => (
            vec![manifest_scope(root)?, boundary_scope(root)?],
            Some(boundary_tool(toolchain, "naming")),
        ),
        Predicate::AdmittedResidue => (
            vec![scope(
                root,
                "store-test-execution-residue",
                &[
                    "workspaces/worth-store/crates/worth-store-certification",
                    "workspaces/worth-store/crates/worth-store-physical-certification",
                    "workspaces/worth-store/crates/worth-store-test-support",
                    "workspaces/worth-store/tools/store-proof-control",
                ],
                &["rs", "toml"],
            )?],
            None,
        ),
    };
    Ok(StructuralPredicatePlan {
        predicate,
        input_scopes,
        tool,
    })
}

fn boundary_scope(root: &Path) -> Result<PreflightInputScope, String> {
    scope(
        root,
        "road1-boundary-authority",
        &[
            "Cargo.toml",
            "crates",
            "workspaces/worth-store",
            "tools/boundary-check",
            "cad/docs/worthy-foundations",
        ],
        &["rs", "toml", "md"],
    )
}

fn inventory_scope(root: &Path) -> Result<PreflightInputScope, String> {
    scope(
        root,
        "store-proof-inventory",
        &[
            "workspaces/worth-store/Cargo.toml",
            "workspaces/worth-store/Cargo.lock",
            "workspaces/worth-store/crates",
            "workspaces/worth-store/test-control",
        ],
        &["rs", "toml", "lock", "json", "md"],
    )
}

fn manifest_scope(root: &Path) -> Result<PreflightInputScope, String> {
    scope(
        root,
        "store-dependency-manifests",
        &["Cargo.toml", "workspaces/worth-store"],
        &["toml"],
    )
}

fn boundary_tool(toolchain: &str, projection: &str) -> StructuralToolDeclaration {
    StructuralToolDeclaration {
        tool_identity: format!("boundary-check::{projection}::{toolchain}"),
        program: "cargo".to_owned(),
        arguments: vec![
            "run".to_owned(),
            "--quiet".to_owned(),
            "--manifest-path".to_owned(),
            "tools/boundary-check/Cargo.toml".to_owned(),
            "--".to_owned(),
            "--root".to_owned(),
            ".".to_owned(),
        ],
        source_scope_identity: "road1-boundary-authority".to_owned(),
    }
}

fn toolchain_identity(root: &Path) -> Result<String, String> {
    let rustc = command_identity(root, "rustc", &["-Vv"])?;
    let cargo = command_identity(root, "cargo", &["-Vv"])?;
    sha256_serialized(&(rustc, cargo))
}

fn command_identity(root: &Path, program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not launch {program}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} identity failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

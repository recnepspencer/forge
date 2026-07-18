use std::path::Path;

use worth_store_test_support::structural_preflight::{
    PreflightInputScope, StructuralPredicate, StructuralPredicatePlan, StructuralPreflightPlan,
    StructuralPreflightRequest, StructuralToolDeclaration,
};

use crate::evidence::sha256_serialized;

use super::inputs::scope;
use super::version_probe::{self, ObservedProgramVersion};

struct RustToolchainIdentity {
    cargo: ObservedProgramVersion,
    identity: String,
}

pub(super) fn build(
    forge_root: &Path,
    request: StructuralPreflightRequest,
) -> Result<StructuralPreflightPlan, String> {
    let needs_rust_toolchain = request.predicates.iter().any(|predicate| {
        matches!(
            predicate,
            StructuralPredicate::Boundary
                | StructuralPredicate::AgentContext
                | StructuralPredicate::Naming
        )
    });
    let toolchain = needs_rust_toolchain
        .then(|| toolchain_identity(forge_root))
        .transpose()?;
    let mut predicates = Vec::with_capacity(request.predicates.len());
    for predicate in &request.predicates {
        predicates.push(predicate_plan(forge_root, *predicate, toolchain.as_ref())?);
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
    toolchain: Option<&RustToolchainIdentity>,
) -> Result<StructuralPredicatePlan, String> {
    use StructuralPredicate as Predicate;
    let (input_scopes, tool) = match predicate {
        Predicate::Boundary => (
            vec![boundary_scope(root)?],
            Some(boundary_tool(required_toolchain(toolchain)?, "boundary")?),
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
            Some(StructuralToolDeclaration::workspace_owned(
                format!("agent-context::{}", required_toolchain(toolchain)?.identity),
                &required_toolchain(toolchain)?.cargo.program_path,
                &required_toolchain(toolchain)?.identity,
                vec![
                    "run".to_owned(),
                    "--quiet".to_owned(),
                    "--manifest-path".to_owned(),
                    "tools/agent-context/Cargo.toml".to_owned(),
                    "--".to_owned(),
                    "check".to_owned(),
                ],
                "agent-context-authority",
                300_000,
                "single-process; inherited-memory-limit; output-cap-bytes=8388608; no-network-required",
            )?),
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
            Some(line_cap_tool(root)?),
        ),
        Predicate::Naming => (
            vec![manifest_scope(root)?, boundary_scope(root)?],
            Some(boundary_tool(required_toolchain(toolchain)?, "naming")?),
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

fn line_cap_tool(
    root: &Path,
) -> Result<StructuralToolDeclaration, String> {
    let bash = version_probe::observe(root, "bash", &["--version"])?;
    let version_identity = sha256_serialized(&bash)?;
    StructuralToolDeclaration::workspace_owned(
                format!("workspace-rust-line-caps::{version_identity}"),
                &bash.program_path,
                version_identity,
                vec!["scripts/ci/check_workspace_rust_line_caps.sh".to_owned()],
                "workspace-rust-line-cap-authority",
                120_000,
                "single-process; inherited-memory-limit; output-cap-bytes=8388608; no-network-required",
            )
}

fn required_toolchain(
    toolchain: Option<&RustToolchainIdentity>,
) -> Result<&RustToolchainIdentity, String> {
    toolchain.ok_or_else(|| "structural predicate omitted its Rust toolchain identity".to_owned())
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

fn boundary_tool(
    toolchain: &RustToolchainIdentity,
    projection: &str,
) -> Result<StructuralToolDeclaration, String> {
    StructuralToolDeclaration::workspace_owned(
        format!("boundary-check::{projection}::{}", toolchain.identity),
        &toolchain.cargo.program_path,
        &toolchain.identity,
        vec![
            "run".to_owned(),
            "--quiet".to_owned(),
            "--manifest-path".to_owned(),
            "tools/boundary-check/Cargo.toml".to_owned(),
            "--".to_owned(),
            "--root".to_owned(),
            ".".to_owned(),
        ],
        "road1-boundary-authority",
        300_000,
        "single-process; inherited-memory-limit; output-cap-bytes=8388608; no-network-required",
    )
}

fn toolchain_identity(root: &Path) -> Result<RustToolchainIdentity, String> {
    let cargo = version_probe::observe(root, "cargo", &["-Vv"])?;
    let rustc = version_probe::observe(root, "rustc", &["-Vv"])?;
    let identity = sha256_serialized(&(&cargo, &rustc))?;
    Ok(RustToolchainIdentity { cargo, identity })
}

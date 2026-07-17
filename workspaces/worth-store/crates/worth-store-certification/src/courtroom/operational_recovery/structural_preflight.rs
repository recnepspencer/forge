use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use sha2::{Digest, Sha256};

use super::S10ProofFoundationalAdoptionMatrix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10StructuralPreflightDenial {
    WorkspaceRootUnavailable(PathBuf),
    ToolLaunchFailed {
        tool: &'static str,
        message: String,
    },
    ToolRejected {
        tool: &'static str,
        status: Option<i32>,
        stderr: String,
    },
    ReverseFlowGateUnexpectedlyCompiled(&'static str),
    ReverseFlowGateMislocalized {
        gate: &'static str,
        missing_type: &'static str,
    },
    AdoptionGateMissing(&'static str),
    SourceReadFailed {
        path: PathBuf,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10StructuralPreflightEvidence {
    boundary_check_identity: [u8; 32],
    agent_context_identity: [u8; 32],
    reverse_flow_compile_identity: [u8; 32],
    adoption_matrix_identity: [u8; 32],
    source_tree_identity: [u8; 32],
    toolchain_identity: [u8; 32],
    evidence_identity: [u8; 32],
}

pub fn execute_s10_structural_preflight(
    workspace_root: &Path,
) -> Result<S10StructuralPreflightEvidence, S10StructuralPreflightDenial> {
    let root = workspace_root.canonicalize().map_err(|_| {
        S10StructuralPreflightDenial::WorkspaceRootUnavailable(workspace_root.to_path_buf())
    })?;
    let boundary = run_success(
        "boundary-check",
        &root,
        &[
            "run",
            "--quiet",
            "--manifest-path",
            "tools/boundary-check/Cargo.toml",
            "--",
            "--root",
            ".",
        ],
    )?;
    let context = run_success(
        "agent-context",
        &root,
        &[
            "run",
            "--quiet",
            "--manifest-path",
            "tools/agent-context/Cargo.toml",
            "--",
            "check",
        ],
    )?;
    let adoption = S10ProofFoundationalAdoptionMatrix::canonical();
    let reverse_flow_compile_identity = run_reverse_flow_gates(&root, &adoption)?;
    let boundary_check_identity = successful_output_identity("boundary-check", &boundary, &root);
    let agent_context_identity = successful_output_identity("agent-context", &context, &root);
    let adoption_matrix_identity = adoption.evidence_identity();
    let source_tree_identity = source_tree_identity(&root)?;
    let toolchain_identity = toolchain_identity(&root)?;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-structural-preflight-v1");
    digest.update(boundary_check_identity);
    digest.update(agent_context_identity);
    digest.update(reverse_flow_compile_identity);
    digest.update(adoption_matrix_identity);
    digest.update(source_tree_identity);
    digest.update(toolchain_identity);
    Ok(S10StructuralPreflightEvidence {
        boundary_check_identity,
        agent_context_identity,
        reverse_flow_compile_identity,
        adoption_matrix_identity,
        source_tree_identity,
        toolchain_identity,
        evidence_identity: digest.finalize().into(),
    })
}

impl S10StructuralPreflightEvidence {
    pub const fn boundary_check_identity(self) -> [u8; 32] {
        self.boundary_check_identity
    }
    pub const fn agent_context_identity(self) -> [u8; 32] {
        self.agent_context_identity
    }
    pub const fn reverse_flow_compile_identity(self) -> [u8; 32] {
        self.reverse_flow_compile_identity
    }
    pub const fn adoption_matrix_identity(self) -> [u8; 32] {
        self.adoption_matrix_identity
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
    pub const fn source_tree_identity(self) -> [u8; 32] {
        self.source_tree_identity
    }
    pub const fn toolchain_identity(self) -> [u8; 32] {
        self.toolchain_identity
    }
}

fn run_success(
    tool: &'static str,
    root: &Path,
    args: &[&str],
) -> Result<Output, S10StructuralPreflightDenial> {
    let output = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| S10StructuralPreflightDenial::ToolLaunchFailed {
            tool,
            message: error.to_string(),
        })?;
    if !output.status.success() {
        return Err(S10StructuralPreflightDenial::ToolRejected {
            tool,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(output)
}

fn run_reverse_flow_gates(
    root: &Path,
    adoption: &S10ProofFoundationalAdoptionMatrix,
) -> Result<[u8; 32], S10StructuralPreflightDenial> {
    let cases = reverse_flow_cases();
    for row in adoption.rows() {
        if !cases
            .iter()
            .any(|(gate, _, _)| *gate == row.reverse_flow_compile_gate())
        {
            return Err(S10StructuralPreflightDenial::AdoptionGateMissing(
                row.reverse_flow_compile_gate(),
            ));
        }
    }
    let case_root = root.join(
        "workspaces/worth-store/crates/worth-store-certification/tests/compile_fail/operational_recovery/cases/adoption_reverse_flow",
    );
    let target = root.join("workspaces/worth-store/target/s10-structural-preflight");
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-reverse-flow-compile-gates-v1");
    for (gate, source, authority) in cases {
        let output = Command::new(env!("CARGO"))
            .args(["check", "--quiet", "--bin", gate])
            .current_dir(&case_root)
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .map_err(|error| S10StructuralPreflightDenial::ToolLaunchFailed {
                tool: gate,
                message: error.to_string(),
            })?;
        if output.status.success() {
            return Err(S10StructuralPreflightDenial::ReverseFlowGateUnexpectedlyCompiled(gate));
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        for expected in [source, authority] {
            if !stderr.contains(expected) {
                return Err(S10StructuralPreflightDenial::ReverseFlowGateMislocalized {
                    gate,
                    missing_type: expected,
                });
            }
        }
        digest.update(gate.as_bytes());
        digest.update(output_identity(&output, root));
    }
    Ok(digest.finalize().into())
}

fn successful_output_identity(tool: &str, output: &Output, root: &Path) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-successful-tool-output-v1");
    digest.update(tool.as_bytes());
    digest.update(output_identity(output, root));
    digest.finalize().into()
}

fn output_identity(output: &Output, root: &Path) -> [u8; 32] {
    let root = root.to_string_lossy();
    let stdout = normalize_tool_output(&output.stdout, &root);
    let stderr = normalize_tool_output(&output.stderr, &root);
    let mut digest = Sha256::new();
    digest.update(output.status.code().unwrap_or(-1).to_be_bytes());
    digest.update((stdout.len() as u64).to_be_bytes());
    digest.update(stdout.as_bytes());
    digest.update((stderr.len() as u64).to_be_bytes());
    digest.update(stderr.as_bytes());
    digest.finalize().into()
}

fn normalize_tool_output(output: &[u8], root: &str) -> String {
    String::from_utf8_lossy(output)
        .replace(root, "<workspace>")
        .replace('\\', "/")
}

fn source_tree_identity(root: &Path) -> Result<[u8; 32], S10StructuralPreflightDenial> {
    let mut files = Vec::new();
    for relative in [
        "workspaces/worth-store/Cargo.toml",
        "workspaces/worth-store/Cargo.lock",
        "workspaces/worth-store/crates",
        "crates/worth-proof",
        "crates/worth-foundational",
        "tools/boundary-check",
        "tools/agent-context",
    ] {
        collect_source_files(root, &root.join(relative), &mut files)?;
    }
    files.sort();
    files.dedup();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-source-tree-v1");
    for path in files {
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let relative = relative.to_string_lossy().replace('\\', "/");
        let bytes = std::fs::read(&path).map_err(|error| {
            S10StructuralPreflightDenial::SourceReadFailed {
                path: path.clone(),
                message: error.to_string(),
            }
        })?;
        digest.update((relative.len() as u64).to_be_bytes());
        digest.update(relative.as_bytes());
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(digest.finalize().into())
}

fn collect_source_files(
    root: &Path,
    path: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), S10StructuralPreflightDenial> {
    if path.is_file() {
        if source_file(path) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    let entries = std::fs::read_dir(path).map_err(|error| {
        S10StructuralPreflightDenial::SourceReadFailed {
            path: path.to_path_buf(),
            message: error.to_string(),
        }
    })?;
    for entry in entries {
        let entry = entry.map_err(|error| S10StructuralPreflightDenial::SourceReadFailed {
            path: root.to_path_buf(),
            message: error.to_string(),
        })?;
        let child = entry.path();
        if child.file_name().is_some_and(|name| name == "target") {
            continue;
        }
        if child.is_dir() || source_file(&child) {
            collect_source_files(root, &child, files)?;
        }
    }
    Ok(())
}

fn source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("rs" | "toml" | "lock" | "cfg" | "tla")
    )
}

fn toolchain_identity(root: &Path) -> Result<[u8; 32], S10StructuralPreflightDenial> {
    let rustc = Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .current_dir(root)
        .output()
        .map_err(|error| S10StructuralPreflightDenial::ToolLaunchFailed {
            tool: "rustc-version",
            message: error.to_string(),
        })?;
    if !rustc.status.success() {
        return Err(S10StructuralPreflightDenial::ToolRejected {
            tool: "rustc-version",
            status: rustc.status.code(),
            stderr: String::from_utf8_lossy(&rustc.stderr).into_owned(),
        });
    }
    let cargo = Command::new(env!("CARGO"))
        .arg("--version")
        .current_dir(root)
        .output()
        .map_err(|error| S10StructuralPreflightDenial::ToolLaunchFailed {
            tool: "cargo-version",
            message: error.to_string(),
        })?;
    if !cargo.status.success() {
        return Err(S10StructuralPreflightDenial::ToolRejected {
            tool: "cargo-version",
            status: cargo.status.code(),
            stderr: String::from_utf8_lossy(&cargo.stderr).into_owned(),
        });
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-toolchain-v1");
    digest.update(output_identity(&rustc, root));
    digest.update(output_identity(&cargo, root));
    Ok(digest.finalize().into())
}

const fn reverse_flow_cases() -> [(&'static str, &'static str, &'static str); 6] {
    [
        (
            "shared_audit_record_cannot_construct_control_record",
            "OperationalAuditRecord",
            "OperationalControlRecord",
        ),
        (
            "terminal_export_cannot_construct_authorization",
            "OperationalEvidenceExport",
            "AuthorizedBackupRestorePlan",
        ),
        (
            "support_bundle_cannot_construct_operational_authority",
            "OperationalAuditSupportPayload",
            "ExecutionReadyRepair",
        ),
        (
            "forensic_bundle_cannot_construct_restore_source",
            "ForensicCustodyRecord",
            "ProductionRestoreAdmissibleBackupBundle",
        ),
        (
            "lineage_projection_cannot_mint_primary_serve_lease",
            "ReplicaPromotionReceipt",
            "PrimaryServeLease",
        ),
        (
            "counter_receipt_cannot_construct_execution_ready_plan",
            "OperationalCounterReceipt",
            "ExecutionReadyRepair",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "runs workspace constitution tools and six nested compile-fail gates"]
    fn real_workspace_preflight_binds_tools_sources_and_reverse_flow_gates() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
        let evidence = execute_s10_structural_preflight(&root).unwrap();
        assert_ne!(evidence.boundary_check_identity(), [0; 32]);
        assert_ne!(evidence.agent_context_identity(), [0; 32]);
        assert_ne!(evidence.reverse_flow_compile_identity(), [0; 32]);
        assert_ne!(evidence.source_tree_identity(), [0; 32]);
        assert_ne!(evidence.toolchain_identity(), [0; 32]);
    }
}

use crate::source::{
    WorthUiArtifactDependencyEdgeKind, WorthUiArtifactDependencyReport,
    WorthUiArtifactDependencyTarget, WorthUiArtifactHandle, WorthUiArtifactNodeKind,
};

pub(crate) fn digest_dependency_report(report: &WorthUiArtifactDependencyReport) -> u64 {
    fold_text(&dependency_report_digest_basis(report))
}

fn dependency_report_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let basis = report.basis();
    let graph = basis.dependency_graph();
    let impact_metadata = basis.impact_metadata();
    [
        dependency_edges_digest_basis(report),
        module_dependencies_digest_basis(report),
        subtree_digests_digest_basis(report),
        runtime_hooks_digest_basis(report),
        module_impacts_digest_basis(report),
        subtree_impacts_digest_basis(report),
        format!(
            "full_artifact_handle_count:{}",
            impact_metadata.full_artifact_handle_count()
        ),
        format!("edge_count:{}", graph.edges().len()),
    ]
    .join("|")
}

fn dependency_edges_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("edges:");
    for edge in report.basis().dependency_graph().edges() {
        digest_basis.push_str(&artifact_handle_digest_basis(edge.source()));
        digest_basis.push_str("->");
        digest_basis.push_str(&dependency_target_digest_basis(edge.target()));
        digest_basis.push(':');
        digest_basis.push_str(edge_kind_digest_basis(edge.kind()));
        digest_basis.push(';');
    }
    digest_basis
}

fn module_dependencies_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("module_dependencies:");
    for (module_id, dependencies) in report.basis().dependency_graph().module_dependencies() {
        digest_basis.push_str(module_id.as_str());
        digest_basis.push('=');
        for dependency in dependencies {
            digest_basis.push_str(dependency.as_str());
            digest_basis.push(',');
        }
        digest_basis.push(';');
    }
    digest_basis
}

fn subtree_digests_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("subtree_digests:");
    for (handle, digest) in report.basis().dependency_graph().subtree_digests() {
        digest_basis.push_str(&artifact_handle_digest_basis(handle));
        digest_basis.push('=');
        digest_basis.push_str(&digest.raw().to_string());
        digest_basis.push(';');
    }
    digest_basis
}

fn runtime_hooks_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("runtime_hooks:");
    for (handle, hooks) in report.basis().dependency_graph().runtime_hooks() {
        digest_basis.push_str(&artifact_handle_digest_basis(handle));
        digest_basis.push('=');
        for hook in hooks {
            digest_basis.push_str(&hook.digest_basis());
            digest_basis.push(',');
        }
        digest_basis.push(';');
    }
    digest_basis
}

fn module_impacts_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("module_impacts:");
    for (module_id, handles) in report.basis().impact_metadata().module_impacts() {
        digest_basis.push_str(module_id.as_str());
        digest_basis.push('=');
        for handle in handles {
            digest_basis.push_str(&artifact_handle_digest_basis(handle));
            digest_basis.push(',');
        }
        digest_basis.push(';');
    }
    digest_basis
}

fn subtree_impacts_digest_basis(report: &WorthUiArtifactDependencyReport) -> String {
    let mut digest_basis = String::from("subtree_impacts:");
    for (handle, impacted_handles) in report.basis().impact_metadata().subtree_impacts() {
        digest_basis.push_str(&artifact_handle_digest_basis(handle));
        digest_basis.push('=');
        for impacted_handle in impacted_handles {
            digest_basis.push_str(&artifact_handle_digest_basis(impacted_handle));
            digest_basis.push(',');
        }
        digest_basis.push(';');
    }
    digest_basis
}

fn dependency_target_digest_basis(target: &WorthUiArtifactDependencyTarget) -> String {
    match target {
        WorthUiArtifactDependencyTarget::Module(module_id) => {
            format!("module:{}", module_id.as_str())
        }
        WorthUiArtifactDependencyTarget::Artifact(handle) => {
            format!("artifact:{}", artifact_handle_digest_basis(handle))
        }
        WorthUiArtifactDependencyTarget::RuntimeHook(hook) => {
            format!("runtime-hook:{}", hook.digest_basis())
        }
    }
}

fn artifact_handle_digest_basis(handle: &WorthUiArtifactHandle) -> String {
    format!(
        "{}:{}:{}",
        artifact_node_kind_digest_basis(handle.kind()),
        handle.module_id().as_str(),
        handle.node_index()
    )
}

fn artifact_node_kind_digest_basis(kind: WorthUiArtifactNodeKind) -> &'static str {
    match kind {
        WorthUiArtifactNodeKind::Import => "import",
        WorthUiArtifactNodeKind::Component => "component",
        WorthUiArtifactNodeKind::Surface => "surface",
        WorthUiArtifactNodeKind::Binding => "binding",
        WorthUiArtifactNodeKind::Token => "token",
    }
}

fn edge_kind_digest_basis(kind: WorthUiArtifactDependencyEdgeKind) -> &'static str {
    match kind {
        WorthUiArtifactDependencyEdgeKind::ModuleImport => "module-import",
        WorthUiArtifactDependencyEdgeKind::MosaicMount => "mosaic-mount",
        WorthUiArtifactDependencyEdgeKind::RuntimeHook => "runtime-hook",
    }
}

fn fold_text(text: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}

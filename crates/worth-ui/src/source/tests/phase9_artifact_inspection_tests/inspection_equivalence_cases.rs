use crate::source::WorthUiArtifactSourceOrigin;

use super::inspection_fixture_support::{
    equivalent_file_authored_inspector_module_source, equivalent_file_authored_main_module_source,
    equivalent_rust_authored_modules, file_authored_inspection_subject,
    inspection_semantic_summary, rust_inspection_subject_from_modules,
};

#[test]
fn provenance_replay_is_deterministic() {
    let (first_artifact, _, first_inspection, first_metrics) =
        rust_inspection_subject_from_modules(equivalent_rust_authored_modules());
    let (second_artifact, _, second_inspection, second_metrics) =
        rust_inspection_subject_from_modules(equivalent_rust_authored_modules());

    assert_eq!(first_artifact, second_artifact);
    assert_eq!(first_inspection, second_inspection);
    assert_eq!(first_metrics, second_metrics);
}

#[test]
fn inspection_does_not_require_rust_control_flow_archaeology() {
    let (rust_artifact, _, rust_inspection, _) =
        rust_inspection_subject_from_modules(equivalent_rust_authored_modules());
    let (file_artifact, _, file_inspection) = file_authored_inspection_subject(
        equivalent_file_authored_main_module_source(),
        equivalent_file_authored_inspector_module_source(),
    );

    assert!(rust_artifact.equivalent_shape(&file_artifact));
    assert_eq!(
        inspection_semantic_summary(&rust_inspection),
        inspection_semantic_summary(&file_inspection)
    );
    assert!(rust_inspection
        .handles()
        .iter()
        .zip(file_inspection.handles().iter())
        .any(|(rust_handle, file_handle)| {
            matches!(
                (
                    rust_inspection
                        .node(rust_handle)
                        .expect("rust inspection node")
                        .source_origin(),
                    file_inspection
                        .node(file_handle)
                        .expect("file inspection node")
                        .source_origin(),
                ),
                (
                    WorthUiArtifactSourceOrigin::RustAuthoredDeclaration { .. },
                    WorthUiArtifactSourceOrigin::ParsedSourceDeclaration { .. },
                )
            )
        }));
}

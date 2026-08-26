use std::sync::Arc;

use worth_server::{
    CompatHttpSurface, WorthNativeSurface, WorthServer, WorthServerBuildError,
    WorthServerProductAdapterCertificationCode, WorthServerProductApplicationAdapterRegistration,
    WorthServerProductOperationBasisKind, WorthServerProductOperationDeclaration,
    WorthServerProductOperationErrorMaps, WorthServerProductOperationSupportSnapshot,
};

#[path = "support/product_adapter_phase_nine/fixture.rs"]
mod fixture;

use fixture::{base_config, result_contract, EditorAdapter};

#[test]
fn product_adapter_registration_rejects_incomplete_authority_or_basis_contract() {
    let result = build_broken_registration_server(
        WorthServerProductOperationDeclaration::product_mutation(
            "product_editor.apply",
            "product-editor.apply.v1",
            result_contract("product-editor.apply.result.v1"),
            WorthServerProductOperationBasisKind::DurableProductDerived,
            WorthServerProductOperationSupportSnapshot::production_admitted(" "),
            "",
        )
        .with_error_map(WorthServerProductOperationErrorMaps::passthrough()),
    );
    assert_registration_code(
        result,
        WorthServerProductAdapterCertificationCode::BlankSupportSnapshotRow,
    );
}

#[test]
fn product_adapter_registration_rejects_missing_error_map_explicitly() {
    let result =
        build_broken_registration_server(WorthServerProductOperationDeclaration::product_read(
            "product_editor.render",
            "product-editor.render.v1",
            result_contract("product-editor.render.result.v1"),
            WorthServerProductOperationBasisKind::DurableProductDerived,
            WorthServerProductOperationSupportSnapshot::production_admitted("render-supported"),
        ));
    assert_registration_code(
        result,
        WorthServerProductAdapterCertificationCode::MissingErrorMap,
    );
}

#[test]
fn product_adapter_registration_rejects_blank_schema_identity_explicitly() {
    let result = build_broken_registration_server(
        WorthServerProductOperationDeclaration::product_read(
            "product_editor.render",
            " ",
            result_contract("product-editor.render.result.v1"),
            WorthServerProductOperationBasisKind::DurableProductDerived,
            WorthServerProductOperationSupportSnapshot::production_admitted("render-supported"),
        )
        .with_error_map(WorthServerProductOperationErrorMaps::passthrough()),
    );
    assert_registration_code(
        result,
        WorthServerProductAdapterCertificationCode::BlankPayloadSchemaIdentity,
    );
}

#[test]
fn primary_graph_application_registration_requires_its_query_owner() {
    let result = build_broken_registration_server(
        WorthServerProductOperationDeclaration::product_read(
            "workflow_editor.render",
            "workflow-editor.render.v1",
            result_contract("workflow-editor.render.result.v1"),
            WorthServerProductOperationBasisKind::PrimaryGraphApplication,
            WorthServerProductOperationSupportSnapshot::production_admitted(
                "workflow-editor-render-supported",
            ),
        )
        .with_error_map(WorthServerProductOperationErrorMaps::passthrough()),
    );
    assert_registration_code(
        result,
        WorthServerProductAdapterCertificationCode::MissingQueryApplicationReadinessProvider,
    );
}

fn build_broken_registration_server(
    declaration: WorthServerProductOperationDeclaration,
) -> Result<WorthServer, WorthServerBuildError> {
    WorthServer::builder()
        .with_config(base_config())
        .register_operations(worth_server::WorthServerOperationRegistration::phase_two_defaults())
        .register_surface(WorthNativeSurface::enabled())
        .register_surface(CompatHttpSurface::phase_one_enabled())
        .register_product_adapter(
            WorthServerProductApplicationAdapterRegistration::new(
                "broken-editor",
                Arc::new(EditorAdapter::default()),
            )
            .with_operation(declaration),
        )
        .build()
}

fn assert_registration_code(
    result: Result<WorthServer, WorthServerBuildError>,
    expected_code: WorthServerProductAdapterCertificationCode,
) {
    match result {
        Err(WorthServerBuildError::InvalidProductAdapterRegistry(error)) => {
            assert_eq!(error.certification_code(), Some(expected_code));
        }
        other => panic!("expected product adapter registration failure, got {other:?}"),
    }
}

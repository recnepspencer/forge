use worth_server::{WorthServerOperationFamily, WorthServerRouteAssemblyError};
use serde_json::json;

#[path = "support/route_assembly_phase_twelve/fixture.rs"]
mod fixture;
#[path = "support/route_assembly_phase_twelve/request_driver.rs"]
mod request_driver;

use fixture::{build_server, direct_mutation, direct_read};
use request_driver::WorthServerRouteHttpTestDriver;

#[tokio::test]
async fn declared_read_route_and_direct_facade_share_operation_plan() {
    let server = build_server();
    let direct = direct_read(&server);
    let driver = WorthServerRouteHttpTestDriver::new(&server);

    let response = driver
        .get(
            "/compat/reads/product_editor.render?basis=basis:7",
            &[
                ("x-principal-id", "principal-7"),
                ("x-tenant-id", "tenant-a"),
                ("x-workspace-id", "workspace-42"),
                ("x-branch-id", "branch-9"),
            ],
        )
        .await;

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert_eq!(response.route_kind(), Some("product_operation"));
    assert_eq!(response.semantic_runtime_entered(), Some(true));
    assert_eq!(response.operation_name(), Some("product_editor.render"));
    assert_eq!(
        response.plan_digest(),
        direct.plan().map(|plan| plan.canonical_digest())
    );
    assert_eq!(
        response.scheduler_lane(),
        direct
            .scheduler_admission()
            .map(|admission| admission.scheduler_lane())
    );
}

#[tokio::test]
async fn declared_mutation_route_and_direct_facade_share_operation_plan() {
    let server = build_server();
    let (product_session, direct) = direct_mutation(&server);
    let driver = WorthServerRouteHttpTestDriver::new(&server);

    let response = driver
        .post_json(
            "/compat/mutations/product_editor.apply?basis=basis:7",
            &[
                ("x-principal-id", "principal-7"),
                ("x-tenant-id", "tenant-a"),
                ("x-workspace-id", "workspace-42"),
                ("x-branch-id", "branch-9"),
                ("x-product-session-id", product_session.identity().as_str()),
            ],
            &json!({ "title": "Rename" }),
        )
        .await;

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert_eq!(response.route_kind(), Some("product_operation"));
    assert_eq!(response.semantic_runtime_entered(), Some(true));
    assert_eq!(response.operation_name(), Some("product_editor.apply"));
    assert_eq!(
        response.plan_digest(),
        direct.plan().map(|plan| plan.canonical_digest())
    );
    assert_eq!(
        response.scheduler_lane(),
        direct
            .scheduler_admission()
            .map(|admission| admission.scheduler_lane())
    );
}

#[test]
fn route_inventory_explains_every_served_path() {
    let server = build_server();
    let inventory = server.route_inventory();
    let mut actual_routes: Vec<(String, String)> = inventory
        .rows()
        .iter()
        .map(|row| (row.method().to_string(), row.path().to_string()))
        .collect();
    actual_routes.sort();

    let expected_routes = vec![
        (
            "GET".to_string(),
            "/compat/reads/product_editor.available_actions".to_string(),
        ),
        (
            "GET".to_string(),
            "/compat/reads/product_editor.render".to_string(),
        ),
        (
            "GET".to_string(),
            "/compat/reads/product_editor.select".to_string(),
        ),
        (
            "POST".to_string(),
            "/compat/mutations/product_editor.apply".to_string(),
        ),
        (
            "POST".to_string(),
            "/compat/mutations/product_editor.finalize".to_string(),
        ),
        (
            "POST".to_string(),
            "/compat/mutations/product_session.close".to_string(),
        ),
        (
            "POST".to_string(),
            "/compat/mutations/product_session.open_mutation".to_string(),
        ),
        (
            "POST".to_string(),
            "/compat/mutations/product_session.open_preview".to_string(),
        ),
        ("GET".to_string(), "/healthz".to_string()),
        ("GET".to_string(), "/metrics".to_string()),
        ("OPTIONS".to_string(), "/compat/preflight".to_string()),
        ("GET".to_string(), "/openapi.json".to_string()),
    ];
    let mut expected_routes = expected_routes;
    expected_routes.sort();

    assert_eq!(actual_routes, expected_routes);
    for row in inventory.rows() {
        assert_eq!(
            row.surface_family(),
            worth_server::WorthServerSurfaceFamily::CompatHttp
        );
        assert!(!row.diagnostics_policy().is_empty());
        assert!(!row.evidence_policy().is_empty());
        if row.operational_label().is_none() {
            assert!(row.operation_family().is_some());
            assert!(row.operation_name().is_some());
            assert!(row.payload_schema_identity().is_some());
            assert!(row.support_row().is_some());
        }
    }
}

#[test]
fn duplicate_or_ambiguous_operation_route_fails_server_assembly() {
    let result = worth_server::WorthServer::builder()
        .with_config(fixture::base_config())
        .register_surface(worth_server::surfaces::WorthNativeSurface::enabled())
        .register_surface(worth_server::surfaces::CompatHttpSurface::phase_one_enabled())
        .register_operation(
            worth_server::WorthServerOperationRegistration::enabled(
                WorthServerOperationFamily::ProductApplicationRead,
            )
            .exposed_on([
                worth_server::WorthServerSurfaceFamily::WorthNative,
                worth_server::WorthServerSurfaceFamily::CompatHttp,
            ])
            .admit_operation_names(["product_editor.allowed_only"]),
        )
        .register_operation(
            worth_server::WorthServerOperationRegistration::enabled(
                WorthServerOperationFamily::ProductApplicationMutation,
            )
            .exposed_on([
                worth_server::WorthServerSurfaceFamily::WorthNative,
                worth_server::WorthServerSurfaceFamily::CompatHttp,
            ]),
        )
        .register_operation(
            worth_server::WorthServerOperationRegistration::enabled(
                WorthServerOperationFamily::ProductSessionCoordination,
            )
            .exposed_on([
                worth_server::WorthServerSurfaceFamily::WorthNative,
                worth_server::WorthServerSurfaceFamily::CompatHttp,
            ]),
        )
        .register_product_adapter(fixture::editor_registration())
        .build();

    match result {
        Err(worth_server::WorthServerBuildError::InvalidRouteAssembly(error)) => {
            assert_eq!(
                error,
                WorthServerRouteAssemblyError::OperationNameNotAdmitted {
                    family: WorthServerOperationFamily::ProductApplicationRead,
                    operation_name: "product_editor.available_actions".to_string(),
                }
            );
        }
        other => panic!("expected operation-name admission failure, got {other:?}"),
    }
}

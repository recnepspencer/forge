use std::collections::BTreeMap;

use crate::{
    CompatHttpSurfaceRoot, WorthServerCompatHttpRouteFamily, WorthServerCompatibilityFacade,
    WorthServerOperationFamily, WorthServerOperationInventory, WorthServerOperationRegistry,
    WorthServerProductAdapterRegistry,
};

use super::{
    assembly_error::WorthServerRouteAssemblyError,
    declared_route::WorthServerDeclaredRoute,
    execution_bridge::WorthServerRouteExecutionBridge,
    inventory::{WorthServerRouteInventory, WorthServerRouteInventoryRow},
    operational_route::{WorthServerOperationalRoute, WorthServerOperationalRouteKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRouteAssembly {
    declared_routes: Vec<WorthServerDeclaredRoute>,
    operational_routes: Vec<WorthServerOperationalRoute>,
    inventory: WorthServerRouteInventory,
}

impl WorthServerRouteAssembly {
    pub(crate) fn assemble(
        compat_http_root: &CompatHttpSurfaceRoot,
        operation_registry: &WorthServerOperationRegistry,
        product_adapter_registry: &WorthServerProductAdapterRegistry,
    ) -> Result<Self, WorthServerRouteAssemblyError> {
        let mut declared_routes = Vec::new();
        let mut route_keys = BTreeMap::new();
        if compat_http_root.capabilities().is_registered()
            && !compat_http_root.capabilities().is_disabled()
        {
            append_declared_product_routes(
                &mut declared_routes,
                &mut route_keys,
                compat_http_root,
                operation_registry,
                product_adapter_registry,
            )?;
            append_session_coordination_routes(
                &mut declared_routes,
                &mut route_keys,
                compat_http_root,
                operation_registry,
            )?;
        }
        let operational_routes = operational_routes();
        let inventory = WorthServerRouteInventory::new(
            declared_routes
                .iter()
                .map(|route| {
                    WorthServerRouteInventoryRow::semantic(
                        route.method(),
                        route.path(),
                        route.operation_family(),
                        route.operation_name(),
                        route.payload_schema_identity(),
                        route.support_row(),
                        route.diagnostics_policy(),
                        route.response_transform(),
                        route.evidence_policy(),
                    )
                })
                .chain(operational_routes.iter().map(|route| {
                    WorthServerRouteInventoryRow::operational(
                        route.method(),
                        route.path(),
                        format!("{:?}", route.kind()),
                    )
                }))
                .collect(),
        );
        Ok(Self {
            declared_routes,
            operational_routes,
            inventory,
        })
    }

    pub fn declared_routes(&self) -> &[WorthServerDeclaredRoute] {
        &self.declared_routes
    }

    pub fn operational_routes(&self) -> &[WorthServerOperationalRoute] {
        &self.operational_routes
    }

    pub fn inventory(&self) -> &WorthServerRouteInventory {
        &self.inventory
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerOperationRouter {
    assembly: WorthServerRouteAssembly,
    compat_http: WorthServerCompatibilityFacade,
}

impl WorthServerOperationRouter {
    pub(crate) fn new(
        assembly: WorthServerRouteAssembly,
        compat_http: WorthServerCompatibilityFacade,
    ) -> Self {
        Self {
            assembly,
            compat_http,
        }
    }

    pub fn inventory(&self) -> &WorthServerRouteInventory {
        self.assembly.inventory()
    }

    pub fn bridge_for(&self, method: &str, path: &str) -> Option<WorthServerRouteExecutionBridge> {
        self.assembly
            .declared_routes()
            .iter()
            .find(|route| route.method() == method && route.path() == path)
            .cloned()
            .map(|route| WorthServerRouteExecutionBridge::semantic(route, self.compat_http.clone()))
            .or_else(|| {
                self.assembly
                    .operational_routes()
                    .iter()
                    .find(|route| route.method() == method && route.path() == path)
                    .cloned()
                    .map(|route| {
                        WorthServerRouteExecutionBridge::operational(
                            route,
                            self.compat_http.clone(),
                        )
                    })
            })
    }
}

fn append_declared_product_routes(
    declared_routes: &mut Vec<WorthServerDeclaredRoute>,
    route_keys: &mut BTreeMap<(String, String), ()>,
    compat_http_root: &CompatHttpSurfaceRoot,
    operation_registry: &WorthServerOperationRegistry,
    product_adapter_registry: &WorthServerProductAdapterRegistry,
) -> Result<(), WorthServerRouteAssemblyError> {
    for declaration in product_adapter_registry.declarations() {
        let route = match declaration.operation_family() {
            WorthServerOperationFamily::ProductApplicationRead => {
                require_route_family(
                    compat_http_root,
                    WorthServerCompatHttpRouteFamily::Read,
                    declaration.operation_name(),
                )?;
                WorthServerDeclaredRoute::new(
                    "GET",
                    format!("/compat/reads/{}", declaration.operation_name()),
                    WorthServerCompatHttpRouteFamily::Read,
                    declaration.operation_family(),
                    declaration.operation_name(),
                    declaration.payload_schema_identity(),
                    declaration.support_snapshot().support_row(),
                )
            }
            WorthServerOperationFamily::ProductApplicationMutation => {
                require_route_family(
                    compat_http_root,
                    WorthServerCompatHttpRouteFamily::Mutation,
                    declaration.operation_name(),
                )?;
                WorthServerDeclaredRoute::new(
                    "POST",
                    format!("/compat/mutations/{}", declaration.operation_name()),
                    WorthServerCompatHttpRouteFamily::Mutation,
                    declaration.operation_family(),
                    declaration.operation_name(),
                    declaration.payload_schema_identity(),
                    declaration.support_snapshot().support_row(),
                )
            }
            WorthServerOperationFamily::ProductSessionCoordination => continue,
            _ => continue,
        };
        require_unique_route(route_keys, route.method(), route.path())?;
        operation_registry
            .admit_operation_name(route.operation_family(), route.operation_name())
            .map_err(
                |_| WorthServerRouteAssemblyError::OperationNameNotAdmitted {
                    family: route.operation_family(),
                    operation_name: route.operation_name().to_string(),
                },
            )?;
        declared_routes.push(route);
    }
    Ok(())
}

fn append_session_coordination_routes(
    declared_routes: &mut Vec<WorthServerDeclaredRoute>,
    route_keys: &mut BTreeMap<(String, String), ()>,
    compat_http_root: &CompatHttpSurfaceRoot,
    operation_registry: &WorthServerOperationRegistry,
) -> Result<(), WorthServerRouteAssemblyError> {
    let operation_inventory: WorthServerOperationInventory = operation_registry.inventory();
    let Some(coordination_row) = operation_inventory.rows().iter().find(|row| {
        row.family() == WorthServerOperationFamily::ProductSessionCoordination && row.enabled()
    }) else {
        return Ok(());
    };
    if !coordination_row
        .exposed_surfaces()
        .contains(&crate::WorthServerSurfaceFamily::CompatHttp)
    {
        return Ok(());
    }
    require_route_family(
        compat_http_root,
        WorthServerCompatHttpRouteFamily::Mutation,
        "product_session.open_mutation",
    )?;
    for operation_name in [
        "product_session.open_preview",
        "product_session.open_mutation",
        "product_session.close",
    ] {
        let route = WorthServerDeclaredRoute::new(
            "POST",
            format!("/compat/mutations/{operation_name}"),
            WorthServerCompatHttpRouteFamily::Mutation,
            WorthServerOperationFamily::ProductSessionCoordination,
            operation_name,
            "worth-server-product-session.v1",
            "product-session",
        );
        require_unique_route(route_keys, route.method(), route.path())?;
        operation_registry
            .admit_operation_name(route.operation_family(), route.operation_name())
            .map_err(
                |_| WorthServerRouteAssemblyError::OperationNameNotAdmitted {
                    family: route.operation_family(),
                    operation_name: route.operation_name().to_string(),
                },
            )?;
        declared_routes.push(route);
    }
    Ok(())
}

fn require_route_family(
    compat_http_root: &CompatHttpSurfaceRoot,
    route_family: WorthServerCompatHttpRouteFamily,
    operation_name: &str,
) -> Result<(), WorthServerRouteAssemblyError> {
    if compat_http_root.route_families().contains(route_family) {
        return Ok(());
    }
    match route_family {
        WorthServerCompatHttpRouteFamily::Read => Err(
            WorthServerRouteAssemblyError::MissingCompatReadRouteFamily {
                operation_name: operation_name.to_string(),
            },
        ),
        WorthServerCompatHttpRouteFamily::Mutation => Err(
            WorthServerRouteAssemblyError::MissingCompatMutationRouteFamily {
                operation_name: operation_name.to_string(),
            },
        ),
        _ => Ok(()),
    }
}

fn require_unique_route(
    route_keys: &mut BTreeMap<(String, String), ()>,
    method: &str,
    path: &str,
) -> Result<(), WorthServerRouteAssemblyError> {
    let key = (method.to_string(), path.to_string());
    if route_keys.insert(key.clone(), ()).is_some() {
        return Err(WorthServerRouteAssemblyError::DuplicateMethodPath {
            method: key.0,
            path: key.1,
        });
    }
    Ok(())
}

fn operational_routes() -> Vec<WorthServerOperationalRoute> {
    vec![
        WorthServerOperationalRoute::new(
            WorthServerOperationalRouteKind::Health,
            "GET",
            "/healthz",
        ),
        WorthServerOperationalRoute::new(
            WorthServerOperationalRouteKind::Metrics,
            "GET",
            "/metrics",
        ),
        WorthServerOperationalRoute::new(
            WorthServerOperationalRouteKind::Preflight,
            "OPTIONS",
            "/compat/preflight",
        ),
        WorthServerOperationalRoute::new(
            WorthServerOperationalRouteKind::DocsExport,
            "GET",
            "/openapi.json",
        ),
    ]
}

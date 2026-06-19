use std::collections::BTreeMap;

use crate::{
    CompatHttpSurfaceRoot, ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityFacade,
    ForgeServerOperationFamily, ForgeServerOperationInventory, ForgeServerOperationRegistry,
    ForgeServerProductAdapterRegistry,
};

use super::{
    assembly_error::ForgeServerRouteAssemblyError,
    declared_route::ForgeServerDeclaredRoute,
    execution_bridge::ForgeServerRouteExecutionBridge,
    inventory::{ForgeServerRouteInventory, ForgeServerRouteInventoryRow},
    operational_route::{ForgeServerOperationalRoute, ForgeServerOperationalRouteKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRouteAssembly {
    declared_routes: Vec<ForgeServerDeclaredRoute>,
    operational_routes: Vec<ForgeServerOperationalRoute>,
    inventory: ForgeServerRouteInventory,
}

impl ForgeServerRouteAssembly {
    pub(crate) fn assemble(
        compat_http_root: &CompatHttpSurfaceRoot,
        operation_registry: &ForgeServerOperationRegistry,
        product_adapter_registry: &ForgeServerProductAdapterRegistry,
    ) -> Result<Self, ForgeServerRouteAssemblyError> {
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
        let inventory = ForgeServerRouteInventory::new(
            declared_routes
                .iter()
                .map(|route| {
                    ForgeServerRouteInventoryRow::semantic(
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
                    ForgeServerRouteInventoryRow::operational(
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

    pub fn declared_routes(&self) -> &[ForgeServerDeclaredRoute] {
        &self.declared_routes
    }

    pub fn operational_routes(&self) -> &[ForgeServerOperationalRoute] {
        &self.operational_routes
    }

    pub fn inventory(&self) -> &ForgeServerRouteInventory {
        &self.inventory
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerOperationRouter {
    assembly: ForgeServerRouteAssembly,
    compat_http: ForgeServerCompatibilityFacade,
}

impl ForgeServerOperationRouter {
    pub(crate) fn new(
        assembly: ForgeServerRouteAssembly,
        compat_http: ForgeServerCompatibilityFacade,
    ) -> Self {
        Self {
            assembly,
            compat_http,
        }
    }

    pub fn inventory(&self) -> &ForgeServerRouteInventory {
        self.assembly.inventory()
    }

    pub fn bridge_for(&self, method: &str, path: &str) -> Option<ForgeServerRouteExecutionBridge> {
        self.assembly
            .declared_routes()
            .iter()
            .find(|route| route.method() == method && route.path() == path)
            .cloned()
            .map(|route| ForgeServerRouteExecutionBridge::semantic(route, self.compat_http.clone()))
            .or_else(|| {
                self.assembly
                    .operational_routes()
                    .iter()
                    .find(|route| route.method() == method && route.path() == path)
                    .cloned()
                    .map(|route| {
                        ForgeServerRouteExecutionBridge::operational(
                            route,
                            self.compat_http.clone(),
                        )
                    })
            })
    }
}

fn append_declared_product_routes(
    declared_routes: &mut Vec<ForgeServerDeclaredRoute>,
    route_keys: &mut BTreeMap<(String, String), ()>,
    compat_http_root: &CompatHttpSurfaceRoot,
    operation_registry: &ForgeServerOperationRegistry,
    product_adapter_registry: &ForgeServerProductAdapterRegistry,
) -> Result<(), ForgeServerRouteAssemblyError> {
    for declaration in product_adapter_registry.declarations() {
        let route = match declaration.operation_family() {
            ForgeServerOperationFamily::ProductApplicationRead => {
                require_route_family(
                    compat_http_root,
                    ForgeServerCompatHttpRouteFamily::Read,
                    declaration.operation_name(),
                )?;
                ForgeServerDeclaredRoute::new(
                    "GET",
                    format!("/compat/reads/{}", declaration.operation_name()),
                    ForgeServerCompatHttpRouteFamily::Read,
                    declaration.operation_family(),
                    declaration.operation_name(),
                    declaration.payload_schema_identity(),
                    declaration.support_snapshot().support_row(),
                )
            }
            ForgeServerOperationFamily::ProductApplicationMutation => {
                require_route_family(
                    compat_http_root,
                    ForgeServerCompatHttpRouteFamily::Mutation,
                    declaration.operation_name(),
                )?;
                ForgeServerDeclaredRoute::new(
                    "POST",
                    format!("/compat/mutations/{}", declaration.operation_name()),
                    ForgeServerCompatHttpRouteFamily::Mutation,
                    declaration.operation_family(),
                    declaration.operation_name(),
                    declaration.payload_schema_identity(),
                    declaration.support_snapshot().support_row(),
                )
            }
            ForgeServerOperationFamily::ProductSessionCoordination => continue,
            _ => continue,
        };
        require_unique_route(route_keys, route.method(), route.path())?;
        operation_registry
            .admit_operation_name(route.operation_family(), route.operation_name())
            .map_err(
                |_| ForgeServerRouteAssemblyError::OperationNameNotAdmitted {
                    family: route.operation_family(),
                    operation_name: route.operation_name().to_string(),
                },
            )?;
        declared_routes.push(route);
    }
    Ok(())
}

fn append_session_coordination_routes(
    declared_routes: &mut Vec<ForgeServerDeclaredRoute>,
    route_keys: &mut BTreeMap<(String, String), ()>,
    compat_http_root: &CompatHttpSurfaceRoot,
    operation_registry: &ForgeServerOperationRegistry,
) -> Result<(), ForgeServerRouteAssemblyError> {
    let operation_inventory: ForgeServerOperationInventory = operation_registry.inventory();
    let Some(coordination_row) = operation_inventory.rows().iter().find(|row| {
        row.family() == ForgeServerOperationFamily::ProductSessionCoordination && row.enabled()
    }) else {
        return Ok(());
    };
    if !coordination_row
        .exposed_surfaces()
        .contains(&crate::ForgeServerSurfaceFamily::CompatHttp)
    {
        return Ok(());
    }
    require_route_family(
        compat_http_root,
        ForgeServerCompatHttpRouteFamily::Mutation,
        "product_session.open_mutation",
    )?;
    for operation_name in [
        "product_session.open_preview",
        "product_session.open_mutation",
        "product_session.close",
    ] {
        let route = ForgeServerDeclaredRoute::new(
            "POST",
            format!("/compat/mutations/{operation_name}"),
            ForgeServerCompatHttpRouteFamily::Mutation,
            ForgeServerOperationFamily::ProductSessionCoordination,
            operation_name,
            "forge-server-product-session.v1",
            "product-session",
        );
        require_unique_route(route_keys, route.method(), route.path())?;
        operation_registry
            .admit_operation_name(route.operation_family(), route.operation_name())
            .map_err(
                |_| ForgeServerRouteAssemblyError::OperationNameNotAdmitted {
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
    route_family: ForgeServerCompatHttpRouteFamily,
    operation_name: &str,
) -> Result<(), ForgeServerRouteAssemblyError> {
    if compat_http_root.route_families().contains(route_family) {
        return Ok(());
    }
    match route_family {
        ForgeServerCompatHttpRouteFamily::Read => Err(
            ForgeServerRouteAssemblyError::MissingCompatReadRouteFamily {
                operation_name: operation_name.to_string(),
            },
        ),
        ForgeServerCompatHttpRouteFamily::Mutation => Err(
            ForgeServerRouteAssemblyError::MissingCompatMutationRouteFamily {
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
) -> Result<(), ForgeServerRouteAssemblyError> {
    let key = (method.to_string(), path.to_string());
    if route_keys.insert(key.clone(), ()).is_some() {
        return Err(ForgeServerRouteAssemblyError::DuplicateMethodPath {
            method: key.0,
            path: key.1,
        });
    }
    Ok(())
}

fn operational_routes() -> Vec<ForgeServerOperationalRoute> {
    vec![
        ForgeServerOperationalRoute::new(
            ForgeServerOperationalRouteKind::Health,
            "GET",
            "/healthz",
        ),
        ForgeServerOperationalRoute::new(
            ForgeServerOperationalRouteKind::Metrics,
            "GET",
            "/metrics",
        ),
        ForgeServerOperationalRoute::new(
            ForgeServerOperationalRouteKind::Preflight,
            "OPTIONS",
            "/compat/preflight",
        ),
        ForgeServerOperationalRoute::new(
            ForgeServerOperationalRouteKind::DocsExport,
            "GET",
            "/openapi.json",
        ),
    ]
}

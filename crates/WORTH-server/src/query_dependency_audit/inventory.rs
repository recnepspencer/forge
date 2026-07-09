use worth_query::facade::WorthQueryRuntimeFacadeFamily;

use crate::{
    WorthServerDirectDeliveryClass, WorthServerDirectFreshnessMode,
    WorthServerQueryHandoffOperation, WorthServerQueryRequestedResume, WorthServerSurfaceFamily,
    WorthServerTransportClass,
};

use super::{WorthServerQueryDependencyAuditPathKind, WorthServerQueryDependencyRuntimeReadiness};

#[derive(Clone, Copy, Debug)]
pub(crate) enum WorthServerQueryDependencyBindingKind {
    QueryHandoff {
        surface_family: WorthServerSurfaceFamily,
        transport_class: WorthServerTransportClass,
        operation: fn() -> WorthServerQueryHandoffOperation,
    },
    ConsumerKitBoundaryAudit,
    StaticTestOnly,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WorthServerQueryDependencyCoveredPath {
    pub row_id: &'static str,
    pub path_kind: WorthServerQueryDependencyAuditPathKind,
    pub runtime_readiness: WorthServerQueryDependencyRuntimeReadiness,
    pub ordinary_path: bool,
    pub binding_kind: WorthServerQueryDependencyBindingKind,
    pub required_query_families: &'static [WorthQueryRuntimeFacadeFamily],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerQueryDependencyCoveredPathInventory {
    row_ids: Vec<String>,
    ordinary_row_count: usize,
    inventory_digest: String,
}

const READ_FAMILIES: &[WorthQueryRuntimeFacadeFamily] = &[
    WorthQueryRuntimeFacadeFamily::Read,
    WorthQueryRuntimeFacadeFamily::SharedRead,
];
const LIVE_FAMILIES: &[WorthQueryRuntimeFacadeFamily] = &[
    WorthQueryRuntimeFacadeFamily::Live,
    WorthQueryRuntimeFacadeFamily::SharedRead,
];
const INSPECT_FAMILIES: &[WorthQueryRuntimeFacadeFamily] = &[
    WorthQueryRuntimeFacadeFamily::Inspect,
    WorthQueryRuntimeFacadeFamily::SharedRead,
];
const WRITE_FAMILIES: &[WorthQueryRuntimeFacadeFamily] = &[
    WorthQueryRuntimeFacadeFamily::Write,
    WorthQueryRuntimeFacadeFamily::Submission,
    WorthQueryRuntimeFacadeFamily::Inspect,
];

fn query_handoff_read() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::query_read("users.profile")
}

fn query_handoff_mutation() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::query_mutation("users.rename")
}

fn direct_read() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::direct_read("users.profile")
}

fn direct_state() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::direct_state("users.profile")
}

fn direct_inspection() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::direct_inspection("users.profile")
}

fn direct_projection() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::direct_projection("users.profile")
}

fn direct_mutation() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::direct_mutation("users.rename")
}

fn downstream_delivery() -> WorthServerQueryHandoffOperation {
    WorthServerQueryHandoffOperation::downstream_delivery(
        "users.profile",
        WorthServerDirectFreshnessMode::LiveStrict,
        WorthServerDirectDeliveryClass::AuthoritativeOrdered,
        WorthServerQueryRequestedResume::runtime_backed(None::<String>),
    )
}

pub(crate) fn covered_paths() -> Vec<WorthServerQueryDependencyCoveredPath> {
    vec![
        WorthServerQueryDependencyCoveredPath {
            row_id: "WORTH-native.direct.read",
            path_kind: WorthServerQueryDependencyAuditPathKind::WorthNativeDirectRead,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_read,
            },
            required_query_families: READ_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "WORTH-native.direct.state",
            path_kind: WorthServerQueryDependencyAuditPathKind::WorthNativeDirectState,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_state,
            },
            required_query_families: LIVE_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "WORTH-native.direct.inspection",
            path_kind: WorthServerQueryDependencyAuditPathKind::WorthNativeDirectInspection,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_inspection,
            },
            required_query_families: INSPECT_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "WORTH-native.direct.projection",
            path_kind: WorthServerQueryDependencyAuditPathKind::WorthNativeDirectProjection,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_projection,
            },
            required_query_families: READ_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "WORTH-native.direct.mutation",
            path_kind: WorthServerQueryDependencyAuditPathKind::WorthNativeDirectMutation,
            runtime_readiness: WorthServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "direct-declaration.support-posture",
            path_kind: WorthServerQueryDependencyAuditPathKind::DirectDeclarationSupportPosture,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineEightConsumerKitClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: direct_read,
            },
            required_query_families: READ_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "compat-http.read-execution",
            path_kind: WorthServerQueryDependencyAuditPathKind::CompatibilityHttpRead,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::CompatHttp,
                transport_class: WorthServerTransportClass::CompatHttp,
                operation: query_handoff_read,
            },
            required_query_families: READ_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "compat-http.mutation-execution",
            path_kind: WorthServerQueryDependencyAuditPathKind::CompatibilityHttpMutation,
            runtime_readiness: WorthServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::CompatHttp,
                transport_class: WorthServerTransportClass::CompatHttp,
                operation: query_handoff_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "query-handoff.read",
            path_kind: WorthServerQueryDependencyAuditPathKind::QueryHandoffRead,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: query_handoff_read,
            },
            required_query_families: READ_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "query-handoff.mutation",
            path_kind: WorthServerQueryDependencyAuditPathKind::QueryHandoffMutation,
            runtime_readiness: WorthServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: query_handoff_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "query-handoff.downstream-delivery",
            path_kind: WorthServerQueryDependencyAuditPathKind::QueryHandoffDownstreamDelivery,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: WorthServerSurfaceFamily::WorthNative,
                transport_class: WorthServerTransportClass::WorthNativeInProcess,
                operation: downstream_delivery,
            },
            required_query_families: LIVE_FAMILIES,
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "server.consumer-boundary-audit",
            path_kind: WorthServerQueryDependencyAuditPathKind::ServerConsumerBoundaryAudit,
            runtime_readiness:
                WorthServerQueryDependencyRuntimeReadiness::QueryNineEightConsumerKitClosureReady,
            ordinary_path: true,
            binding_kind: WorthServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit,
            required_query_families: &[],
        },
        WorthServerQueryDependencyCoveredPath {
            row_id: "tests.support.query-handoff-runtime",
            path_kind: WorthServerQueryDependencyAuditPathKind::CertificationTestBackendSupport,
            runtime_readiness: WorthServerQueryDependencyRuntimeReadiness::StaticTestOnly,
            ordinary_path: false,
            binding_kind: WorthServerQueryDependencyBindingKind::StaticTestOnly,
            required_query_families: &[],
        },
    ]
}

pub(crate) fn covered_path_inventory() -> WorthServerQueryDependencyCoveredPathInventory {
    let paths = covered_paths();
    let mut row_ids = paths
        .iter()
        .map(|path| path.row_id.to_string())
        .collect::<Vec<_>>();
    row_ids.sort();
    let ordinary_row_count = paths.iter().filter(|path| path.ordinary_path).count();
    let inventory_digest = row_ids.join("|");
    WorthServerQueryDependencyCoveredPathInventory {
        row_ids,
        ordinary_row_count,
        inventory_digest,
    }
}

impl WorthServerQueryDependencyCoveredPath {
    pub(crate) fn surface_family(&self) -> WorthServerSurfaceFamily {
        match self.binding_kind {
            WorthServerQueryDependencyBindingKind::QueryHandoff { surface_family, .. } => {
                surface_family
            }
            WorthServerQueryDependencyBindingKind::StaticTestOnly => {
                WorthServerSurfaceFamily::WorthNative
            }
            WorthServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                WorthServerSurfaceFamily::WorthNative
            }
        }
    }

    pub(crate) fn transport_class(&self) -> WorthServerTransportClass {
        match self.binding_kind {
            WorthServerQueryDependencyBindingKind::QueryHandoff {
                transport_class, ..
            } => transport_class,
            WorthServerQueryDependencyBindingKind::StaticTestOnly => {
                WorthServerTransportClass::WorthNativeInProcess
            }
            WorthServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                WorthServerTransportClass::WorthNativeInProcess
            }
        }
    }

    pub(crate) fn operation(&self) -> WorthServerQueryHandoffOperation {
        match self.binding_kind {
            WorthServerQueryDependencyBindingKind::QueryHandoff { operation, .. } => operation(),
            WorthServerQueryDependencyBindingKind::StaticTestOnly => {
                WorthServerQueryHandoffOperation::query_read("unused")
            }
            WorthServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                WorthServerQueryHandoffOperation::query_read("unused")
            }
        }
    }
}

impl WorthServerQueryDependencyCoveredPathInventory {
    pub fn row_ids(&self) -> &[String] {
        &self.row_ids
    }

    pub fn ordinary_row_count(&self) -> usize {
        self.ordinary_row_count
    }

    pub fn static_test_only_row_count(&self) -> usize {
        self.row_ids.len().saturating_sub(self.ordinary_row_count)
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}

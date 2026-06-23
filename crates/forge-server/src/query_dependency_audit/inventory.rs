use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::{
    ForgeServerDirectDeliveryClass, ForgeServerDirectFreshnessMode,
    ForgeServerQueryHandoffOperation, ForgeServerQueryRequestedResume, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

use super::{ForgeServerQueryDependencyAuditPathKind, ForgeServerQueryDependencyRuntimeReadiness};

#[derive(Clone, Copy, Debug)]
pub(crate) enum ForgeServerQueryDependencyBindingKind {
    QueryHandoff {
        surface_family: ForgeServerSurfaceFamily,
        transport_class: ForgeServerTransportClass,
        operation: fn() -> ForgeServerQueryHandoffOperation,
    },
    ConsumerKitBoundaryAudit,
    StaticTestOnly,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ForgeServerQueryDependencyCoveredPath {
    pub row_id: &'static str,
    pub path_kind: ForgeServerQueryDependencyAuditPathKind,
    pub runtime_readiness: ForgeServerQueryDependencyRuntimeReadiness,
    pub ordinary_path: bool,
    pub binding_kind: ForgeServerQueryDependencyBindingKind,
    pub required_query_families: &'static [ForgeQueryRuntimeFacadeFamily],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryDependencyCoveredPathInventory {
    row_ids: Vec<String>,
    ordinary_row_count: usize,
    inventory_digest: String,
}

const READ_FAMILIES: &[ForgeQueryRuntimeFacadeFamily] = &[
    ForgeQueryRuntimeFacadeFamily::Read,
    ForgeQueryRuntimeFacadeFamily::SharedRead,
];
const LIVE_FAMILIES: &[ForgeQueryRuntimeFacadeFamily] = &[
    ForgeQueryRuntimeFacadeFamily::Live,
    ForgeQueryRuntimeFacadeFamily::SharedRead,
];
const INSPECT_FAMILIES: &[ForgeQueryRuntimeFacadeFamily] = &[
    ForgeQueryRuntimeFacadeFamily::Inspect,
    ForgeQueryRuntimeFacadeFamily::SharedRead,
];
const WRITE_FAMILIES: &[ForgeQueryRuntimeFacadeFamily] = &[
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Submission,
    ForgeQueryRuntimeFacadeFamily::Inspect,
];

fn query_handoff_read() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::query_read("users.profile")
}

fn query_handoff_mutation() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::query_mutation("users.rename")
}

fn direct_read() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::direct_read("users.profile")
}

fn direct_state() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::direct_state("users.profile")
}

fn direct_inspection() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::direct_inspection("users.profile")
}

fn direct_projection() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::direct_projection("users.profile")
}

fn direct_mutation() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::direct_mutation("users.rename")
}

fn downstream_delivery() -> ForgeServerQueryHandoffOperation {
    ForgeServerQueryHandoffOperation::downstream_delivery(
        "users.profile",
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        ForgeServerQueryRequestedResume::runtime_backed(None::<String>),
    )
}

pub(crate) fn covered_paths() -> Vec<ForgeServerQueryDependencyCoveredPath> {
    vec![
        ForgeServerQueryDependencyCoveredPath {
            row_id: "forge-native.direct.read",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ForgeNativeDirectRead,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_read,
            },
            required_query_families: READ_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "forge-native.direct.state",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ForgeNativeDirectState,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_state,
            },
            required_query_families: LIVE_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "forge-native.direct.inspection",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ForgeNativeDirectInspection,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_inspection,
            },
            required_query_families: INSPECT_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "forge-native.direct.projection",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ForgeNativeDirectProjection,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_projection,
            },
            required_query_families: READ_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "forge-native.direct.mutation",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ForgeNativeDirectMutation,
            runtime_readiness: ForgeServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "direct-declaration.support-posture",
            path_kind: ForgeServerQueryDependencyAuditPathKind::DirectDeclarationSupportPosture,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineEightConsumerKitClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: direct_read,
            },
            required_query_families: READ_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "compat-http.read-execution",
            path_kind: ForgeServerQueryDependencyAuditPathKind::CompatibilityHttpRead,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::CompatHttp,
                transport_class: ForgeServerTransportClass::CompatHttp,
                operation: query_handoff_read,
            },
            required_query_families: READ_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "compat-http.mutation-execution",
            path_kind: ForgeServerQueryDependencyAuditPathKind::CompatibilityHttpMutation,
            runtime_readiness: ForgeServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::CompatHttp,
                transport_class: ForgeServerTransportClass::CompatHttp,
                operation: query_handoff_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "query-handoff.read",
            path_kind: ForgeServerQueryDependencyAuditPathKind::QueryHandoffRead,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: query_handoff_read,
            },
            required_query_families: READ_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "query-handoff.mutation",
            path_kind: ForgeServerQueryDependencyAuditPathKind::QueryHandoffMutation,
            runtime_readiness: ForgeServerQueryDependencyRuntimeReadiness::
                QueryNineSevenDeterministicSubmissionClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: query_handoff_mutation,
            },
            required_query_families: WRITE_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "query-handoff.downstream-delivery",
            path_kind: ForgeServerQueryDependencyAuditPathKind::QueryHandoffDownstreamDelivery,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineSevenSharedReadClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::QueryHandoff {
                surface_family: ForgeServerSurfaceFamily::ForgeNative,
                transport_class: ForgeServerTransportClass::ForgeNativeInProcess,
                operation: downstream_delivery,
            },
            required_query_families: LIVE_FAMILIES,
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "server.consumer-boundary-audit",
            path_kind: ForgeServerQueryDependencyAuditPathKind::ServerConsumerBoundaryAudit,
            runtime_readiness:
                ForgeServerQueryDependencyRuntimeReadiness::QueryNineEightConsumerKitClosureReady,
            ordinary_path: true,
            binding_kind: ForgeServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit,
            required_query_families: &[],
        },
        ForgeServerQueryDependencyCoveredPath {
            row_id: "tests.support.query-handoff-runtime",
            path_kind: ForgeServerQueryDependencyAuditPathKind::CertificationTestBackendSupport,
            runtime_readiness: ForgeServerQueryDependencyRuntimeReadiness::StaticTestOnly,
            ordinary_path: false,
            binding_kind: ForgeServerQueryDependencyBindingKind::StaticTestOnly,
            required_query_families: &[],
        },
    ]
}

pub(crate) fn covered_path_inventory() -> ForgeServerQueryDependencyCoveredPathInventory {
    let paths = covered_paths();
    let mut row_ids = paths
        .iter()
        .map(|path| path.row_id.to_string())
        .collect::<Vec<_>>();
    row_ids.sort();
    let ordinary_row_count = paths.iter().filter(|path| path.ordinary_path).count();
    let inventory_digest = row_ids.join("|");
    ForgeServerQueryDependencyCoveredPathInventory {
        row_ids,
        ordinary_row_count,
        inventory_digest,
    }
}

impl ForgeServerQueryDependencyCoveredPath {
    pub(crate) fn surface_family(&self) -> ForgeServerSurfaceFamily {
        match self.binding_kind {
            ForgeServerQueryDependencyBindingKind::QueryHandoff { surface_family, .. } => {
                surface_family
            }
            ForgeServerQueryDependencyBindingKind::StaticTestOnly => {
                ForgeServerSurfaceFamily::ForgeNative
            }
            ForgeServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                ForgeServerSurfaceFamily::ForgeNative
            }
        }
    }

    pub(crate) fn transport_class(&self) -> ForgeServerTransportClass {
        match self.binding_kind {
            ForgeServerQueryDependencyBindingKind::QueryHandoff {
                transport_class, ..
            } => transport_class,
            ForgeServerQueryDependencyBindingKind::StaticTestOnly => {
                ForgeServerTransportClass::ForgeNativeInProcess
            }
            ForgeServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                ForgeServerTransportClass::ForgeNativeInProcess
            }
        }
    }

    pub(crate) fn operation(&self) -> ForgeServerQueryHandoffOperation {
        match self.binding_kind {
            ForgeServerQueryDependencyBindingKind::QueryHandoff { operation, .. } => operation(),
            ForgeServerQueryDependencyBindingKind::StaticTestOnly => {
                ForgeServerQueryHandoffOperation::query_read("unused")
            }
            ForgeServerQueryDependencyBindingKind::ConsumerKitBoundaryAudit => {
                ForgeServerQueryHandoffOperation::query_read("unused")
            }
        }
    }
}

impl ForgeServerQueryDependencyCoveredPathInventory {
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

use crate::identity::hash_parts;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimePublicSurfaceKind {
    PublicFacade,
    InternalRouteComposer,
    DownstreamRuntimeBoundary,
}

impl ForgeQueryLowerRuntimePublicSurfaceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PublicFacade => "public-facade",
            Self::InternalRouteComposer => "internal-route-composer",
            Self::DownstreamRuntimeBoundary => "downstream-runtime-boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimePublicSurfaceRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    surface_label: &'static str,
    implementation_path: &'static str,
    surface_kind: ForgeQueryLowerRuntimePublicSurfaceKind,
    delegated_lane: &'static str,
}

impl ForgeQueryLowerRuntimePublicSurfaceRow {
    pub(crate) const fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        surface_label: &'static str,
        implementation_path: &'static str,
        surface_kind: ForgeQueryLowerRuntimePublicSurfaceKind,
        delegated_lane: &'static str,
    ) -> Self {
        Self {
            seam_key,
            surface_label,
            implementation_path,
            surface_kind,
            delegated_lane,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn surface_label(&self) -> &'static str {
        self.surface_label
    }

    pub fn implementation_path(&self) -> &'static str {
        self.implementation_path
    }

    pub fn surface_kind(&self) -> ForgeQueryLowerRuntimePublicSurfaceKind {
        self.surface_kind
    }

    pub fn delegated_lane(&self) -> &'static str {
        self.delegated_lane
    }

    pub fn row_digest(&self) -> String {
        hash_parts(&[
            self.seam_key.as_str().to_string(),
            self.surface_label.to_string(),
            self.implementation_path.to_string(),
            self.surface_kind.as_str().to_string(),
            self.delegated_lane.to_string(),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimePublicSurfaceInventory {
    rows: &'static [ForgeQueryLowerRuntimePublicSurfaceRow],
}

impl ForgeQueryLowerRuntimePublicSurfaceInventory {
    pub(crate) const fn new(rows: &'static [ForgeQueryLowerRuntimePublicSurfaceRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryLowerRuntimePublicSurfaceRow] {
        self.rows
    }

    pub fn public_surface_digest(&self) -> String {
        hash_parts(
            &self
                .rows
                .iter()
                .map(ForgeQueryLowerRuntimePublicSurfaceRow::row_digest)
                .collect::<Vec<_>>(),
        )
    }
}

const PUBLIC_SURFACE_ROWS: &[ForgeQueryLowerRuntimePublicSurfaceRow] = &[
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        "ForgeQueryWorkspace::compose_read(...)",
        "crates/forge-query/src/runtime/workspace_queries.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "read-family intent execution",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::ComposeReadWithInvariantPack,
        "ForgeQueryWorkspace::compose_read_with_invariant_pack(...)",
        "crates/forge-query/src/runtime/workspace_queries.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "read-family intent execution after invariant admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamily,
        "ForgeQueryWorkspace::execute_read_family(...)",
        "crates/forge-query/src/runtime/workspace_queries.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "read-family intent execution",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        "ForgeQueryWorkspace::execute_read_family_in_basis_context(...)",
        "crates/forge-query/src/runtime/workspace_queries.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "basis-context read-family intent execution",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        "ForgeQueryWorkspace::live_view(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "workspace live declaration routed through runtime declaration and installation",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        "ForgeQueryWorkspace::live_view_request(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "workspace live declaration routed through runtime declaration and installation",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        "ForgeQueryRuntime::declare_live_view(...)",
        "crates/forge-query/src/runtime/runtime_declarations.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "live declaration receipt plus installation route",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::write(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::insert(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::update(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::update_existing(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after existing-truth binding admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::assert_existing(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after existing-truth binding admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::verify_existing(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after existing-truth binding admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::update_existing_verified(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after backend-verified existing-truth admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::delete(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::delete_with(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::delete_existing(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after existing-truth binding admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::delete_existing_with(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after existing-truth binding admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::delete_existing_verified(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution after backend-verified existing-truth admission",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryRuntime::write(...)",
        "crates/forge-query/src/runtime/runtime_writes.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryRuntime::write_batch(...)",
        "crates/forge-query/src/runtime/runtime_batch_write_entrypoints.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative batch mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        "ForgeQueryWorkspace::batch(...)",
        "crates/forge-query/src/runtime/workspace.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::PublicFacade,
        "authoritative batch mutation intent execution through write-authority routing",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        "ForgeQueryRuntime::execute_authoritative_write_command_direct(...)",
        "crates/forge-query/src/runtime/runtime_writes.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::InternalRouteComposer,
        "mutation receipt routing through signal invalidation boundary receipts",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        "ForgeQueryRuntime::execute_authoritative_write_batch_direct(...)",
        "crates/forge-query/src/runtime/runtime_batch_writes.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::InternalRouteComposer,
        "batch mutation receipt routing through signal invalidation boundary receipts",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        "ForgeQueryRuntime::install_live_subscription_for_request(...)",
        "crates/forge-query/src/runtime/runtime_sessions.rs",
        ForgeQueryLowerRuntimePublicSurfaceKind::InternalRouteComposer,
        "subscription declaration, lowering, admission, and activation route",
    ),
    ForgeQueryLowerRuntimePublicSurfaceRow::new(
        ForgeQueryLowerRuntimeSeamKey::DownstreamQueryRuntimeBoundarySubtree,
        "worth-topo projection runtime boundary",
        "crates/worth-topo/src/projection/runtime_boundary",
        ForgeQueryLowerRuntimePublicSurfaceKind::DownstreamRuntimeBoundary,
        "declared downstream runtime-boundary subtree",
    ),
];

pub fn forge_query_lower_runtime_public_surface_inventory(
) -> ForgeQueryLowerRuntimePublicSurfaceInventory {
    ForgeQueryLowerRuntimePublicSurfaceInventory::new(PUBLIC_SURFACE_ROWS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_crossing_inventory, forge_query_lower_runtime_support_matrix,
        ForgeQueryLowerRuntimeSupportPosture,
    };

    #[test]
    fn public_surface_rows_reference_known_routed_or_audited_seams() {
        let crossing_inventory = forge_query_lower_runtime_crossing_inventory();
        let support = forge_query_lower_runtime_support_matrix();

        for row in forge_query_lower_runtime_public_surface_inventory().rows() {
            if row.seam_key()
                == ForgeQueryLowerRuntimeSeamKey::DownstreamQueryRuntimeBoundarySubtree
            {
                continue;
            }
            assert!(crossing_inventory
                .rows()
                .iter()
                .any(|crossing| crossing.seam_key() == row.seam_key()));
            assert_eq!(
                support
                    .support_for(row.seam_key())
                    .expect("public surface rows must resolve through support")
                    .posture(),
                ForgeQueryLowerRuntimeSupportPosture::Admitted
            );
        }
    }

    #[test]
    fn public_surface_inventory_digest_is_row_order_stable() {
        let inventory = forge_query_lower_runtime_public_surface_inventory();
        let expected = hash_parts(
            &inventory
                .rows()
                .iter()
                .map(ForgeQueryLowerRuntimePublicSurfaceRow::row_digest)
                .collect::<Vec<_>>(),
        );
        assert_eq!(inventory.public_surface_digest(), expected);
    }

    #[test]
    fn public_surface_inventory_covers_mutation_facades_and_internal_routing_composers() {
        let inventory = forge_query_lower_runtime_public_surface_inventory();

        for label in [
            "ForgeQueryWorkspace::live_view(...)",
            "ForgeQueryWorkspace::live_view_request(...)",
            "ForgeQueryWorkspace::write(...)",
            "ForgeQueryWorkspace::assert_existing(...)",
            "ForgeQueryWorkspace::verify_existing(...)",
            "ForgeQueryWorkspace::update_existing_verified(...)",
            "ForgeQueryWorkspace::batch(...)",
            "ForgeQueryWorkspace::delete_existing_verified(...)",
            "ForgeQueryRuntime::write(...)",
            "ForgeQueryRuntime::write_batch(...)",
            "ForgeQueryRuntime::execute_authoritative_write_command_direct(...)",
            "ForgeQueryRuntime::execute_authoritative_write_batch_direct(...)",
        ] {
            assert!(inventory
                .rows()
                .iter()
                .any(|row| row.surface_label() == label));
        }
    }
}

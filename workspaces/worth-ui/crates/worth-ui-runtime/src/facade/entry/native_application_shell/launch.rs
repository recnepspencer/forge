use super::{
    NativeMountedRow, WorthUiNativeApplicationCleanup, WorthUiNativeApplicationShell,
    WorthUiNativeApplicationShellLaunchDenial,
};
use crate::facade::entry::WorthUiApp;
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiSurfaceBindingCoordinatePosture, UiSurfaceBindingGeneration,
    UiSurfaceBindingProfile,
};
use std::collections::HashMap;

struct ConfiguredNativeSurface {
    binding: UiSurfaceBindingGeneration,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    mounted_rows: Vec<NativeMountedRow>,
    mounted_row_indices: HashMap<Box<str>, usize>,
}

struct NativeSurfaceConfigurationFailure {
    cause: WorthUiNativeApplicationShellLaunchDenial,
    expected_released_surface_count: usize,
}

impl WorthUiApp {
    /// Launch one native surface without exposing mounted construction authority.
    pub fn launch_native_surface(
        self,
    ) -> Result<WorthUiNativeApplicationShell, WorthUiNativeApplicationShellLaunchDenial> {
        self.launch_native_surface_at_scale(1_000)
    }

    #[doc(hidden)]
    pub fn launch_native_surface_at_scale(
        self,
        scale_factor_milli: u32,
    ) -> Result<WorthUiNativeApplicationShell, WorthUiNativeApplicationShellLaunchDenial> {
        let mut session = self.launch().map_err(|denial| match denial {
            crate::runtime::WorthUiRuntimeLaunchDenial::HostSessionReleaseIndeterminate {
                ..
            }
            | crate::runtime::WorthUiRuntimeLaunchDenial::HostSessionReleaseMismatch { .. } => {
                WorthUiNativeApplicationShellLaunchDenial::RuntimeLaunchCleanup(denial)
            }
            _ => WorthUiNativeApplicationShellLaunchDenial::RuntimeLaunch,
        })?;
        let configured = match configure_native_surface(&mut session, scale_factor_milli) {
            Ok(configured) => configured,
            Err(failure) => {
                let mut cleanup = session.shutdown();
                return Err(
                    if launch_cleanup_complete(&cleanup, failure.expected_released_surface_count) {
                        failure.cause
                    } else {
                        WorthUiNativeApplicationShellLaunchDenial::ApplicationCleanup(
                            WorthUiNativeApplicationCleanup {
                                host_cleanup: cleanup.take_host_session_recovery(),
                            },
                        )
                    },
                );
            }
        };
        Ok(WorthUiNativeApplicationShell {
            session,
            binding: configured.binding,
            surface: configured.surface,
            mounted_rows: configured.mounted_rows,
            mounted_row_indices: configured.mounted_row_indices,
        })
    }
}

fn configure_native_surface(
    session: &mut crate::facade::entry::WorthUiActiveApplicationSession,
    scale_factor_milli: u32,
) -> Result<ConfiguredNativeSurface, NativeSurfaceConfigurationFailure> {
    let surface = session.create_semantic_surface().map_err(|_| {
        configuration_failure(
            WorthUiNativeApplicationShellLaunchDenial::SemanticSurfaceCreation,
            0,
        )
    })?;
    let profile = UiSurfaceBindingProfile::new(
        scale_factor_milli,
        UiSurfaceBindingCoordinatePosture::LogicalPoints,
        1,
    )
    .map_err(|_| {
        configuration_failure(
            WorthUiNativeApplicationShellLaunchDenial::HostSurfaceRegistration,
            0,
        )
    })?;
    let binding = session
        .register_host_surface(
            surface,
            UiHostSurfacePresentationMode::NativeDisplay,
            profile,
        )
        .map_err(|_| {
            configuration_failure(
                WorthUiNativeApplicationShellLaunchDenial::HostSurfaceRegistration,
                0,
            )
        })?;
    let graph_nodes = {
        let graph = session.graph();
        graph
            .node_identities()
            .filter_map(|identity| {
                let lookup = graph.lookup().graph_node(identity)?;
                let semantic = lookup
                    .value()
                    .declaration_identity()
                    .authored_semantic_name()
                    .to_owned();
                semantic
                    .starts_with("component:")
                    .then(|| (identity, Box::<str>::from(semantic)))
            })
            .collect::<Vec<_>>()
    };
    let mut mounted_rows = Vec::with_capacity(graph_nodes.len());
    let mut mounted_row_indices = HashMap::with_capacity(graph_nodes.len());
    for (graph_node, authored_semantic_identity) in graph_nodes {
        let handle = session.mounted_graph_node(graph_node).map_err(|_| {
            configuration_failure(
                WorthUiNativeApplicationShellLaunchDenial::MountedInstanceCreation,
                1,
            )
        })?;
        let mounted = session.mount_instance(handle, surface).map_err(|_| {
            configuration_failure(
                WorthUiNativeApplicationShellLaunchDenial::MountedInstanceCreation,
                1,
            )
        })?;
        let index = mounted_rows.len();
        if mounted_row_indices
            .insert(authored_semantic_identity.clone(), index)
            .is_some()
        {
            return Err(configuration_failure(
                WorthUiNativeApplicationShellLaunchDenial::MountedInstanceCreation,
                1,
            ));
        }
        mounted_rows.push(NativeMountedRow {
            graph_node,
            mounted: Some(mounted),
        });
    }
    session
        .establish_native_viewport_allocation()
        .map_err(|_| {
            configuration_failure(
                WorthUiNativeApplicationShellLaunchDenial::ViewportAllocation,
                1,
            )
        })?;
    Ok(ConfiguredNativeSurface {
        binding: binding.binding_generation(),
        surface,
        mounted_rows,
        mounted_row_indices,
    })
}

fn configuration_failure(
    cause: WorthUiNativeApplicationShellLaunchDenial,
    expected_released_surface_count: usize,
) -> NativeSurfaceConfigurationFailure {
    NativeSurfaceConfigurationFailure {
        cause,
        expected_released_surface_count,
    }
}

fn launch_cleanup_complete(
    receipt: &crate::runtime::WorthUiRuntimeShutdownReceipt,
    expected_released_surface_count: usize,
) -> bool {
    matches!(
        receipt.host_session_release(),
        Some(worth_ui_host_contract::UiHostSessionReleaseOutcome::Released(released))
            if released.released_surface_count() == expected_released_surface_count
    )
}

use forge_server::{
    facade::{ForgeServer, ForgeServerBuildError},
    surfaces::ForgeNativeSurface,
    ForgeServerConfig, ForgeServerSurfaceFamily, ForgeServerSurfaceRegistryError,
};

#[test]
fn duplicate_surface_family_registration_fails_typed() {
    let build_error = ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .build()
                .expect("bind address should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(ForgeNativeSurface::disabled())
        .build()
        .expect_err("duplicate family should be rejected");

    assert_eq!(
        build_error,
        ForgeServerBuildError::InvalidSurfaceRegistry(
            ForgeServerSurfaceRegistryError::DuplicateSurfaceFamily {
                family: ForgeServerSurfaceFamily::ForgeNative,
            },
        )
    );
}

#[test]
fn duplicate_surface_rejection_fails_at_build_boundary() {
    let result = ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .build()
                .expect("bind address should validate"),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .register_surface(ForgeNativeSurface::disabled())
        .build();

    assert!(matches!(
        result,
        Err(ForgeServerBuildError::InvalidSurfaceRegistry(_))
    ));
}

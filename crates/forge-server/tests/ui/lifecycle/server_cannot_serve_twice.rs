use forge_server::{surfaces::ForgeNativeSurface, ForgeServer, ForgeServerConfig};

fn main() {
    let server = ForgeServer::builder()
        .with_config(
            ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .build()
                .unwrap(),
        )
        .register_surface(ForgeNativeSurface::disabled())
        .build()
        .unwrap();

    let _first_serve = server.serve();
    let _second_serve = server.serve();
}

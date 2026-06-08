use std::io;

use axum::Router;
use tokio::net::TcpListener;

use crate::runtime::ForgeServerRuntime;

pub(crate) async fn serve_runtime(runtime: ForgeServerRuntime) -> io::Result<()> {
    let listener =
        TcpListener::bind(runtime.assembly().config().bind_address().socket_addr()).await?;
    runtime.assembly().counters().increment_serve_start_count();
    let _surface_inventory = runtime.assembly().surface_registry().inventory();
    axum::serve(listener, Router::new()).await
}

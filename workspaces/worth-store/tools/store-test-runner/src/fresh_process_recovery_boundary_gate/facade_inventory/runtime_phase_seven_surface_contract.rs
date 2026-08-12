mod backend;
mod runtime;
mod store;
mod wal;

pub(super) fn phase_seven_delivery_surfaces(
) -> impl Iterator<Item = &'static (&'static str, &'static str, &'static str)> {
    backend::BACKEND_SURFACES
        .iter()
        .chain(runtime::RUNTIME_SURFACES)
        .chain(store::STORE_SURFACES)
        .chain(wal::WAL_SURFACES)
}

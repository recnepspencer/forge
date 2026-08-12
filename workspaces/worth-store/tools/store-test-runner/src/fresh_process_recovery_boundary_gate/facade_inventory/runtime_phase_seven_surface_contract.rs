mod backend;
mod format;
mod media_effect;
mod runtime;
mod store;
mod wal;

pub(super) fn phase_seven_delivery_surfaces(
) -> impl Iterator<Item = &'static (&'static str, &'static str, &'static str)> {
    backend::BACKEND_SURFACES
        .iter()
        .chain(format::FORMAT_SURFACES)
        .chain(media_effect::MEDIA_EFFECT_SURFACES)
        .chain(runtime::RUNTIME_SURFACES)
        .chain(store::STORE_SURFACES)
        .chain(wal::WAL_SURFACES)
}

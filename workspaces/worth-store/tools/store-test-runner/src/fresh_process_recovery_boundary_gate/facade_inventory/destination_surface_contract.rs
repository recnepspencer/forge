mod backend_recovery;
mod backend_recovery_publication;
mod store_recovery_coordination;
mod store_recovery_publication;
#[cfg(test)]
mod tests;

pub(super) const BACKEND_RECOVERY_SURFACES: &[(&str, &str, &str)] =
    backend_recovery::BACKEND_RECOVERY_SURFACES;
pub(super) const BACKEND_RECOVERY_PUBLICATION_SURFACES: &[(&str, &str, &str)] =
    backend_recovery_publication::BACKEND_RECOVERY_PUBLICATION_SURFACES;
pub(super) const STORE_RECOVERY_COORDINATION_SURFACES: &[(&str, &str, &str)] =
    store_recovery_coordination::STORE_RECOVERY_COORDINATION_SURFACES;
pub(super) const STORE_RECOVERY_PUBLICATION_SURFACES: &[(&str, &str, &str)] =
    store_recovery_publication::STORE_RECOVERY_PUBLICATION_SURFACES;

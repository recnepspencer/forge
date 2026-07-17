mod acquisition;
mod untrusted_media_set;

pub use acquisition::{OfflineMediaAcquisitionDenial, OfflineMediaAcquisitionDimension};
pub use untrusted_media_set::UntrustedOfflineMediaSet;

pub(crate) use acquisition::acquire_read_only_media;

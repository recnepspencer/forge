use worth_store_physical_backend::QualifiedFilesystemMedia;

/// Sole owner of the qualified media route used by physical work.
///
/// Existing C.5 record serving temporarily borrows this same route until its
/// reads and publication path migrate into canonical physical work.
pub(in crate::physical_runtime) struct PhysicalWorkExecutor {
    media: QualifiedFilesystemMedia,
}

impl PhysicalWorkExecutor {
    pub(super) const fn new(media: QualifiedFilesystemMedia) -> Self {
        Self { media }
    }

    pub(in crate::physical_runtime) const fn record_serving_media(
        &self,
    ) -> &QualifiedFilesystemMedia {
        &self.media
    }

    pub(in crate::physical_runtime) fn into_media(self) -> QualifiedFilesystemMedia {
        self.media
    }
}

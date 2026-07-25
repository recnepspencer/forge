use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::RecordArtifactFile;

use super::{
    frame_loading::{
        CanonicalFrameReadSource, DirectFrameReadSource, FrameLoadFailure, FrameReadWorkAdmission,
        LoadedPhysicalFrame, ObservedArtifactLength,
    },
    frame_ports::{FrameLoadPort, RecordFramePorts},
};

/// Read-only access to record frames through either bootstrap media or the
/// serving runtime's canonical physical-work route.
pub(in crate::physical_runtime::record_serving) struct RecordFrameReader<'media> {
    route: RecordFrameReadRoute<'media>,
}

enum RecordFrameReadRoute<'media> {
    Bootstrap {
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn FrameLoadPort + Send + Sync),
    },
    Serving {
        frame_ports: RecordFramePorts,
        source: CanonicalFrameReadSource,
    },
}

impl<'media> RecordFrameReader<'media> {
    pub(in crate::physical_runtime::record_serving) const fn bootstrap(
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn FrameLoadPort + Send + Sync),
    ) -> Self {
        Self {
            route: RecordFrameReadRoute::Bootstrap { media, loader },
        }
    }

    pub(in crate::physical_runtime::record_serving) fn serving(
        frame_ports: RecordFramePorts,
        source: CanonicalFrameReadSource,
    ) -> RecordFrameReader<'static> {
        RecordFrameReader {
            route: RecordFrameReadRoute::Serving {
                frame_ports,
                source,
            },
        }
    }

    pub(in crate::physical_runtime::record_serving) fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameLoadFailure> {
        match &self.route {
            RecordFrameReadRoute::Bootstrap { media, loader } => {
                loader.file_length(&DirectFrameReadSource::new(media), artifact)
            }
            RecordFrameReadRoute::Serving {
                frame_ports,
                source,
            } => frame_ports.loader().file_length(source, artifact),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn load_exact(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        match &self.route {
            RecordFrameReadRoute::Bootstrap { media, loader } => loader.load_exact(
                &DirectFrameReadSource::new(media),
                artifact,
                offset,
                length,
                FrameReadWorkAdmission::ResidencyFaultOnly,
            ),
            RecordFrameReadRoute::Serving {
                frame_ports,
                source,
            } => frame_ports.loader().load_exact(
                source,
                artifact,
                offset,
                length,
                FrameReadWorkAdmission::EveryAccess,
            ),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn load_bounded(
        &self,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        match &self.route {
            RecordFrameReadRoute::Bootstrap { media, loader } => loader.load_bounded(
                &DirectFrameReadSource::new(media),
                artifact,
                limit,
                FrameReadWorkAdmission::ResidencyFaultOnly,
            ),
            RecordFrameReadRoute::Serving {
                frame_ports,
                source,
            } => frame_ports.loader().load_bounded(
                source,
                artifact,
                limit,
                FrameReadWorkAdmission::EveryAccess,
            ),
        }
    }
}

use super::{
    CertificationConfinementEffect, FilesystemMediaOwner, MediaOperationRole,
    NamespaceConfinementDenial, NamespaceFileOpenResult, NamespaceRelativePath,
    PositionedWriteRequest,
};

impl FilesystemMediaOwner {
    pub(crate) fn certification_confinement_probe(
        &self,
        component: &str,
    ) -> Result<(), NamespaceConfinementDenial> {
        let boundary = self.boundary().begin(MediaOperationRole::OpenExisting, 0);
        match super::namespace_confinement::certification_probe_component(component) {
            Ok(()) => {
                boundary.denied();
                Ok(())
            }
            Err(denial) => {
                boundary.confinement_denied();
                Err(denial)
            }
        }
    }

    pub(crate) fn certification_staging_effect_probe(
        &self,
        component: &str,
    ) -> CertificationConfinementEffect {
        let path =
            match NamespaceRelativePath::bind_certification_staging(self.identity(), component) {
                Ok(path) => path,
                Err(denial) => return CertificationConfinementEffect::ComponentDenied(denial),
            };
        let handle = match self.open_existing_for_mutation(&path).into_result() {
            NamespaceFileOpenResult::Failed(failure) => {
                return CertificationConfinementEffect::OpenDenied(failure.effect_status())
            }
            NamespaceFileOpenResult::Opened { handle, .. } => handle,
        };
        let write = handle.positioned_write(PositionedWriteRequest::new(
            0,
            b"worth-store-confinement-probe",
        ));
        CertificationConfinementEffect::WriteReached(write.effect_status())
    }
}

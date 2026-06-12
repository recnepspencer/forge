macro_rules! impl_hadwiger_artifact {
    ($type:ty, $core:ident) => {
        impl $crate::domain_artifacts::core_artifact::HadwigerCanonicalArtifact for $type {
            fn artifact_kind(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactKind {
                self.$core.artifact_kind()
            }

            fn artifact_digest(
                &self,
            ) -> &$crate::domain_artifacts::core_artifact::HadwigerArtifactDigest {
                self.$core.artifact_digest()
            }

            fn authority_owner(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactAuthorityOwner {
                self.$core.authority_owner()
            }

            fn source_reference(
                &self,
            ) -> &$crate::domain_artifacts::core_artifact::HadwigerArtifactSourceReference {
                self.$core.source_reference()
            }

            fn parent_artifacts(
                &self,
            ) -> &[$crate::domain_artifacts::core_artifact::HadwigerArtifactReference] {
                self.$core.parent_artifacts()
            }

            fn reference(
                &self,
            ) -> $crate::domain_artifacts::core_artifact::HadwigerArtifactReference {
                self.$core.reference()
            }
        }
    };
}

pub(crate) use impl_hadwiger_artifact;

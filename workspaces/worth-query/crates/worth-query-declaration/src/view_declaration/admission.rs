use crate::canonicalization::CanonicalQueryBundle;

use super::compatibility::ViewShapeCompatibilityMatrixArtifact;
use super::descriptor::ViewShapeDescriptor;
use super::digest::ViewShapeDigest;
use super::error::{ViewShapeError, ViewShapeFailureClass};
use super::family::ViewShapeFamily;
use super::identity::ViewShapeIdentityBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedViewShape {
    descriptor: ViewShapeDescriptor,
    digest: ViewShapeDigest,
    compatibility: ViewShapeCompatibilityMatrixArtifact,
    identity_binding: ViewShapeIdentityBinding,
}

impl AdmittedViewShape {
    pub fn family(&self) -> ViewShapeFamily {
        self.descriptor.family()
    }

    pub fn digest(&self) -> &ViewShapeDigest {
        &self.digest
    }

    pub fn compatibility(&self) -> &ViewShapeCompatibilityMatrixArtifact {
        &self.compatibility
    }

    pub fn descriptor(&self) -> &ViewShapeDescriptor {
        &self.descriptor
    }

    pub fn identity_binding(&self) -> &ViewShapeIdentityBinding {
        &self.identity_binding
    }
}

pub fn admit_view_shape(
    canonical: &CanonicalQueryBundle,
    descriptor: ViewShapeDescriptor,
) -> Result<AdmittedViewShape, ViewShapeError> {
    let compatibility = ViewShapeCompatibilityMatrixArtifact::pending(
        canonical.query().family().clone(),
        canonical.result_shape().family().clone(),
        descriptor.family(),
    );

    if descriptor.family() == ViewShapeFamily::InspectorDetailFocused
        && descriptor.native_focused_aspect_key().is_none()
    {
        return Err(ViewShapeError::new(
            ViewShapeFailureClass::FocusAspectRequired,
            "focused inspector detail requires an explicit focused aspect",
        ));
    }

    if descriptor.family() == ViewShapeFamily::KanbanGrouped
        && descriptor.native_grouping_aspect_key().is_none()
    {
        return Err(ViewShapeError::new(
            ViewShapeFailureClass::GroupingAspectRequired,
            "kanban grouped requires an explicit grouping aspect",
        ));
    }

    if descriptor.identity_consumption() != &super::identity::ViewShapeIdentityConsumption::None
        && !matches!(
            descriptor.family(),
            ViewShapeFamily::InspectorDetailObserved | ViewShapeFamily::InspectorDetailFocused
        )
    {
        return Err(ViewShapeError::new(
            ViewShapeFailureClass::IdentityConsumptionUnsupportedForViewFamily,
            "identity-aware view semantics are admitted only for inspector families in this batch",
        ));
    }

    let compatible = match descriptor.family() {
        ViewShapeFamily::Table | ViewShapeFamily::KanbanGrouped => {
            canonical.query().family() == &crate::authoring::QueryFamily::Collection
                && canonical.result_shape().family()
                    == &crate::authoring::ResultShapeFamily::Collection
        }
        ViewShapeFamily::Detail
        | ViewShapeFamily::InspectorDetailObserved
        | ViewShapeFamily::InspectorDetailFocused => {
            canonical.query().family() == &crate::authoring::QueryFamily::Detail
                && canonical.result_shape().family() == &crate::authoring::ResultShapeFamily::Detail
        }
    };

    if !compatible {
        return Err(ViewShapeError::new(
            ViewShapeFailureClass::IncompatibleCanonicalFamily,
            format!(
                "view family '{}' is incompatible with query family '{:?}' and result-shape family '{:?}'",
                descriptor.family().as_str(),
                canonical.query().family(),
                canonical.result_shape().family()
            ),
        ));
    }

    let digest = ViewShapeDigest::from_parts(&[
        format!("family:{}", descriptor.family().as_str()),
        format!("query_family:{:?}", canonical.query().family()),
        format!(
            "result_shape_family:{:?}",
            canonical.result_shape().family()
        ),
        format!(
            "focus:{}",
            descriptor
                .native_focused_aspect_key()
                .map(|key| key.as_str())
                .unwrap_or("none")
        ),
        format!(
            "grouping:{}",
            descriptor
                .native_grouping_aspect_key()
                .map(|key| key.as_str())
                .unwrap_or("none")
        ),
        format!(
            "identity_consumption:{}",
            descriptor.identity_consumption().digest().as_str()
        ),
    ]);
    let identity_binding = ViewShapeIdentityBinding::new(descriptor.identity_consumption().clone());

    Ok(AdmittedViewShape {
        descriptor,
        digest,
        compatibility: compatibility.mark_admitted(),
        identity_binding,
    })
}

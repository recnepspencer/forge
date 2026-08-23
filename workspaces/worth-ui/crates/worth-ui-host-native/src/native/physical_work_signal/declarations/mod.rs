mod aspects;
mod resources;

pub(crate) use aspects::{UiNativePhysicalSignalAspect, PHYSICAL_SIGNAL_ASPECT_COUNT};
pub(crate) use resources::{
    UiNativePhysicalSignalOperation, UiNativePhysicalSignalResourceDeclaration,
    PHYSICAL_SIGNAL_ROUTE_CAPACITY,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalDeclarations {
    pub(crate) aspects: [UiNativePhysicalSignalAspect; PHYSICAL_SIGNAL_ASPECT_COUNT],
    pub(crate) resources: [UiNativePhysicalSignalResourceDeclaration; 3],
}

impl UiNativePhysicalSignalDeclarations {
    pub(crate) const fn install() -> Self {
        Self {
            aspects: [
                UiNativePhysicalSignalAspect::HostLineage,
                UiNativePhysicalSignalAspect::WorkIdentity,
                UiNativePhysicalSignalAspect::Demand,
                UiNativePhysicalSignalAspect::Target,
                UiNativePhysicalSignalAspect::Submission,
                UiNativePhysicalSignalAspect::Recovery,
            ],
            resources: [
                UiNativePhysicalSignalResourceDeclaration::new(
                    UiNativePhysicalSignalOperation::AtlasUpload,
                ),
                UiNativePhysicalSignalResourceDeclaration::new(
                    UiNativePhysicalSignalOperation::PresentationReadback,
                ),
                UiNativePhysicalSignalResourceDeclaration::new(
                    UiNativePhysicalSignalOperation::Recovery,
                ),
            ],
        }
    }
}

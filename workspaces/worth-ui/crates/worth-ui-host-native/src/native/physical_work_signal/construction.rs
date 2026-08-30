use super::{
    declarations::UiNativePhysicalSignalDeclarations,
    identity::UiNativePhysicalSignalRuntimeIdentity, routing::UiNativePhysicalSignalRoute,
    worker::UiNativePhysicalSignalWorker,
};

pub(crate) struct UiNativePhysicalSignalConstruction {
    pub(crate) runtime_identity: UiNativePhysicalSignalRuntimeIdentity,
    #[cfg(test)]
    pub(crate) declarations: UiNativePhysicalSignalDeclarations,
    pub(crate) route: UiNativePhysicalSignalRoute,
    pub(crate) worker: UiNativePhysicalSignalWorker,
}

impl UiNativePhysicalSignalConstruction {
    pub(crate) fn build() -> Self {
        let declarations = UiNativePhysicalSignalDeclarations::install();
        Self {
            runtime_identity: UiNativePhysicalSignalRuntimeIdentity::mint(),
            worker: UiNativePhysicalSignalWorker::new(declarations),
            #[cfg(test)]
            declarations,
            route: UiNativePhysicalSignalRoute::new(),
        }
    }
}

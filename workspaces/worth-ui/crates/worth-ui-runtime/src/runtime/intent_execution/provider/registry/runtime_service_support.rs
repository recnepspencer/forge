impl super::FrozenIntentExecutionBindings {
    pub(crate) fn runtime_service_support(&self) -> crate::capability::UiRuntimeServiceSupport {
        let mut support = crate::capability::UiRuntimeServiceSupport::none_installed();
        for descriptor in &self.descriptors {
            if descriptor.support != super::UiIntentExecutionBindingSupport::Supported {
                continue;
            }
            let crate::capability::UiIntentExecutionDestination::RuntimeService(destination) =
                descriptor.destination
            else {
                continue;
            };
            match destination {
                crate::capability::UiIntentRuntimeServiceDestination::OpenPortal
                | crate::capability::UiIntentRuntimeServiceDestination::ClosePortal => {
                    support = support
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Portal)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Focus)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Scroll)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Selection)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Motion);
                }
                crate::capability::UiIntentRuntimeServiceDestination::InvokeCommand => {
                    support = support
                        .with_installed(crate::capability::UiRuntimeServiceFamily::CommandRouting);
                }
            }
        }
        support
    }
}

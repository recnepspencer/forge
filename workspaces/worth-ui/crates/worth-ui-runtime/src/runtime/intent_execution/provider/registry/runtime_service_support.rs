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
                    // Portal emits focus and motion requirements, and the focus
                    // owner may emit its one reveal requirement. Scroll owns that
                    // reveal decision, so a portal destination demands it as a
                    // compiled participant rather than leaving the reveal to an
                    // owner that a scrolling Mosaic region happened to install.
                    support = support
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Portal)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Focus)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Motion)
                        .with_installed(crate::capability::UiRuntimeServiceFamily::Scroll);
                }
                crate::capability::UiIntentRuntimeServiceDestination::InvokeCommand => {
                    // The destination consumes a route receipt; routable command
                    // capabilities install the owner that can issue one.
                }
            }
        }
        support
    }
}

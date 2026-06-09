use crate::ForgeServerSurfaceFamily;

use super::{ForgeServerRequestContext, ForgeServerTransportClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerResolvedRequestContext {
    request_context: ForgeServerRequestContext,
    surface_family: ForgeServerSurfaceFamily,
    transport_class: ForgeServerTransportClass,
}

impl ForgeServerResolvedRequestContext {
    pub(crate) fn new(
        request_context: ForgeServerRequestContext,
        surface_family: ForgeServerSurfaceFamily,
        transport_class: ForgeServerTransportClass,
    ) -> Self {
        Self {
            request_context,
            surface_family,
            transport_class,
        }
    }

    pub fn request_context(&self) -> &ForgeServerRequestContext {
        &self.request_context
    }

    pub fn surface_family(&self) -> ForgeServerSurfaceFamily {
        self.surface_family
    }

    pub fn transport_class(&self) -> ForgeServerTransportClass {
        self.transport_class
    }
}

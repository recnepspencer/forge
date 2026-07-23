#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthServerAuthenticatedPrincipal {
    principal_id: String,
    admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
}

impl WorthServerAuthenticatedPrincipal {
    pub(crate) fn new(
        principal_id: String,
        admitted_transport_caller: Option<crate::WorthServerAdmittedTransportCaller>,
    ) -> Self {
        Self {
            principal_id,
            admitted_transport_caller,
        }
    }

    pub fn principal_id(&self) -> &str {
        &self.principal_id
    }

    pub fn admitted_transport_caller(&self) -> Option<&crate::WorthServerAdmittedTransportCaller> {
        self.admitted_transport_caller.as_ref()
    }
}

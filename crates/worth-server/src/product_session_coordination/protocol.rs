pub(crate) const PRODUCT_SESSION_CREATION_REQUEST_SCHEMA_IDENTITY: &str =
    "worth-server-product-session-creation.request.v1";
pub(crate) const PRODUCT_SESSION_CLOSE_REQUEST_SCHEMA_IDENTITY: &str =
    "worth-server-product-session-close.request.v1";
pub(crate) const PRODUCT_SESSION_RESPONSE_SCHEMA_IDENTITY: &str =
    "worth-server-product-session-coordination.response.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthServerProductSessionProtocolDeclaration {
    operation_name: &'static str,
    request_schema_identity: &'static str,
    response_schema_identity: &'static str,
    requires_product_session: bool,
}

impl WorthServerProductSessionProtocolDeclaration {
    const fn new(
        operation_name: &'static str,
        request_schema_identity: &'static str,
        requires_product_session: bool,
    ) -> Self {
        Self {
            operation_name,
            request_schema_identity,
            response_schema_identity: PRODUCT_SESSION_RESPONSE_SCHEMA_IDENTITY,
            requires_product_session,
        }
    }

    pub(crate) fn operation_name(self) -> &'static str {
        self.operation_name
    }

    pub(crate) fn request_schema_identity(self) -> &'static str {
        self.request_schema_identity
    }

    pub(crate) fn response_schema_identity(self) -> &'static str {
        self.response_schema_identity
    }

    pub(crate) fn requires_product_session(self) -> bool {
        self.requires_product_session
    }
}

pub(crate) const fn product_session_protocol_declarations(
) -> [WorthServerProductSessionProtocolDeclaration; 3] {
    [
        WorthServerProductSessionProtocolDeclaration::new(
            "product_session.open_preview",
            PRODUCT_SESSION_CREATION_REQUEST_SCHEMA_IDENTITY,
            false,
        ),
        WorthServerProductSessionProtocolDeclaration::new(
            "product_session.open_mutation",
            PRODUCT_SESSION_CREATION_REQUEST_SCHEMA_IDENTITY,
            false,
        ),
        WorthServerProductSessionProtocolDeclaration::new(
            "product_session.close",
            PRODUCT_SESSION_CLOSE_REQUEST_SCHEMA_IDENTITY,
            true,
        ),
    ]
}

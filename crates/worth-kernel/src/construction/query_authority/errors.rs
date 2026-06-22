use forge_query::facade::consumer_kit::{
    ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind,
};
use forge_query::facade::{
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleInvalidContext,
};

use super::domain::PrimitiveConstructionQueryDomain;
use super::operating_context::PrimitiveConstructionOperatingContext;

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionQueryAuthorityError {
    InvalidOperatingContext(
        ForgeQueryConfiguredDomainHandleInvalidContext<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    ),
    Admission(
        ForgeQueryConfiguredDomainHandleAdmissionError<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    ),
    SupportPinning(ForgeQuerySupportPinningError),
}

impl
    From<
        ForgeQueryConfiguredDomainHandleInvalidContext<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    > for PrimitiveConstructionQueryAuthorityError
{
    fn from(
        value: ForgeQueryConfiguredDomainHandleInvalidContext<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    ) -> Self {
        Self::InvalidOperatingContext(value)
    }
}

impl
    From<
        ForgeQueryConfiguredDomainHandleAdmissionError<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    > for PrimitiveConstructionQueryAuthorityError
{
    fn from(
        value: ForgeQueryConfiguredDomainHandleAdmissionError<
            PrimitiveConstructionQueryDomain,
            PrimitiveConstructionOperatingContext,
        >,
    ) -> Self {
        Self::Admission(value)
    }
}

impl From<ForgeQuerySupportPinningError> for PrimitiveConstructionQueryAuthorityError {
    fn from(value: ForgeQuerySupportPinningError) -> Self {
        Self::SupportPinning(value)
    }
}

impl PrimitiveConstructionQueryAuthorityError {
    pub(crate) fn support_pinning_kind(&self) -> Option<ForgeQuerySupportPinningErrorKind> {
        match self {
            Self::SupportPinning(error) => Some(error.kind()),
            Self::InvalidOperatingContext(_) | Self::Admission(_) => None,
        }
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryAuthorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOperatingContext(error) => write!(f, "{error:?}"),
            Self::Admission(error) => write!(f, "{error:?}"),
            Self::SupportPinning(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryAuthorityError {}

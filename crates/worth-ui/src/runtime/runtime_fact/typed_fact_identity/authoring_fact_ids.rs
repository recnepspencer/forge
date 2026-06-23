use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeFactIdentityError {
    raw_identity: String,
}

impl WorthUiRuntimeFactIdentityError {
    fn new(raw_identity: &str) -> Self {
        Self {
            raw_identity: raw_identity.to_owned(),
        }
    }

    pub fn raw_identity(&self) -> &str {
        &self.raw_identity
    }
}

macro_rules! define_runtime_fact_identity {
    ($name:ident) => {
        #[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name {
            identity: RuntimeFactIdentityText,
        }

        impl $name {
            pub fn new(
                raw_identity: impl AsRef<str>,
            ) -> Result<Self, WorthUiRuntimeFactIdentityError> {
                Ok(Self {
                    identity: RuntimeFactIdentityText::new(raw_identity.as_ref())?,
                })
            }

            pub fn as_str(&self) -> &str {
                self.identity.as_str()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.as_str())
                    .finish()
            }
        }
    };
}

define_runtime_fact_identity!(WorthUiPageTemplateId);
define_runtime_fact_identity!(WorthUiPageInstanceId);
define_runtime_fact_identity!(WorthUiContentSlotId);
define_runtime_fact_identity!(WorthUiAppearanceRecipeId);
define_runtime_fact_identity!(WorthUiDensityTokenId);
define_runtime_fact_identity!(WorthUiActionPostureId);
define_runtime_fact_identity!(WorthUiOverlaySurfaceId);
define_runtime_fact_identity!(WorthUiToastSurfaceId);
define_runtime_fact_identity!(WorthUiInspectorSurfaceId);

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct RuntimeFactIdentityText {
    canonical: String,
}

impl RuntimeFactIdentityText {
    pub(crate) fn new(raw_identity: &str) -> Result<Self, WorthUiRuntimeFactIdentityError> {
        if is_valid_runtime_fact_identity(raw_identity) {
            Ok(Self {
                canonical: raw_identity.to_owned(),
            })
        } else {
            Err(WorthUiRuntimeFactIdentityError::new(raw_identity))
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl fmt::Debug for RuntimeFactIdentityText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RuntimeFactIdentityText")
            .field(&self.as_str())
            .finish()
    }
}

fn is_valid_runtime_fact_identity(raw_identity: &str) -> bool {
    !raw_identity.is_empty()
        && raw_identity.trim() == raw_identity
        && raw_identity
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-' | ':' | '/'))
}

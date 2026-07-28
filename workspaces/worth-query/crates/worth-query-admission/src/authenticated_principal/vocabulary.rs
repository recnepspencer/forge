const MAX_AUDIENCE_BYTES: usize = 1_024;
const MAX_METHOD_BYTES: usize = 128;

macro_rules! bounded_vocabulary {
    ($name:ident, $label:literal, $limit:expr) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(
                value: impl Into<String>,
            ) -> Result<Self, WorthQueryAuthenticationVocabularyDenial> {
                let value = value.into();
                validate($label, &value, $limit)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

bounded_vocabulary!(
    WorthQueryAuthenticationAudience,
    "authentication audience",
    MAX_AUDIENCE_BYTES
);
bounded_vocabulary!(
    WorthQueryAuthenticationMethod,
    "authentication method",
    MAX_METHOD_BYTES
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthenticationVocabularyDenial {
    field: &'static str,
}

impl WorthQueryAuthenticationVocabularyDenial {
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl std::fmt::Display for WorthQueryAuthenticationVocabularyDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid {}", self.field)
    }
}

impl std::error::Error for WorthQueryAuthenticationVocabularyDenial {}

fn validate(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), WorthQueryAuthenticationVocabularyDenial> {
    if value.is_empty()
        || value.trim() != value
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(WorthQueryAuthenticationVocabularyDenial { field });
    }
    Ok(())
}

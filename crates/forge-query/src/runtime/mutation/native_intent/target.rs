use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryParsedAspectTarget {
    aspect_key: AspectKey,
    field_path: Option<CanonicalFieldPath>,
}

impl ForgeQueryParsedAspectTarget {
    pub(crate) fn from_authoring_path(
        path: impl Into<String>,
    ) -> Result<Self, ForgeQueryParsedAspectTargetDenial> {
        let path = path.into();
        let trimmed = path.trim();
        if trimmed.is_empty() {
            return Err(ForgeQueryParsedAspectTargetDenial::EmptyPath);
        }
        if trimmed.contains(char::is_whitespace) {
            return Err(ForgeQueryParsedAspectTargetDenial::Whitespace { path: path.clone() });
        }

        let mut segments = trimmed.split('.').collect::<Vec<_>>();
        if segments.iter().any(|segment| segment.trim().is_empty()) {
            return Err(ForgeQueryParsedAspectTargetDenial::EmptySegment { path: path.clone() });
        }

        let aspect_label = segments.remove(0);
        let Some(aspect_key) = AspectKey::new(aspect_label) else {
            return Err(ForgeQueryParsedAspectTargetDenial::InvalidAspectKey {
                path: path.clone(),
                aspect_label: aspect_label.to_string(),
            });
        };

        let field_path = if segments.is_empty() {
            None
        } else {
            let mut fields = Vec::with_capacity(segments.len());
            for segment in segments {
                let Some(field_key) = FieldKey::new(segment) else {
                    return Err(ForgeQueryParsedAspectTargetDenial::InvalidFieldKey {
                        path: path.clone(),
                        field_label: segment.to_string(),
                    });
                };
                fields.push(field_key);
            }
            Some(CanonicalFieldPath::new(fields).ok_or_else(|| {
                ForgeQueryParsedAspectTargetDenial::EmptyFieldPath { path: path.clone() }
            })?)
        };

        Ok(Self {
            aspect_key,
            field_path,
        })
    }

    pub(crate) fn from_native_parts(
        aspect_key: AspectKey,
        field_path: Option<CanonicalFieldPath>,
    ) -> Self {
        Self {
            aspect_key,
            field_path,
        }
    }

    pub(crate) fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub(crate) fn field_path(&self) -> Option<&CanonicalFieldPath> {
        self.field_path.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryParsedAspectTargetDenial {
    EmptyPath,
    Whitespace { path: String },
    EmptySegment { path: String },
    InvalidAspectKey { path: String, aspect_label: String },
    InvalidFieldKey { path: String, field_label: String },
    EmptyFieldPath { path: String },
}

impl std::fmt::Display for ForgeQueryParsedAspectTargetDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPath => f.write_str("aspect target path may not be empty"),
            Self::Whitespace { path } => {
                write!(f, "aspect target path `{path}` may not contain whitespace")
            }
            Self::EmptySegment { path } => {
                write!(f, "aspect target path `{path}` contains an empty segment")
            }
            Self::InvalidAspectKey { path, aspect_label } => write!(
                f,
                "aspect target path `{path}` did not produce a foundational aspect key from `{aspect_label}`"
            ),
            Self::InvalidFieldKey { path, field_label } => write!(
                f,
                "aspect target path `{path}` did not produce a foundational field key from `{field_label}`"
            ),
            Self::EmptyFieldPath { path } => {
                write!(f, "aspect target path `{path}` produced an empty field path")
            }
        }
    }
}

impl std::error::Error for ForgeQueryParsedAspectTargetDenial {}

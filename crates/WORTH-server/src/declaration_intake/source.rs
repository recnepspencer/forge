#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerDirectDeclarationSource {
    NamedRead { operation_name: String },
    SavedQuery { saved_query_name: String },
    Template { template_name: String },
}

impl WorthServerDirectDeclarationSource {
    pub fn named_read(operation_name: impl Into<String>) -> Self {
        Self::NamedRead {
            operation_name: operation_name.into().trim().to_owned(),
        }
    }

    pub fn saved_query(saved_query_name: impl Into<String>) -> Self {
        Self::SavedQuery {
            saved_query_name: saved_query_name.into().trim().to_owned(),
        }
    }

    pub fn template(template_name: impl Into<String>) -> Self {
        Self::Template {
            template_name: template_name.into().trim().to_owned(),
        }
    }

    pub fn kind(&self) -> WorthServerDirectDeclarationSourceKind {
        match self {
            Self::NamedRead { .. } => WorthServerDirectDeclarationSourceKind::NamedRead,
            Self::SavedQuery { .. } => WorthServerDirectDeclarationSourceKind::SavedQuery,
            Self::Template { .. } => WorthServerDirectDeclarationSourceKind::Template,
        }
    }

    pub fn support_status(&self) -> WorthServerDirectDeclarationSourceSupportStatus {
        match self {
            Self::NamedRead { .. } => WorthServerDirectDeclarationSourceSupportStatus::Supported,
            Self::SavedQuery { .. } | Self::Template { .. } => {
                WorthServerDirectDeclarationSourceSupportStatus::DeferredDebt
            }
        }
    }

    pub fn support_reason(&self) -> &'static str {
        match self {
            Self::NamedRead { .. } => {
                "named read declaration intake is admitted in direct server Phase 2"
            }
            Self::SavedQuery { .. } => {
                "saved-query declaration intake remains deferred until a later direct-consumption phase"
            }
            Self::Template { .. } => {
                "template declaration intake remains deferred until a later direct-consumption phase"
            }
        }
    }

    pub(crate) fn canonical_label(&self) -> String {
        match self {
            Self::NamedRead { operation_name } => format!("named-read:{operation_name}"),
            Self::SavedQuery { saved_query_name } => format!("saved-query:{saved_query_name}"),
            Self::Template { template_name } => format!("template:{template_name}"),
        }
    }

    pub(crate) fn binding_label(&self) -> &str {
        match self {
            Self::NamedRead { operation_name } => operation_name,
            Self::SavedQuery { saved_query_name } => saved_query_name,
            Self::Template { template_name } => template_name,
        }
    }

    pub(crate) fn has_blank_binding_label(&self) -> bool {
        self.binding_label().is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectDeclarationSourceKind {
    NamedRead,
    SavedQuery,
    Template,
}

impl WorthServerDirectDeclarationSourceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NamedRead => "named-read",
            Self::SavedQuery => "saved-query",
            Self::Template => "template",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDirectDeclarationSourceSupportStatus {
    Supported,
    DeferredDebt,
    Unsupported,
}

impl WorthServerDirectDeclarationSourceSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DeferredDebt => "deferred-debt",
            Self::Unsupported => "unsupported",
        }
    }
}

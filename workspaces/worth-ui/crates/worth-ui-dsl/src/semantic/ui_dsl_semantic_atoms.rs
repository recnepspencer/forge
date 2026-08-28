macro_rules! semantic_text_wrapper {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let value = value.into();
                let trimmed = value.trim();
                assert!(
                    !trimmed.is_empty(),
                    concat!(stringify!($name), " cannot be empty"),
                );
                Self(trimmed.to_owned())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

semantic_text_wrapper!(UiDslSemanticKey);
semantic_text_wrapper!(UiDslAspectName);
semantic_text_wrapper!(UiDslStructuralToken);
semantic_text_wrapper!(UiDslPostureToken);
semantic_text_wrapper!(UiDslSupportToken);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiDslSemanticFamily {
    Page,
    PageSet,
    Region,
    Mosaic,
    LocalComposition,
    Control,
    QueryBinding,
    Intent,
    DiagnosticSurface,
    RuntimeService,
}

impl UiDslSemanticFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::PageSet => "page-set",
            Self::Region => "region",
            Self::Mosaic => "mosaic",
            Self::LocalComposition => "local-composition",
            Self::Control => "control",
            Self::QueryBinding => "query-binding",
            Self::Intent => "intent",
            Self::DiagnosticSurface => "diagnostic-surface",
            Self::RuntimeService => "runtime-service",
        }
    }
}

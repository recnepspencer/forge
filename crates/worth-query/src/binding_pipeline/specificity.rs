#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryBindingSpecificity {
    FallbackRetainedContext,
    BroadAmbientContext,
    ScopedActiveSelection,
    TypedCurrentArtifact,
    ExactExplicit,
}

impl WorthQueryBindingSpecificity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FallbackRetainedContext => "fallback_retained_context",
            Self::BroadAmbientContext => "broad_ambient_context",
            Self::ScopedActiveSelection => "scoped_active_selection",
            Self::TypedCurrentArtifact => "typed_current_artifact",
            Self::ExactExplicit => "exact_explicit",
        }
    }
}

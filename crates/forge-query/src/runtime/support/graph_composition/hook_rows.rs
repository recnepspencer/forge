use crate::identity::hash_parts;
use crate::runtime::mutation::GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphCompositionExtensionHookBoundary {
    Lowering,
    InvariantPack,
    Interpretation,
}

impl ForgeQueryGraphCompositionExtensionHookBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lowering => "lowering",
            Self::InvariantPack => "invariant-pack",
            Self::Interpretation => "interpretation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionExtensionHookSupportRow {
    hook_family: String,
    boundary: ForgeQueryGraphCompositionExtensionHookBoundary,
    semantic_bypass_allowed: bool,
    row_digest: String,
}

impl ForgeQueryGraphCompositionExtensionHookSupportRow {
    pub(crate) fn new(
        hook_family: impl Into<String>,
        boundary: ForgeQueryGraphCompositionExtensionHookBoundary,
        semantic_bypass_allowed: bool,
    ) -> Self {
        let hook_family = hook_family.into();
        let row_digest = hash_parts(&[
            format!("family:{hook_family}"),
            format!("boundary:{}", boundary.as_str()),
            format!("semantic-bypass:{semantic_bypass_allowed}"),
        ]);
        Self {
            hook_family,
            boundary,
            semantic_bypass_allowed,
            row_digest,
        }
    }

    pub fn hook_family(&self) -> &str {
        &self.hook_family
    }

    pub fn boundary(&self) -> ForgeQueryGraphCompositionExtensionHookBoundary {
        self.boundary
    }

    pub fn semantic_bypass_allowed(&self) -> bool {
        self.semantic_bypass_allowed
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn default_graph_composition_extension_hook_support_rows(
) -> Vec<ForgeQueryGraphCompositionExtensionHookSupportRow> {
    debug_assert_eq!(GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES.len(), 3);
    vec![
        ForgeQueryGraphCompositionExtensionHookSupportRow::new(
            GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES[0],
            ForgeQueryGraphCompositionExtensionHookBoundary::Lowering,
            false,
        ),
        ForgeQueryGraphCompositionExtensionHookSupportRow::new(
            GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES[1],
            ForgeQueryGraphCompositionExtensionHookBoundary::InvariantPack,
            false,
        ),
        ForgeQueryGraphCompositionExtensionHookSupportRow::new(
            GRAPH_COMPOSITION_EXTENSION_HOOK_FAMILIES[2],
            ForgeQueryGraphCompositionExtensionHookBoundary::Interpretation,
            false,
        ),
    ]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiScrollAnchorPolicy {
    StableKey,
    Clamp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiScrollDeclaration {
    identity: Box<str>,
    nested: bool,
    anchor: WorthUiScrollAnchorPolicy,
}

impl WorthUiScrollDeclaration {
    pub(super) fn parse(
        identity: &str,
        words: &[super::Word],
    ) -> Result<Self, super::WorthUiServiceDeclarationParseError> {
        super::validate_clauses(
            words,
            &[
                super::ClauseRule::Flag("nested"),
                super::ClauseRule::Single("anchor"),
            ],
        )?;
        let anchor = match super::one_value(words, "anchor")? {
            "stable_key" => WorthUiScrollAnchorPolicy::StableKey,
            "clamp" => WorthUiScrollAnchorPolicy::Clamp,
            value => {
                return Err(super::invalid(
                    "scroll anchor",
                    value,
                    "use stable_key or clamp",
                ))
            }
        };
        Ok(Self {
            identity: identity.into(),
            nested: super::optional_flag(words, "nested"),
            anchor,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn nested(&self) -> bool {
        self.nested
    }
    pub const fn anchor(&self) -> WorthUiScrollAnchorPolicy {
        self.anchor
    }
    pub(super) fn canonical_text(&self) -> String {
        format!("scroll:{}:{}:{:?}", self.identity, self.nested, self.anchor)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFocusScope {
    Workbench,
    Portal,
    Composite,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFocusDeclaration {
    identity: Box<str>,
    scope: WorthUiFocusScope,
    restore: bool,
    reveal: bool,
}

impl WorthUiFocusDeclaration {
    pub(super) fn parse(
        identity: &str,
        words: &[super::Word],
    ) -> Result<Self, super::WorthUiServiceDeclarationParseError> {
        super::validate_clauses(
            words,
            &[
                super::ClauseRule::Single("scope"),
                super::ClauseRule::Flag("restore"),
                super::ClauseRule::Flag("reveal"),
            ],
        )?;
        let scope = match super::one_value(words, "scope")? {
            "workbench" => WorthUiFocusScope::Workbench,
            "portal" => WorthUiFocusScope::Portal,
            "composite" => WorthUiFocusScope::Composite,
            value => {
                return Err(super::invalid(
                    "focus scope",
                    value,
                    "use workbench, portal, or composite",
                ))
            }
        };
        Ok(Self {
            identity: identity.into(),
            scope,
            restore: super::optional_flag(words, "restore"),
            reveal: super::optional_flag(words, "reveal"),
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn scope(&self) -> WorthUiFocusScope {
        self.scope
    }
    pub const fn restores(&self) -> bool {
        self.restore
    }
    pub const fn reveals(&self) -> bool {
        self.reveal
    }
    pub(super) fn canonical_text(&self) -> String {
        format!(
            "focus:{}:{:?}:{}:{}",
            self.identity, self.scope, self.restore, self.reveal
        )
    }
}

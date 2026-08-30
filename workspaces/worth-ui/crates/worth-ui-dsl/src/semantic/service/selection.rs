#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSelectionMode {
    Single,
    Multiple,
    Range,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSelectionDeclaration {
    identity: Box<str>,
    mode: WorthUiSelectionMode,
    item_identity: Box<str>,
    preserve_stable_key: bool,
}

impl WorthUiSelectionDeclaration {
    pub(super) fn parse(
        identity: &str,
        words: &[super::Word],
    ) -> Result<Self, super::WorthUiServiceDeclarationParseError> {
        super::validate_clauses(
            words,
            &[
                super::ClauseRule::Single("mode"),
                super::ClauseRule::Single("identity"),
                super::ClauseRule::Single("preserve"),
            ],
        )?;
        let mode = match super::one_value(words, "mode")? {
            "single" => WorthUiSelectionMode::Single,
            "multiple" => WorthUiSelectionMode::Multiple,
            "range" => WorthUiSelectionMode::Range,
            value => {
                return Err(super::invalid(
                    "selection mode",
                    value,
                    "use single, multiple, or range",
                ))
            }
        };
        let item_identity = super::one_value(words, "identity")?;
        let preserve = super::one_value(words, "preserve")?;
        if preserve != "stable_key" {
            return Err(super::invalid(
                "selection preservation",
                preserve,
                "use stable_key; row indexes are not identity",
            ));
        }
        Ok(Self {
            identity: identity.into(),
            mode,
            item_identity: item_identity.into(),
            preserve_stable_key: true,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn mode(&self) -> WorthUiSelectionMode {
        self.mode
    }
    pub fn item_identity(&self) -> &str {
        &self.item_identity
    }
    pub const fn preserves_stable_key(&self) -> bool {
        self.preserve_stable_key
    }
    pub(super) fn canonical_text(&self) -> String {
        format!(
            "selection:{}:{:?}:{}:{}",
            self.identity, self.mode, self.item_identity, self.preserve_stable_key
        )
    }
}

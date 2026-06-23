#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionKind {
    Click,
    Submit,
    Command,
    Toggle,
    Open,
    Focus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionFieldValue {
    Identifier(String),
    Number(u32),
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionField {
    name: String,
    value: WorthUiInteractionFieldValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionPayload {
    kind: WorthUiInteractionKind,
    fields: Vec<WorthUiInteractionField>,
    authored_facts_digest: u64,
}

impl WorthUiInteractionPayload {
    pub(crate) fn new(
        kind: WorthUiInteractionKind,
        mut fields: Vec<WorthUiInteractionField>,
        authored_facts_digest: u64,
    ) -> Self {
        fields.sort_by(|left, right| left.name().cmp(right.name()));
        fields.dedup();
        Self {
            kind,
            fields,
            authored_facts_digest,
        }
    }

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn fields(&self) -> &[WorthUiInteractionField] {
        &self.fields
    }

    pub fn authored_facts_digest(&self) -> u64 {
        self.authored_facts_digest
    }

    pub fn digest(&self) -> u64 {
        let mut digest = self.authored_facts_digest;
        digest = fold_digest(digest, self.kind.token().as_bytes());
        for field in &self.fields {
            digest = fold_digest(digest, field.name().as_bytes());
            digest = fold_digest(digest, field.value().digest_basis().as_bytes());
        }
        digest
    }

    pub fn field(&self, name: &str) -> Option<&WorthUiInteractionFieldValue> {
        self.fields
            .iter()
            .find(|field| field.name() == name)
            .map(WorthUiInteractionField::value)
    }
}

impl WorthUiInteractionField {
    pub(crate) fn new(name: impl Into<String>, value: WorthUiInteractionFieldValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthUiInteractionFieldValue {
        &self.value
    }
}

impl WorthUiInteractionFieldValue {
    pub fn as_text(&self) -> String {
        match self {
            Self::Identifier(value) | Self::Text(value) => value.clone(),
            Self::Number(value) => value.to_string(),
        }
    }

    fn digest_basis(&self) -> String {
        match self {
            Self::Identifier(value) => format!("identifier:{value}"),
            Self::Number(value) => format!("number:{value}"),
            Self::Text(value) => format!("text:{value}"),
        }
    }
}

impl WorthUiInteractionKind {
    pub fn token(self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::Submit => "submit",
            Self::Command => "command",
            Self::Toggle => "toggle",
            Self::Open => "open",
            Self::Focus => "focus",
        }
    }
}

pub(crate) fn fold_digest(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}

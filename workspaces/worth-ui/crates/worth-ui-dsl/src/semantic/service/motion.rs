#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReducedMotionPolicy {
    SystemRespecting,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiMotionDeclaration {
    identity: Box<str>,
    reduced_motion: WorthUiReducedMotionPolicy,
}

impl WorthUiMotionDeclaration {
    pub(super) fn parse(
        identity: &str,
        words: &[super::Word],
    ) -> Result<Self, super::WorthUiServiceDeclarationParseError> {
        super::validate_clauses(words, &[super::ClauseRule::Single("reduced")])?;
        let reduced_motion = match super::one_value(words, "reduced")? {
            "system_respecting" => WorthUiReducedMotionPolicy::SystemRespecting,
            value => {
                return Err(super::invalid(
                    "reduced motion",
                    value,
                    "use system_respecting",
                ))
            }
        };
        Ok(Self {
            identity: identity.into(),
            reduced_motion,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn reduced_motion(&self) -> WorthUiReducedMotionPolicy {
        self.reduced_motion
    }
    pub(super) fn canonical_text(&self) -> String {
        format!("motion:{}:{:?}", self.identity, self.reduced_motion)
    }
}

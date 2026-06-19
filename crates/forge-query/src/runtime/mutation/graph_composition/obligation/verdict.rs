use super::error::ForgeQueryGraphObligationDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationVerdict {
    posture: ForgeQueryGraphObligationVerdictPosture,
    context: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ForgeQueryGraphObligationVerdictPosture {
    Allow,
    Advise,
    Block,
}

impl ForgeQueryGraphObligationVerdict {
    pub fn allow() -> Self {
        Self {
            posture: ForgeQueryGraphObligationVerdictPosture::Allow,
            context: None,
        }
    }

    pub fn allow_with_context(
        context: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        let context = non_empty(context.into())?;
        Ok(Self {
            posture: ForgeQueryGraphObligationVerdictPosture::Allow,
            context: Some(context),
        })
    }

    pub fn advise(
        context: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Ok(Self {
            posture: ForgeQueryGraphObligationVerdictPosture::Advise,
            context: Some(non_empty(context.into())?),
        })
    }

    pub fn block(
        context: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        Ok(Self {
            posture: ForgeQueryGraphObligationVerdictPosture::Block,
            context: Some(non_empty(context.into())?),
        })
    }

    pub fn as_str(&self) -> &'static str {
        match self.posture {
            ForgeQueryGraphObligationVerdictPosture::Allow => "allow",
            ForgeQueryGraphObligationVerdictPosture::Advise => "advise",
            ForgeQueryGraphObligationVerdictPosture::Block => "block",
        }
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    pub fn is_allow(&self) -> bool {
        self.posture == ForgeQueryGraphObligationVerdictPosture::Allow
    }

    pub fn is_advisory(&self) -> bool {
        self.posture == ForgeQueryGraphObligationVerdictPosture::Advise
    }

    pub fn is_blocking(&self) -> bool {
        self.posture == ForgeQueryGraphObligationVerdictPosture::Block
    }
}

fn non_empty(value: String) -> Result<String, ForgeQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(ForgeQueryGraphObligationDispatchError::EmptyVerdictContext);
    }
    Ok(value)
}

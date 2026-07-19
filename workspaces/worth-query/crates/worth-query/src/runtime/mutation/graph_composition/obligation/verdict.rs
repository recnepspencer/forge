use super::error::WorthQueryGraphObligationDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationVerdict {
    posture: WorthQueryGraphObligationVerdictPosture,
    context: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthQueryGraphObligationVerdictPosture {
    Allow,
    Advise,
    Block,
}

impl WorthQueryGraphObligationVerdict {
    pub fn allow() -> Self {
        Self {
            posture: WorthQueryGraphObligationVerdictPosture::Allow,
            context: None,
        }
    }

    pub fn allow_with_context(
        context: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        let context = non_empty(context.into())?;
        Ok(Self {
            posture: WorthQueryGraphObligationVerdictPosture::Allow,
            context: Some(context),
        })
    }

    pub fn advise(
        context: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Ok(Self {
            posture: WorthQueryGraphObligationVerdictPosture::Advise,
            context: Some(non_empty(context.into())?),
        })
    }

    pub fn block(
        context: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        Ok(Self {
            posture: WorthQueryGraphObligationVerdictPosture::Block,
            context: Some(non_empty(context.into())?),
        })
    }

    pub fn as_str(&self) -> &'static str {
        match self.posture {
            WorthQueryGraphObligationVerdictPosture::Allow => "allow",
            WorthQueryGraphObligationVerdictPosture::Advise => "advise",
            WorthQueryGraphObligationVerdictPosture::Block => "block",
        }
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    pub fn is_allow(&self) -> bool {
        self.posture == WorthQueryGraphObligationVerdictPosture::Allow
    }

    pub fn is_advisory(&self) -> bool {
        self.posture == WorthQueryGraphObligationVerdictPosture::Advise
    }

    pub fn is_blocking(&self) -> bool {
        self.posture == WorthQueryGraphObligationVerdictPosture::Block
    }
}

fn non_empty(value: String) -> Result<String, WorthQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(WorthQueryGraphObligationDispatchError::EmptyVerdictContext);
    }
    Ok(value)
}

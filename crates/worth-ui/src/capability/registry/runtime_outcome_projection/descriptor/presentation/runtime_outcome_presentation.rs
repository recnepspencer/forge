use crate::capability::IconId;

use super::{RuntimeOutcomeAffordance, RuntimeOutcomeTone};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeOutcomePresentation {
    label: Option<String>,
    icon: Option<IconId>,
    tone: Option<RuntimeOutcomeTone>,
    affordance: Option<RuntimeOutcomeAffordance>,
}

impl RuntimeOutcomePresentation {
    pub fn new() -> Self {
        Self {
            label: None,
            icon: None,
            tone: None,
            affordance: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_icon(mut self, icon: IconId) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_tone(mut self, tone: RuntimeOutcomeTone) -> Self {
        self.tone = Some(tone);
        self
    }

    pub fn with_affordance(mut self, affordance: RuntimeOutcomeAffordance) -> Self {
        self.affordance = Some(affordance);
        self
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn icon(&self) -> Option<&IconId> {
        self.icon.as_ref()
    }

    pub fn tone(&self) -> Option<&RuntimeOutcomeTone> {
        self.tone.as_ref()
    }

    pub fn affordance(&self) -> Option<&RuntimeOutcomeAffordance> {
        self.affordance.as_ref()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            length_prefixed_optional(self.label()),
            length_prefixed_optional(self.icon().map(|icon| icon.as_str())),
            self.tone()
                .map(|tone| tone.digest_basis())
                .unwrap_or("none"),
            self.affordance()
                .map(|affordance| affordance.digest_basis())
                .unwrap_or("none")
        )
    }
}

impl Default for RuntimeOutcomePresentation {
    fn default() -> Self {
        Self::new()
    }
}

fn length_prefixed_optional(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("some:{}:{value}", value.len()),
        None => "none".to_string(),
    }
}

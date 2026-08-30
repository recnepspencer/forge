use super::UiRuntimeServiceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiRuntimeServiceSupportPosture {
    Installed,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiRuntimeServiceSupport {
    postures: [UiRuntimeServiceSupportPosture; 6],
}

impl UiRuntimeServiceSupport {
    pub(crate) const fn none_installed() -> Self {
        Self {
            postures: [UiRuntimeServiceSupportPosture::Unsupported; 6],
        }
    }

    #[must_use]
    pub(crate) const fn with_installed(mut self, family: UiRuntimeServiceFamily) -> Self {
        self.postures[family.index()] = UiRuntimeServiceSupportPosture::Installed;
        self
    }

    pub(crate) const fn posture(
        self,
        family: UiRuntimeServiceFamily,
    ) -> UiRuntimeServiceSupportPosture {
        self.postures[family.index()]
    }

    #[must_use]
    pub(crate) const fn union(mut self, other: Self) -> Self {
        let mut index = 0;
        while index < self.postures.len() {
            if matches!(
                other.postures[index],
                UiRuntimeServiceSupportPosture::Installed
            ) {
                self.postures[index] = UiRuntimeServiceSupportPosture::Installed;
            }
            index += 1;
        }
        self
    }
}

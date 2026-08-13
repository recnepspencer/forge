const MAXIMUM_FRAMES: usize = 32;
const MAXIMUM_CHANGES_PER_FRAME: usize = 4_096;

#[must_use]
pub struct UiNativeApplicationProgram {
    frames: Box<[UiNativeApplicationFrame]>,
}

#[must_use]
pub struct UiNativeApplicationFrame {
    component_presence: Box<[UiNativeComponentPresenceChange]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeComponentPresenceChange {
    authored_semantic_identity: Box<str>,
    present: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeApplicationProgramDenial {
    Empty,
    FrameCapacityExceeded,
    ChangeCapacityExceeded,
    InvalidComponentIdentity,
}

impl UiNativeApplicationProgram {
    pub fn new(
        frames: impl IntoIterator<Item = UiNativeApplicationFrame>,
    ) -> Result<Self, UiNativeApplicationProgramDenial> {
        let frames = frames.into_iter().collect::<Vec<_>>();
        if frames.is_empty() {
            return Err(UiNativeApplicationProgramDenial::Empty);
        }
        if frames.len() > MAXIMUM_FRAMES {
            return Err(UiNativeApplicationProgramDenial::FrameCapacityExceeded);
        }
        Ok(Self {
            frames: frames.into_boxed_slice(),
        })
    }

    pub fn single_frame() -> Self {
        Self {
            frames: Box::new([UiNativeApplicationFrame::present_current()]),
        }
    }

    pub(crate) fn frames(&self) -> &[UiNativeApplicationFrame] {
        &self.frames
    }
}

impl UiNativeApplicationFrame {
    pub fn present_current() -> Self {
        Self {
            component_presence: Box::new([]),
        }
    }

    pub fn with_component_presence(
        changes: impl IntoIterator<Item = UiNativeComponentPresenceChange>,
    ) -> Result<Self, UiNativeApplicationProgramDenial> {
        let changes = changes.into_iter().collect::<Vec<_>>();
        if changes.len() > MAXIMUM_CHANGES_PER_FRAME {
            return Err(UiNativeApplicationProgramDenial::ChangeCapacityExceeded);
        }
        Ok(Self {
            component_presence: changes.into_boxed_slice(),
        })
    }

    pub(crate) fn component_presence(&self) -> &[UiNativeComponentPresenceChange] {
        &self.component_presence
    }
}

impl UiNativeComponentPresenceChange {
    pub fn new(
        authored_semantic_identity: impl Into<Box<str>>,
        present: bool,
    ) -> Result<Self, UiNativeApplicationProgramDenial> {
        let identity = authored_semantic_identity.into();
        if !identity.starts_with("component:") || identity.len() == "component:".len() {
            return Err(UiNativeApplicationProgramDenial::InvalidComponentIdentity);
        }
        Ok(Self {
            authored_semantic_identity: identity,
            present,
        })
    }

    pub(crate) fn authored_semantic_identity(&self) -> &str {
        &self.authored_semantic_identity
    }

    pub(crate) const fn present(&self) -> bool {
        self.present
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_admission_is_bounded_and_component_semantic() {
        assert!(matches!(
            UiNativeApplicationProgram::new([]),
            Err(UiNativeApplicationProgramDenial::Empty)
        ));
        assert!(UiNativeComponentPresenceChange::new("row", true).is_err());
        let change = UiNativeComponentPresenceChange::new("component:app.row", false).unwrap();
        let frame = UiNativeApplicationFrame::with_component_presence([change]).unwrap();
        assert_eq!(
            UiNativeApplicationProgram::new([frame])
                .unwrap()
                .frames()
                .len(),
            1
        );
    }
}

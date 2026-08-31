#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPrimaryPointerKind {
    Mouse,
    Stylus,
    Touch,
}

pub(crate) const fn primary_pointer_admitted(kind: UiPrimaryPointerKind) -> bool {
    matches!(
        kind,
        UiPrimaryPointerKind::Mouse | UiPrimaryPointerKind::Stylus
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn touch_cannot_become_primary_pointer() {
        assert!(super::primary_pointer_admitted(
            super::UiPrimaryPointerKind::Mouse
        ));
        assert!(super::primary_pointer_admitted(
            super::UiPrimaryPointerKind::Stylus
        ));
        assert!(!super::primary_pointer_admitted(
            super::UiPrimaryPointerKind::Touch
        ));
    }
}

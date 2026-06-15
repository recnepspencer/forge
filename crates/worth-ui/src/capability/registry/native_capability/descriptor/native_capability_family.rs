/// Built-in native platform capability families Worth UI may route through.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NativeCapabilityFamily {
    kind: NativeCapabilityFamilyKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum NativeCapabilityFamilyKind {
    NativeMenuAdapter,
    FileDialog,
    Clipboard,
    DragDrop,
    Notification,
    Tray,
    UrlFileAssociation,
    OsTheme,
    Keychain,
    UnsupportedForDiagnostics(String),
}

impl NativeCapabilityFamily {
    pub fn native_menu_adapter() -> Self {
        Self::known(NativeCapabilityFamilyKind::NativeMenuAdapter)
    }

    pub fn file_dialog() -> Self {
        Self::known(NativeCapabilityFamilyKind::FileDialog)
    }

    pub fn clipboard() -> Self {
        Self::known(NativeCapabilityFamilyKind::Clipboard)
    }

    pub fn drag_drop() -> Self {
        Self::known(NativeCapabilityFamilyKind::DragDrop)
    }

    pub fn notification() -> Self {
        Self::known(NativeCapabilityFamilyKind::Notification)
    }

    pub fn tray() -> Self {
        Self::known(NativeCapabilityFamilyKind::Tray)
    }

    pub fn url_file_association() -> Self {
        Self::known(NativeCapabilityFamilyKind::UrlFileAssociation)
    }

    pub fn os_theme() -> Self {
        Self::known(NativeCapabilityFamilyKind::OsTheme)
    }

    pub fn keychain() -> Self {
        Self::known(NativeCapabilityFamilyKind::Keychain)
    }

    pub fn unsupported_for_diagnostics(family: impl Into<String>) -> Self {
        Self::known(NativeCapabilityFamilyKind::UnsupportedForDiagnostics(
            family.into(),
        ))
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(
            self.kind,
            NativeCapabilityFamilyKind::UnsupportedForDiagnostics(_)
        )
    }

    pub(crate) fn digest_basis(&self) -> String {
        match &self.kind {
            NativeCapabilityFamilyKind::NativeMenuAdapter => "native_menu_adapter".to_owned(),
            NativeCapabilityFamilyKind::FileDialog => "file_dialog".to_owned(),
            NativeCapabilityFamilyKind::Clipboard => "clipboard".to_owned(),
            NativeCapabilityFamilyKind::DragDrop => "drag_drop".to_owned(),
            NativeCapabilityFamilyKind::Notification => "notification".to_owned(),
            NativeCapabilityFamilyKind::Tray => "tray".to_owned(),
            NativeCapabilityFamilyKind::UrlFileAssociation => "url_file_association".to_owned(),
            NativeCapabilityFamilyKind::OsTheme => "os_theme".to_owned(),
            NativeCapabilityFamilyKind::Keychain => "keychain".to_owned(),
            NativeCapabilityFamilyKind::UnsupportedForDiagnostics(family) => {
                format!("unsupported:{family}")
            }
        }
    }

    fn known(kind: NativeCapabilityFamilyKind) -> Self {
        Self { kind }
    }
}

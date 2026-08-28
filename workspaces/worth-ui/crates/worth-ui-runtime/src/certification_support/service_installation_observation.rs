#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRuntimeServiceInstallationCertificationSnapshot {
    portal: bool,
    focus: bool,
    motion: bool,
    command_routing: bool,
    scroll: bool,
    selection: bool,
}

pub trait WorthUiRuntimeServiceInstallationCertificationExt {
    fn inspect_runtime_service_installation_for_certification(
        &self,
    ) -> UiRuntimeServiceInstallationCertificationSnapshot;
}

impl WorthUiRuntimeServiceInstallationCertificationExt
    for crate::facade::WorthUiActiveApplicationSession
{
    fn inspect_runtime_service_installation_for_certification(
        &self,
    ) -> UiRuntimeServiceInstallationCertificationSnapshot {
        crate::facade::WorthUiActiveApplicationSession::inspect_runtime_service_installation_for_certification(self)
    }
}

impl UiRuntimeServiceInstallationCertificationSnapshot {
    pub(crate) const fn new(
        portal: bool,
        focus: bool,
        motion: bool,
        command_routing: bool,
        scroll: bool,
        selection: bool,
    ) -> Self {
        Self {
            portal,
            focus,
            motion,
            command_routing,
            scroll,
            selection,
        }
    }

    pub const fn installed_family_count(self) -> usize {
        self.portal as usize
            + self.focus as usize
            + self.motion as usize
            + self.command_routing as usize
            + self.scroll as usize
            + self.selection as usize
    }

    pub const fn portal(self) -> bool {
        self.portal
    }
    pub const fn focus(self) -> bool {
        self.focus
    }
    pub const fn motion(self) -> bool {
        self.motion
    }
    pub const fn command_routing(self) -> bool {
        self.command_routing
    }
    pub const fn scroll(self) -> bool {
        self.scroll
    }
    pub const fn selection(self) -> bool {
        self.selection
    }
}

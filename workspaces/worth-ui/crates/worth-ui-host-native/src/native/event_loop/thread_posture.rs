#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UiNativeEventLoopThreadPosture {
    #[default]
    MainThreadRequired,
    CertificationWorker,
}

impl UiNativeEventLoopThreadPosture {
    #[cfg(target_os = "windows")]
    pub(super) fn configure<T>(self, builder: &mut winit::event_loop::EventLoopBuilder<T>) {
        use winit::platform::windows::EventLoopBuilderExtWindows;

        match self {
            Self::MainThreadRequired => {
                builder.with_any_thread(false);
            }
            Self::CertificationWorker => {
                builder.with_any_thread(true);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub(super) fn configure<T>(self, _builder: &mut winit::event_loop::EventLoopBuilder<T>) {}

    pub const fn label(self) -> &'static str {
        match self {
            Self::MainThreadRequired => "main-thread-required",
            Self::CertificationWorker => "certification-worker",
        }
    }
}

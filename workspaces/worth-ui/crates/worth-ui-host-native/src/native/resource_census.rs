macro_rules! native_resource_schema {
    ($( $variant:ident => $field:ident ),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) enum UiNativeResourceClass { $( $variant ),+ }

        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct UiNativeResourceCensus { $( pub $field: usize ),+ }

        impl UiNativeResourceClass {
            pub(crate) const fn all() -> &'static [Self] {
                &[$(Self::$variant),+]
            }

            const fn field_name(self) -> &'static str {
                match self { $( Self::$variant => stringify!($field) ),+ }
            }
        }

        impl UiNativeResourceCensus {
            pub fn entries(self) -> impl Iterator<Item = (&'static str, usize)> {
                [$( (stringify!($field), self.$field) ),+].into_iter()
            }

            pub fn field_names() -> impl Iterator<Item = &'static str> {
                UiNativeResourceClass::all().iter().copied().map(UiNativeResourceClass::field_name)
            }

            pub(crate) fn record(&mut self, class: UiNativeResourceClass) {
                match class { $( UiNativeResourceClass::$variant => self.$field += 1 ),+ }
            }

            pub(crate) fn max(self, other: Self) -> Self {
                Self { $( $field: self.$field.max(other.$field) ),+ }
            }
        }
    };
}

native_resource_schema! {
    Window => windows,
    Surface => surfaces,
    Adapter => adapters,
    Device => devices,
    Queue => queues,
    RetainedTarget => retained_targets,
    HostRegistration => registrations,
    ReadbackBuffer => readback_buffers,
    PendingSubmission => pending_submissions,
    EventWakeRegistration => event_wake_registrations,
    ApplicationDriver => application_drivers,
}

impl UiNativeResourceCensus {
    pub fn is_zero(self) -> bool {
        self.entries().all(|(_, count)| count == 0)
    }
}

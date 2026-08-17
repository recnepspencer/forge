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
    AlphaAtlasPage => alpha_atlas_pages,
    ColorAtlasPage => color_atlas_pages,
    AtlasStagingBuffer => atlas_staging_buffers,
    TextAtlasPlan => text_atlas_plans,
    TextAtlasReservation => text_atlas_reservations,
    TextAtlasPin => text_atlas_pins,
    TextAtlasRecovery => text_atlas_recoveries,
    TextAtlasAlphaEntry => text_atlas_alpha_entries,
    TextAtlasColorEntry => text_atlas_color_entries,
    TextAtlasUploadSubmission => text_atlas_upload_submissions,
    TextAtlasInFlightTransaction => text_atlas_in_flight_transactions,
    TextAtlasRecoveryAuthority => text_atlas_recovery_authorities,
    PhysicalSignalRuntime => physical_signal_runtimes,
    PhysicalSignalWorker => physical_signal_workers,
    PhysicalSignalPendingWork => physical_signal_pending_work,
    PhysicalSignalWake => physical_signal_wakes,
}

impl UiNativeResourceCensus {
    pub fn is_zero(self) -> bool {
        self.entries().all(|(_, count)| count == 0)
    }

    pub(crate) fn with_text_atlas(
        mut self,
        atlas: super::text_atlas::UiNativeTextAtlasCensus,
    ) -> Self {
        let super::text_atlas::UiNativeTextAtlasCensus {
            plans,
            reservations,
            pins,
            recoveries,
            alpha_pages,
            color_pages,
            alpha_entries,
            color_entries,
            staging_buffers,
            upload_submissions,
            in_flight_transactions,
            recovery_authorities,
        } = atlas;
        self.alpha_atlas_pages = self.alpha_atlas_pages.max(alpha_pages);
        self.color_atlas_pages = self.color_atlas_pages.max(color_pages);
        self.atlas_staging_buffers = self.atlas_staging_buffers.max(staging_buffers);
        self.text_atlas_plans = self.text_atlas_plans.max(plans);
        self.text_atlas_reservations = self.text_atlas_reservations.max(reservations);
        self.text_atlas_pins = self.text_atlas_pins.max(pins);
        self.text_atlas_recoveries = self.text_atlas_recoveries.max(recoveries);
        self.text_atlas_alpha_entries = self.text_atlas_alpha_entries.max(alpha_entries);
        self.text_atlas_color_entries = self.text_atlas_color_entries.max(color_entries);
        self.text_atlas_upload_submissions =
            self.text_atlas_upload_submissions.max(upload_submissions);
        self.text_atlas_in_flight_transactions = self
            .text_atlas_in_flight_transactions
            .max(in_flight_transactions);
        self.text_atlas_recovery_authorities = self
            .text_atlas_recovery_authorities
            .max(recovery_authorities);
        self
    }

    pub(crate) fn with_physical_signal(
        mut self,
        signal: super::physical_work_signal::UiNativePhysicalSignalObservation,
    ) -> Self {
        let active = usize::from(signal.runtime_owned);
        self.physical_signal_runtimes = self.physical_signal_runtimes.max(active);
        self.physical_signal_workers = self.physical_signal_workers.max(active);
        self.physical_signal_pending_work = self
            .physical_signal_pending_work
            .max(signal.active_requests);
        self.physical_signal_wakes = self.physical_signal_wakes.max(signal.pending_wakes);
        self
    }
}

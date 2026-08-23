//! Compiler-total future text-atlas resource inventory.
//!
//! Classes exist so adding a live owner later must update the schema. The
//! schema is deliberately separate from the ordinary graphics census: atlas
//! plans, pins, staging, and recovery have different lifetimes and must not be
//! hidden inside a generic resource counter.

macro_rules! text_atlas_resource_schema {
    ($($variant:ident => $field:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum UiNativeTextAtlasResourceClass { $($variant),+ }

        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct UiNativeTextAtlasCensus { $(pub $field: usize),+ }

        impl UiNativeTextAtlasResourceClass {
            pub const fn all() -> &'static [Self] {
                &[$(Self::$variant),+]
            }

            const fn field_name(self) -> &'static str {
                match self { $(Self::$variant => stringify!($field)),+ }
            }
        }

        impl UiNativeTextAtlasCensus {
            pub fn entries(self) -> impl Iterator<Item = (&'static str, usize)> {
                [$((stringify!($field), self.$field)),+].into_iter()
            }

            pub fn field_names() -> impl Iterator<Item = &'static str> {
                UiNativeTextAtlasResourceClass::all()
                    .iter()
                    .copied()
                    .map(UiNativeTextAtlasResourceClass::field_name)
            }

            pub fn is_zero(self) -> bool {
                self.entries().all(|(_, count)| count == 0)
            }
        }
    };
}

use super::settlement::UiNativeTextAtlasSnapshot;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasPhysicalPosture {
    pub(crate) alpha_pages: usize,
    pub(crate) color_pages: usize,
    pub(crate) staging_buffers: usize,
    pub(crate) upload_submissions: usize,
    pub(crate) in_flight_transactions: usize,
}

impl UiNativeTextAtlasCensus {
    pub(crate) fn from_snapshot_with_posture(
        snapshot: UiNativeTextAtlasSnapshot,
        recoveries: usize,
        physical: UiNativeTextAtlasPhysicalPosture,
    ) -> Self {
        Self {
            plans: usize::from(snapshot.reservation_active),
            reservations: if snapshot.reservation_active { 1 } else { 0 },
            pins: snapshot.pins as usize,
            recoveries,
            alpha_pages: physical.alpha_pages.max(snapshot.alpha_pages as usize),
            color_pages: physical.color_pages.max(snapshot.color_pages as usize),
            alpha_entries: snapshot.alpha_entries as usize,
            color_entries: snapshot.color_entries as usize,
            staging_buffers: physical
                .staging_buffers
                .max(usize::from(snapshot.staging_bytes != 0)),
            upload_submissions: physical.upload_submissions,
            in_flight_transactions: physical.in_flight_transactions,
            recovery_authorities: recoveries,
        }
    }
}

text_atlas_resource_schema! {
    Plan => plans,
    Reservation => reservations,
    Pin => pins,
    Recovery => recoveries,
    AlphaPage => alpha_pages,
    ColorPage => color_pages,
    AlphaEntry => alpha_entries,
    ColorEntry => color_entries,
    StagingBuffer => staging_buffers,
    UploadSubmission => upload_submissions,
    InFlightTransaction => in_flight_transactions,
    RecoveryAuthority => recovery_authorities,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn census_is_compiler_total_and_starts_at_zero() {
        let census = UiNativeTextAtlasCensus::default();
        assert!(census.is_zero());
        assert_eq!(UiNativeTextAtlasResourceClass::all().len(), 12);
        assert!(UiNativeTextAtlasCensus::field_names().any(|name| name == "plans"));
        assert!(UiNativeTextAtlasCensus::field_names().any(|name| name == "alpha_pages"));
        assert!(UiNativeTextAtlasCensus::field_names().any(|name| name == "upload_submissions"));
        assert_eq!(census.plans, 0);
        assert_eq!(census.alpha_pages, 0);
    }
}

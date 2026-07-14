macro_rules! record_layout_observation {
    ($method:ident, $family:ident, $case_id:ty, $name:ident) => {
        pub fn $method(
            &mut self,
            observed: forge_store_layout_indexes::OwnerCaseObservation<$case_id>,
        ) {
            self.record(
                super::LayoutOwnerFamily::$family,
                observed.case_id().$name(),
            );
        }
    };
}

pub(crate) use record_layout_observation;

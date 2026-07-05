use worth_ui_query_binding::WorthUiQueryMeasurementFactFamily;

pub(crate) fn query_measurement_fact_family_set_digest(
    families: &[WorthUiQueryMeasurementFactFamily],
) -> u64 {
    families
        .iter()
        .fold(0x61A7_0000_0000_0000, |digest, family| {
            digest ^ query_measurement_fact_family_digest(*family).rotate_left(7)
        })
}

const fn query_measurement_fact_family_digest(family: WorthUiQueryMeasurementFactFamily) -> u64 {
    match family {
        WorthUiQueryMeasurementFactFamily::ScrollContentExtent => 0x61A7_0000_0000_0001,
    }
}

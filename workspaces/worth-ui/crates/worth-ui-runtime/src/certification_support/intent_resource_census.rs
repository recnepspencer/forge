pub trait WorthUiIntentResourceCensusCertificationExt {
    fn intent_resource_census_for_certification(
        &self,
    ) -> crate::facade::intent::UiIntentResourceCensus;
}

impl WorthUiIntentResourceCensusCertificationExt
    for crate::facade::WorthUiActiveApplicationSession
{
    fn intent_resource_census_for_certification(
        &self,
    ) -> crate::facade::intent::UiIntentResourceCensus {
        self.snapshot_intent_resources_for_certification()
    }
}

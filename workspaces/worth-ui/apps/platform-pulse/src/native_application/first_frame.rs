use worth_ui::facade::app::UiMountedFramePublicationReceipt;
use worth_ui::facade::source::WorthUiSourcePackageRevision;

use crate::lifecycle_observation_publication::PlatformPulseObservationPublicationDenial;

use super::PlatformPulseApplicationRuntime;

impl PlatformPulseApplicationRuntime {
    pub(super) fn publish_first_frame(
        &mut self,
        source: &WorthUiSourcePackageRevision,
        publication: &UiMountedFramePublicationReceipt,
    ) -> Result<(), PlatformPulseObservationPublicationDenial> {
        self.publisher.first_frame(source, publication)?;
        self.native_input.arm_after_first_frame();
        Ok(())
    }
}

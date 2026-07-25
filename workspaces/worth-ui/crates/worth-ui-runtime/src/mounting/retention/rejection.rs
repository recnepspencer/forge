pub struct UiMountedFrameRetentionRejection {
    denial: super::UiMountedFrameRetentionDenial,
    frame: Box<super::super::UiPreparedMountedFrame>,
}

impl UiMountedFrameRetentionRejection {
    pub(crate) fn new(
        frame: super::super::UiPreparedMountedFrame,
        denial: super::UiMountedFrameRetentionDenial,
    ) -> Self {
        Self {
            denial,
            frame: Box::new(frame),
        }
    }

    pub fn denial(&self) -> super::UiMountedFrameRetentionDenial {
        self.denial
    }

    pub fn frame(&self) -> &super::super::UiPreparedMountedFrame {
        &self.frame
    }

    pub fn into_frame(self) -> super::super::UiPreparedMountedFrame {
        *self.frame
    }
}

impl std::fmt::Debug for UiMountedFrameRetentionRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiMountedFrameRetentionRejection")
            .field("denial", &self.denial)
            .finish_non_exhaustive()
    }
}

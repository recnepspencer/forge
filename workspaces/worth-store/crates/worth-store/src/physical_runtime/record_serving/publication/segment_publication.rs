use worth_store_physical_format::{
    PageGenerationCell, PersistedRecordIdentity, RecordArtifactFile, SlotGenerationCell,
};

pub(in crate::physical_runtime::record_serving) struct SegmentDataPlan {
    pub(in crate::physical_runtime::record_serving) artifact: RecordArtifactFile,
    pub(in crate::physical_runtime::record_serving) pages: Vec<PageDataPlan>,
}

pub(in crate::physical_runtime::record_serving) struct PageDataPlan {
    pub(in crate::physical_runtime::record_serving) page: PageGenerationCell,
    pub(in crate::physical_runtime::record_serving) existing_frame:
        Option<super::ExistingDataFrameImage>,
    pub(in crate::physical_runtime::record_serving) records:
        Vec<(PersistedRecordIdentity, SlotGenerationCell, Vec<u8>)>,
}

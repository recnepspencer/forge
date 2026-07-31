use worth_store_physical_format::RecordFrameCoordinate;

pub(in crate::physical_runtime::record_serving) struct ExistingDataFrameImage {
    coordinate: RecordFrameCoordinate,
    bytes: Vec<u8>,
}

impl ExistingDataFrameImage {
    pub(in crate::physical_runtime::record_serving) fn new(
        coordinate: RecordFrameCoordinate,
        bytes: Vec<u8>,
    ) -> Option<Self> {
        if bytes.len() != coordinate.length() as usize {
            return None;
        }
        Some(Self { coordinate, bytes })
    }

    pub(in crate::physical_runtime::record_serving) const fn coordinate(
        &self,
    ) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub(in crate::physical_runtime::record_serving) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(in crate::physical_runtime::record_serving) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalResidencyManifest {
    resident_artifact_keys: Vec<String>,
    in_flight_transfer_keys: Vec<String>,
}

impl CanonicalResidencyManifest {
    pub(crate) fn new(
        mut resident_artifact_keys: Vec<String>,
        mut in_flight_transfer_keys: Vec<String>,
    ) -> Self {
        resident_artifact_keys.sort();
        resident_artifact_keys.dedup();
        in_flight_transfer_keys.sort();
        in_flight_transfer_keys.dedup();
        Self {
            resident_artifact_keys,
            in_flight_transfer_keys,
        }
    }

    pub fn resident_artifact_keys(&self) -> &[String] {
        &self.resident_artifact_keys
    }

    pub fn in_flight_transfer_keys(&self) -> &[String] {
        &self.in_flight_transfer_keys
    }
}

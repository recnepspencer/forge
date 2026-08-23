use std::path::Path;

use super::artifacts::{collect_files, contains_bytes};
use super::{parent_oracle, ExpectedWriterHistory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InFlightMutationFate {
    DurableEffect,
    ProvenNoEffect,
    Indeterminate,
}

impl ExpectedWriterHistory {
    pub(crate) fn classify_in_flight_mutation(
        &self,
        root: &Path,
    ) -> Result<InFlightMutationFate, String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("canonicalize in-flight mutation root: {error}"))?;
        let mut files = Vec::new();
        collect_files(&root, &root, &mut files)?;
        let identity_present = files
            .iter()
            .any(|(_, bytes)| contains_bytes(bytes, &self.in_flight_identity()));
        let payload_present = files
            .iter()
            .any(|(_, bytes)| contains_bytes(bytes, self.in_flight_payload()));
        Ok(classify_presence(identity_present, payload_present))
    }

    pub(crate) fn classify_in_flight_from_physical_artifacts(
        &self,
        root: &Path,
    ) -> Result<InFlightMutationFate, String> {
        let root = root
            .canonicalize()
            .map_err(|error| format!("canonicalize semantic in-flight root: {error}"))?;
        let mut files = Vec::new();
        collect_files(&root, &root, &mut files)?;
        let (identity_present, payload_present) = parent_oracle::classify_in_flight_artifacts(
            &files,
            &self.in_flight_identity(),
            self.in_flight_payload(),
        )?;
        let current_root_payloads = parent_oracle::current_root_payloads(&files)?;
        Ok(
            if current_root_payloads.contains(self.in_flight_payload()) {
                InFlightMutationFate::DurableEffect
            } else {
                classify_presence(identity_present, payload_present)
            },
        )
    }
}

fn classify_presence(identity_present: bool, payload_present: bool) -> InFlightMutationFate {
    match (identity_present, payload_present) {
        (_, true) => InFlightMutationFate::DurableEffect,
        // A missing identity and payload is only missing evidence. The crash
        // contract forbids turning non-observation into a no-effect
        // conclusion; a persisted terminal cancellation fact is required for
        // ProvenNoEffect.
        (false, false) => InFlightMutationFate::Indeterminate,
        (true, false) => InFlightMutationFate::Indeterminate,
    }
}

fn classify_terminal_evidence(
    identity_present: bool,
    cancellation_terminal_present: bool,
) -> InFlightMutationFate {
    if identity_present && cancellation_terminal_present {
        InFlightMutationFate::ProvenNoEffect
    } else {
        InFlightMutationFate::Indeterminate
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpectedWriterHistory, InFlightMutationFate};

    #[test]
    fn in_flight_classifier_distinguishes_each_raw_evidence_shape() {
        let parent = tempfile::tempdir().expect("in-flight classifier parent");
        let expected = ExpectedWriterHistory::for_seeds(7, 11);
        let cases = [
            ("none", Vec::new(), InFlightMutationFate::Indeterminate),
            (
                "identity-only",
                expected.in_flight_identity().to_vec(),
                InFlightMutationFate::Indeterminate,
            ),
            (
                "payload-only",
                expected.in_flight_payload().to_vec(),
                InFlightMutationFate::DurableEffect,
            ),
        ];
        for (name, bytes, expected_fate) in cases {
            let root = parent.path().join(name);
            std::fs::create_dir(&root).expect("in-flight classifier case root");
            if !bytes.is_empty() {
                std::fs::write(root.join("evidence"), bytes).expect("in-flight evidence");
            }
            assert_eq!(
                expected.classify_in_flight_mutation(&root).unwrap(),
                expected_fate,
                "raw in-flight evidence classification drifted for {name}"
            );
        }
    }

    #[test]
    fn terminal_classifier_requires_an_explicit_cancellation_fact() {
        assert_eq!(
            super::classify_terminal_evidence(true, true),
            InFlightMutationFate::ProvenNoEffect
        );
        assert_eq!(
            super::classify_terminal_evidence(true, false),
            InFlightMutationFate::Indeterminate
        );
        assert_eq!(
            super::classify_terminal_evidence(false, false),
            InFlightMutationFate::Indeterminate
        );
    }
}

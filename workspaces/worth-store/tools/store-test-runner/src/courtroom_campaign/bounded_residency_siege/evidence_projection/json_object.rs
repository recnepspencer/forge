use serde::Serialize;

const REPORT_CAPACITY_BYTES: usize = 32 * 1024 * 1024;

pub(super) struct JsonObjectEncoder {
    encoded: Vec<u8>,
    first: bool,
}

impl JsonObjectEncoder {
    pub(super) fn new() -> Self {
        let mut encoded = Vec::with_capacity(REPORT_CAPACITY_BYTES);
        encoded.push(b'{');
        Self {
            encoded,
            first: true,
        }
    }

    pub(super) fn field(
        &mut self,
        name: &str,
        value: &(impl Serialize + ?Sized),
    ) -> Result<(), String> {
        if !self.first {
            self.encoded.push(b',');
        }
        self.first = false;
        serde_json::to_writer(&mut self.encoded, name)
            .map_err(|error| format!("cannot encode Courtroom C field name: {error}"))?;
        self.encoded.push(b':');
        serde_json::to_writer(&mut self.encoded, value)
            .map_err(|error| format!("cannot encode Courtroom C field `{name}`: {error}"))
    }

    pub(super) fn finish(mut self) -> Vec<u8> {
        self.encoded.push(b'}');
        self.encoded
    }

    #[cfg(test)]
    const fn capacity(&self) -> usize {
        self.encoded.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonObjectEncoder, REPORT_CAPACITY_BYTES};

    #[test]
    fn encoder_streams_complete_json_fields() {
        let mut encoder = JsonObjectEncoder::new();
        encoder.field("first", &7).unwrap();
        encoder.field("second", &["a", "b"]).unwrap();
        let decoded: serde_json::Value = serde_json::from_slice(&encoder.finish()).unwrap();
        assert_eq!(decoded["first"], 7);
        assert_eq!(decoded["second"], serde_json::json!(["a", "b"]));
    }

    #[test]
    fn report_encoder_reserves_one_current_report() {
        let encoder = JsonObjectEncoder::new();
        if encoder.capacity() < REPORT_CAPACITY_BYTES {
            panic!("MUTANT_PREDICATE:report-encoding-growth-copy-regressed");
        }
    }
}

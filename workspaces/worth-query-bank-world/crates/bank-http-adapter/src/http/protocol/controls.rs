use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankHttpProtocolVersion {
    V1,
    Unsupported(String),
}

impl Serialize for BankHttpProtocolVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::V1 => "v1",
            Self::Unsupported(value) => value,
        })
    }
}

impl<'de> Deserialize<'de> for BankHttpProtocolVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(if value == "v1" {
            Self::V1
        } else {
            Self::Unsupported(value)
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BankHttpRequestControls {
    pub deadline_milliseconds: u64,
    pub maximum_results: usize,
    pub maximum_work: usize,
}

impl BankHttpRequestControls {
    pub const fn new(
        deadline_milliseconds: u64,
        maximum_results: usize,
        maximum_work: usize,
    ) -> Self {
        Self {
            deadline_milliseconds,
            maximum_results,
            maximum_work,
        }
    }
}

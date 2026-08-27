use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use worth_foundational::{FoundationalBranchTargetBasis, FoundationalBranchTargetEncoding};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SignalBranchTarget {
    graph_instance_id: String,
    definition_basis: u64,
    snapshot_id: Option<u64>,
    restore_snapshot_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalBranchTargetConstructionDenial {
    EmptyGraphInstanceId,
}

impl SignalBranchTarget {
    pub fn new(
        graph_instance_id: impl Into<String>,
        definition_basis: u64,
        snapshot_id: Option<u64>,
        restore_snapshot_id: Option<u64>,
    ) -> Result<Self, SignalBranchTargetConstructionDenial> {
        let graph_instance_id = graph_instance_id.into();
        if graph_instance_id.trim().is_empty() {
            return Err(SignalBranchTargetConstructionDenial::EmptyGraphInstanceId);
        }
        Ok(Self {
            graph_instance_id,
            definition_basis,
            snapshot_id,
            restore_snapshot_id,
        })
    }

    pub fn graph_instance_id(&self) -> &str {
        &self.graph_instance_id
    }

    pub const fn definition_basis(&self) -> u64 {
        self.definition_basis
    }

    pub const fn snapshot_id(&self) -> Option<u64> {
        self.snapshot_id
    }

    pub const fn restore_snapshot_id(&self) -> Option<u64> {
        self.restore_snapshot_id
    }
}

impl<'de> Deserialize<'de> for SignalBranchTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Fields {
            graph_instance_id: String,
            definition_basis: u64,
            snapshot_id: Option<u64>,
            restore_snapshot_id: Option<u64>,
        }

        let fields = Fields::deserialize(deserializer)?;
        Self::new(
            fields.graph_instance_id,
            fields.definition_basis,
            fields.snapshot_id,
            fields.restore_snapshot_id,
        )
        .map_err(|denial| D::Error::custom(format!("invalid Signal branch target: {denial:?}")))
    }
}

impl FoundationalBranchTargetBasis for SignalBranchTarget {
    fn canonical_encoding(&self) -> FoundationalBranchTargetEncoding {
        let mut bytes = Vec::new();
        write_bytes(&mut bytes, self.graph_instance_id.as_bytes());
        bytes.extend_from_slice(&self.definition_basis.to_be_bytes());
        write_optional_u64(&mut bytes, self.snapshot_id);
        write_optional_u64(&mut bytes, self.restore_snapshot_id);
        FoundationalBranchTargetEncoding::new("worth.signal.branch-target", 1, bytes)
            .expect("static signal target encoding is valid")
    }
}

fn write_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

fn write_optional_u64(bytes: &mut Vec<u8>, value: Option<u64>) {
    match value {
        Some(value) => {
            bytes.push(1);
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        None => bytes.push(0),
    }
}

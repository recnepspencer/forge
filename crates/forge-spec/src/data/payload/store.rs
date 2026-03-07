use serde::{Deserialize, Serialize};

use crate::data::payload::PayloadKey;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadRecord {
    pub key: PayloadKey,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadStore {
    next_key: u64,
    records: Vec<PayloadRecord>,
}

impl PayloadStore {
    pub fn insert(&mut self, bytes: Vec<u8>) -> PayloadKey {
        let key = PayloadKey::new(self.next_key);
        self.next_key += 1;
        self.records.push(PayloadRecord { key, bytes });
        key
    }

    pub fn get(&self, key: PayloadKey) -> Option<&PayloadRecord> {
        self.records.iter().find(|record| record.key == key)
    }

    pub fn records(&self) -> &[PayloadRecord] {
        &self.records
    }
}

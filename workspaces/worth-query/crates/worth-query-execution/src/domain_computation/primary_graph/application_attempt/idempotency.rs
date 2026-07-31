#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationIdempotencyBinding {
    key_identity: [u8; 32],
    intent_identity: [u8; 32],
    precondition_identity: Option<[u8; 32]>,
}

impl WorthQueryApplicationIdempotencyBinding {
    pub const fn new(key_identity: [u8; 32], intent_identity: [u8; 32]) -> Self {
        Self {
            key_identity,
            intent_identity,
            precondition_identity: None,
        }
    }

    pub const fn key_identity(&self) -> &[u8; 32] {
        &self.key_identity
    }

    pub const fn intent_identity(&self) -> &[u8; 32] {
        &self.intent_identity
    }

    pub(in crate::domain_computation::primary_graph) fn key_text(self) -> String {
        encode_identity(self.key_identity)
    }

    pub(in crate::domain_computation::primary_graph) fn intent_text(self) -> String {
        let mut encoded = encode_identity(self.intent_identity);
        if let Some(precondition) = self.precondition_identity {
            encoded.push(':');
            encoded.push_str(&encode_identity(precondition));
        }
        encoded
    }

    pub(in crate::domain_computation::primary_graph) const fn bind_preconditions(
        self,
        precondition_identity: Option<&[u8; 32]>,
    ) -> Self {
        Self {
            key_identity: self.key_identity,
            intent_identity: self.intent_identity,
            precondition_identity: match precondition_identity {
                Some(identity) => Some(*identity),
                None => None,
            },
        }
    }
}

fn encode_identity(identity: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in identity {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

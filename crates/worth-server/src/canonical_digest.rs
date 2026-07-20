use sha2::{Digest, Sha256};

pub(crate) struct WorthServerCanonicalDigestBuilder {
    hasher: Sha256,
}

impl WorthServerCanonicalDigestBuilder {
    pub(crate) fn new(domain: &str) -> Self {
        let mut builder = Self {
            hasher: Sha256::new(),
        };
        builder.append_component(domain.as_bytes());
        builder
    }

    pub(crate) fn field(mut self, name: &str, value: &str) -> Self {
        self.append_component(name.as_bytes());
        self.append_component(value.as_bytes());
        self
    }

    pub(crate) fn finish(self) -> String {
        hex_digest(self.hasher.finalize().as_slice())
    }

    fn append_component(&mut self, value: &[u8]) {
        self.hasher.update((value.len() as u64).to_be_bytes());
        self.hasher.update(value);
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

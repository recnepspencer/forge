use std::path::Path;

const MAGIC: &[u8] = b"WORTH-C8-SUBMITTED-OPERATIONS-V1\n";
const OPERATION_COUNT: usize = 4;
const PAYLOAD_BYTES: usize = 8 * 1024;
const MATERIAL_BYTES: usize = 32;

pub(super) struct C8OperationProgram {
    operations: Vec<C8Operation>,
}

pub(super) struct C8Operation {
    payload: Vec<u8>,
    material: [u8; MATERIAL_BYTES],
}

impl C8OperationProgram {
    pub(super) fn operations(&self) -> &[C8Operation] {
        &self.operations
    }
}

impl C8Operation {
    pub(super) fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub(super) const fn material(&self) -> [u8; MATERIAL_BYTES] {
        self.material
    }
}

pub(super) fn read(path: &Path) -> Result<C8OperationProgram, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("read C8 submitted operation program {path:?}: {error}"))?;
    let mut cursor = Cursor::new(&bytes);
    if cursor.take(MAGIC.len())? != MAGIC {
        return Err("C8 submitted operation program has an invalid magic".to_owned());
    }
    if cursor.take_u32()? as usize != OPERATION_COUNT {
        return Err("C8 submitted operation program has an invalid operation count".to_owned());
    }
    let mut operations = Vec::with_capacity(OPERATION_COUNT);
    for _ in 0..OPERATION_COUNT {
        let material = cursor.take_array::<MATERIAL_BYTES>()?;
        if cursor.take_u64()? as usize != PAYLOAD_BYTES {
            return Err("C8 submitted operation program has an invalid payload length".to_owned());
        }
        operations.push(C8Operation {
            payload: cursor.take(PAYLOAD_BYTES)?.to_owned(),
            material,
        });
    }
    if !cursor.is_empty() {
        return Err("C8 submitted operation program has trailing bytes".to_owned());
    }
    Ok(C8OperationProgram { operations })
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| "C8 submitted operation program length overflow".to_owned())?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| "C8 submitted operation program is truncated".to_owned())?;
        self.offset = end;
        Ok(value)
    }

    fn take_u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take_array()?))
    }

    fn take_u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take_array()?))
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], String> {
        self.take(N).and_then(|value| {
            value
                .try_into()
                .map_err(|_| "C8 submitted operation program array width mismatch".to_owned())
        })
    }

    fn is_empty(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

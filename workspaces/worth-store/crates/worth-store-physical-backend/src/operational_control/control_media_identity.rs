use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use sha2::{Digest, Sha256};

use super::{ControlMediaFault, ControlMediaLocation};

const IDENTITY_MAGIC: &[u8; 8] = b"WCTID001";
const TOKEN_BYTES: usize = 32;
const CHECKSUM_BYTES: usize = 32;
const IDENTITY_BYTES: usize = IDENTITY_MAGIC.len() + TOKEN_BYTES + CHECKSUM_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlMediaIdentity([u8; 32]);

impl ControlMediaIdentity {
    pub(crate) fn open(
        location: &ControlMediaLocation,
        journal: &File,
    ) -> Result<Self, ControlMediaFault> {
        let token = load_or_create_token(location, journal.metadata()?.len() == 0)?;
        Ok(Self(fingerprint(location, journal, token)?))
    }

    pub(crate) fn observe(
        location: &ControlMediaLocation,
        journal: &File,
    ) -> Result<Self, ControlMediaFault> {
        let token = read_token(location)?;
        Ok(Self(fingerprint(location, journal, token)?))
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

fn load_or_create_token(
    location: &ControlMediaLocation,
    empty_journal: bool,
) -> Result<[u8; TOKEN_BYTES], ControlMediaFault> {
    match read_token(location) {
        Ok(token) => Ok(token),
        Err(ControlMediaFault::MissingControlMediaIdentity) if empty_journal => {
            create_token(location)
        }
        Err(error) => Err(error),
    }
}

fn create_token(location: &ControlMediaLocation) -> Result<[u8; TOKEN_BYTES], ControlMediaFault> {
    let mut token = [0; TOKEN_BYTES];
    getrandom::fill(&mut token).map_err(|_| ControlMediaFault::IdentityEntropyUnavailable)?;
    let bytes = encode_identity(token);
    let path = location.identity_path();
    match OpenOptions::new().create_new(true).write(true).open(&path) {
        Ok(mut file) => {
            file.write_all(&bytes)?;
            file.sync_all()?;
            if let Some(parent) = path.parent() {
                crate::directory_durability::sync_directory(parent)?;
            }
            Ok(token)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => read_token(location),
        Err(error) => Err(error.into()),
    }
}

fn read_token(location: &ControlMediaLocation) -> Result<[u8; TOKEN_BYTES], ControlMediaFault> {
    let path = location.identity_path();
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(ControlMediaFault::MissingControlMediaIdentity)
        }
        Err(error) => return Err(error.into()),
    };
    if file.metadata()?.len() != IDENTITY_BYTES as u64 {
        return Err(ControlMediaFault::CorruptControlMediaIdentity);
    }
    let mut bytes = [0; IDENTITY_BYTES];
    file.read_exact(&mut bytes)?;
    decode_identity(bytes)
}

fn encode_identity(token: [u8; TOKEN_BYTES]) -> [u8; IDENTITY_BYTES] {
    let mut bytes = [0; IDENTITY_BYTES];
    bytes[..IDENTITY_MAGIC.len()].copy_from_slice(IDENTITY_MAGIC);
    bytes[IDENTITY_MAGIC.len()..IDENTITY_MAGIC.len() + TOKEN_BYTES].copy_from_slice(&token);
    let checksum = Sha256::digest(&bytes[..IDENTITY_MAGIC.len() + TOKEN_BYTES]);
    bytes[IDENTITY_MAGIC.len() + TOKEN_BYTES..].copy_from_slice(&checksum);
    bytes
}

fn decode_identity(bytes: [u8; IDENTITY_BYTES]) -> Result<[u8; TOKEN_BYTES], ControlMediaFault> {
    let payload_end = IDENTITY_MAGIC.len() + TOKEN_BYTES;
    if &bytes[..IDENTITY_MAGIC.len()] != IDENTITY_MAGIC
        || Sha256::digest(&bytes[..payload_end]).as_slice() != &bytes[payload_end..]
    {
        return Err(ControlMediaFault::CorruptControlMediaIdentity);
    }
    let mut token = [0; TOKEN_BYTES];
    token.copy_from_slice(&bytes[IDENTITY_MAGIC.len()..payload_end]);
    Ok(token)
}

fn fingerprint(
    location: &ControlMediaLocation,
    journal: &File,
    token: [u8; TOKEN_BYTES],
) -> Result<[u8; 32], ControlMediaFault> {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-control-media-v1\0");
    digest.update(token);
    update_file_identity(&mut digest, location, journal)?;
    Ok(digest.finalize().into())
}

#[cfg(unix)]
fn update_file_identity(
    digest: &mut Sha256,
    _location: &ControlMediaLocation,
    file: &File,
) -> Result<(), ControlMediaFault> {
    use std::os::unix::fs::MetadataExt;

    let metadata = file.metadata()?;
    digest.update([1]);
    digest.update(metadata.dev().to_le_bytes());
    digest.update(metadata.ino().to_le_bytes());
    Ok(())
}

#[cfg(windows)]
fn update_file_identity(
    digest: &mut Sha256,
    location: &ControlMediaLocation,
    _file: &File,
) -> Result<(), ControlMediaFault> {
    match file_id::get_file_id(location.path())? {
        file_id::FileId::Inode {
            device_id,
            inode_number,
        } => {
            digest.update([1]);
            digest.update(device_id.to_le_bytes());
            digest.update(inode_number.to_le_bytes());
        }
        file_id::FileId::LowRes {
            volume_serial_number,
            file_index,
        } => {
            digest.update([2]);
            digest.update(volume_serial_number.to_le_bytes());
            digest.update(file_index.to_le_bytes());
        }
        file_id::FileId::HighRes {
            volume_serial_number,
            file_id,
        } => {
            digest.update([3]);
            digest.update(volume_serial_number.to_le_bytes());
            digest.update(file_id.to_le_bytes());
        }
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn update_file_identity(
    _digest: &mut Sha256,
    _location: &ControlMediaLocation,
    _file: &File,
) -> Result<(), ControlMediaFault> {
    Err(ControlMediaFault::ControlMediaIdentityUnavailable)
}

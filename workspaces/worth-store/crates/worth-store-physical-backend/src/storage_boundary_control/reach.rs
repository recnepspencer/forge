use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};

use crate::ProductionStorageBoundarySeam;

use super::{ProductionStorageBoundaryControl, StorageBoundaryFault, StorageBoundaryRegion};

pub fn reach_storage_boundary(
    control: &impl ProductionStorageBoundaryControl,
    seam: ProductionStorageBoundarySeam,
    file: &mut File,
    region: StorageBoundaryRegion,
) -> io::Result<()> {
    control.record_reached(seam);
    let Some(fault) = control.fault_at(seam) else {
        return Ok(());
    };
    apply_storage_fault(file, region, fault)?;
    control.record_injected(seam, fault);
    Err(io::Error::new(
        io::ErrorKind::Interrupted,
        format!("injected storage fault at {}", seam.token()),
    ))
}

fn apply_storage_fault(
    file: &mut File,
    region: StorageBoundaryRegion,
    fault: StorageBoundaryFault,
) -> io::Result<()> {
    match fault {
        StorageBoundaryFault::Interrupt | StorageBoundaryFault::AbortBeforeDurabilityBarrier => {
            Ok(())
        }
        StorageBoundaryFault::TearWrite { retained_bytes } => file.set_len(
            region
                .offset()
                .saturating_add(retained_bytes.min(region.bytes())),
        ),
        StorageBoundaryFault::CorruptByte {
            relative_offset,
            xor_mask,
        } => corrupt_region_byte(file, region, relative_offset, xor_mask),
    }
}

fn corrupt_region_byte(
    file: &mut File,
    region: StorageBoundaryRegion,
    relative_offset: u64,
    xor_mask: u8,
) -> io::Result<()> {
    if relative_offset >= region.bytes() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "corruption offset is outside the persisted region",
        ));
    }
    let absolute_offset = region.offset() + relative_offset;
    file.seek(SeekFrom::Start(absolute_offset))?;
    let mut byte = [0_u8; 1];
    std::io::Read::read_exact(file, &mut byte)?;
    byte[0] ^= xor_mask;
    file.seek(SeekFrom::Start(absolute_offset))?;
    file.write_all(&byte)
}

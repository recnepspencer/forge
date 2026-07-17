use std::io::Read;
use std::path::{Path, PathBuf};

use worth_store_physical_backend::{
    OfflineMediaConsistencyBasis, OfflineMediaReadDenial, ReadOnlyOfflineMediaCapability,
};

use crate::{OfflineInspectionBudget, OfflineInspectionCancellation, OfflineInspectionDenial};

pub(super) struct OwnerMediaReadSession {
    media: ReadOnlyOfflineMediaCapability,
    budget: OfflineInspectionBudget,
    cancellation: OfflineInspectionCancellation,
    started_at: std::time::Instant,
}

impl OwnerMediaReadSession {
    pub(super) fn open(
        paths: impl IntoIterator<Item = PathBuf>,
        basis: OfflineMediaConsistencyBasis,
        budget: OfflineInspectionBudget,
        cancellation: OfflineInspectionCancellation,
        started_at: std::time::Instant,
    ) -> Result<Self, OfflineMediaReadDenial> {
        Ok(Self {
            media: ReadOnlyOfflineMediaCapability::open_bounded(
                paths,
                basis,
                budget.maximum_owned_allocation_bytes(),
            )?,
            budget,
            cancellation,
            started_at,
        })
    }

    pub(super) fn reader(
        &mut self,
        path: &Path,
    ) -> Result<OwnerArtifactReader<'_>, OfflineMediaReadDenial> {
        let file_index = self
            .media
            .file_index(path)
            .ok_or(OfflineMediaReadDenial::InvalidFileIndex)?;
        let length = self
            .media
            .file(file_index)
            .ok_or(OfflineMediaReadDenial::InvalidFileIndex)?
            .length();
        Ok(OwnerArtifactReader {
            media: &mut self.media,
            file_index,
            length,
            offset: 0,
            budget: self.budget,
            cancellation: &self.cancellation,
            started_at: self.started_at,
            denial: None,
        })
    }

    pub(super) fn revalidate_consistency(&self) -> Result<(), OfflineMediaReadDenial> {
        self.media.revalidate_consistency()
    }

    pub(super) fn reject_interruption(&self) -> Result<(), OfflineInspectionDenial> {
        crate::inspection::reject_inspection_interruption(
            self.budget,
            &self.cancellation,
            self.started_at,
        )
    }

    pub(super) const fn resident_owned_allocation_bytes(&self) -> u64 {
        self.media.resident_owned_allocation_bytes()
    }

    pub(super) const fn peak_owned_allocation_bytes(&self) -> u64 {
        self.media.peak_owned_allocation_bytes()
    }
}

pub(super) struct OwnerArtifactReader<'a> {
    media: &'a mut ReadOnlyOfflineMediaCapability,
    file_index: usize,
    length: u64,
    offset: u64,
    budget: OfflineInspectionBudget,
    cancellation: &'a OfflineInspectionCancellation,
    started_at: std::time::Instant,
    denial: Option<OfflineInspectionDenial>,
}

impl OwnerArtifactReader<'_> {
    pub(super) const fn length(&self) -> u64 {
        self.length
    }

    pub(super) fn finish(self) -> Result<u64, OfflineInspectionDenial> {
        match self.denial {
            Some(denial) => Err(denial),
            None => Ok(self.offset),
        }
    }

    fn reject_interruption(&self) -> Result<(), OfflineInspectionDenial> {
        crate::inspection::reject_inspection_interruption(
            self.budget,
            self.cancellation,
            self.started_at,
        )
    }

    fn reject(&mut self, denial: OfflineInspectionDenial) -> std::io::Error {
        self.denial = Some(denial);
        std::io::Error::other("offline owner media read denied")
    }
}

impl Read for OwnerArtifactReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        if buffer.is_empty() || self.offset == self.length {
            return Ok(0);
        }
        if let Err(denial) = self.reject_interruption() {
            return Err(self.reject(denial));
        }
        let remaining = self.length - self.offset;
        let take = usize::try_from(remaining)
            .unwrap_or(usize::MAX)
            .min(buffer.len());
        let observation =
            match self
                .media
                .read_bounded_into(self.file_index, self.offset, &mut buffer[..take])
            {
                Ok(observation) => observation,
                Err(denial) => return Err(self.reject(OfflineInspectionDenial::Media(denial))),
            };
        let read = observation.bytes_read();
        self.offset = self
            .offset
            .checked_add(read as u64)
            .ok_or_else(|| self.reject(OfflineInspectionDenial::CounterOverflow))?;
        Ok(read)
    }
}

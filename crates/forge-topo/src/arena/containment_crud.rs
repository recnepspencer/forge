//! CRUD operations for containment entities: Shell, Body, Lump, Region.
//!
//! DOMAIN: Insert, get, remove, iterate, count, and version/generation
//! queries for the ownership hierarchy entities.

use forge_core::KernelError;

use crate::arena::core::TopologyArena;
use crate::arena::slot::{validate_generation, cold_err_bounds, cold_err_deleted};
use crate::arena::containment_schema::*;
use crate::handles::{ShellId, BodyId, LumpId, RegionId};

impl TopologyArena {
    // ── Shell ───────────────────────────────────────────────────

    /// Insert a new shell, returning its handle.
    pub(crate) fn insert_shell(&mut self, data: ShellData) -> ShellId {
        let (index, gen) = Self::insert_slot(&mut self.shell_slots, &mut self.free_shell_head, data);
        self.active_shell_count += 1;
        ShellId::new(index, gen)
    }

    /// Get a shell by handle, validating the generation.
    #[inline]
    pub fn get_shell(&self, id: ShellId) -> Result<&ShellData, KernelError> {
        let slot = self.shell_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a shell by handle.
    #[inline]
    pub fn get_shell_mut(&mut self, id: ShellId) -> Result<&mut ShellData, KernelError> {
        let slot = self.shell_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))
    }

    /// Remove a shell, bumping the generation of its slot.
    pub(crate) fn remove_shell(&mut self, id: ShellId) -> Result<ShellData, KernelError> {
        let slot = self.shell_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Shell", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Shell", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Shell", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_shell_head;
        self.free_shell_head = Some(id.index());
        self.active_shell_count -= 1;
        Ok(data)
    }

    /// Iterate over all active shells, yielding `(ShellId, &ShellData)` pairs.
    pub fn iter_shells(&self) -> impl Iterator<Item = (ShellId, &ShellData)> {
        self.shell_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((ShellId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active shells.
    pub fn shell_count(&self) -> usize { self.active_shell_count }

    /// Indices of all active (occupied) shell slots.
    pub fn active_shell_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.shell_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of shell at slot index, or None if vacant/out-of-bounds.
    pub fn shell_generation(&self, index: usize) -> Option<u32> {
        self.shell_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of shell at slot index, or None if vacant/out-of-bounds.
    pub fn shell_version(&self, index: usize) -> Option<u32> {
        self.shell_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Body ────────────────────────────────────────────────────

    /// Insert a new solid, returning its handle.
    pub(crate) fn insert_body(&mut self, data: BodyData) -> BodyId {
        let (index, gen) = Self::insert_slot(&mut self.body_slots, &mut self.free_body_head, data);
        self.active_body_count += 1;
        BodyId::new(index, gen)
    }

    /// Get a solid by handle, validating the generation.
    #[inline]
    pub fn get_body(&self, id: BodyId) -> Result<&BodyData, KernelError> {
        let slot = self.body_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a solid by handle.
    #[inline]
    pub fn get_body_mut(&mut self, id: BodyId) -> Result<&mut BodyData, KernelError> {
        let slot = self.body_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))
    }

    /// Remove a solid, bumping the generation of its slot.
    pub(crate) fn remove_body(&mut self, id: BodyId) -> Result<BodyData, KernelError> {
        let slot = self.body_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Body", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Body", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Body", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_body_head;
        self.free_body_head = Some(id.index());
        self.active_body_count -= 1;
        Ok(data)
    }

    /// Iterate over all active solids, yielding `(BodyId, &BodyData)` pairs.
    pub fn iter_bodies(&self) -> impl Iterator<Item = (BodyId, &BodyData)> {
        self.body_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((BodyId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active solids.
    pub fn body_count(&self) -> usize { self.active_body_count }

    /// Indices of all active (occupied) solid slots.
    pub fn active_body_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.body_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of solid at slot index, or None if vacant/out-of-bounds.
    pub fn body_generation(&self, index: usize) -> Option<u32> {
        self.body_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of solid at slot index, or None if vacant/out-of-bounds.
    pub fn body_version(&self, index: usize) -> Option<u32> {
        self.body_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Lump ────────────────────────────────────────────────────

    /// Insert a new lump, returning its handle.
    pub(crate) fn insert_lump(&mut self, data: LumpData) -> LumpId {
        let (index, gen) = Self::insert_slot(&mut self.lump_slots, &mut self.free_lump_head, data);
        self.active_lump_count += 1;
        LumpId::new(index, gen)
    }

    /// Get a lump by handle, validating the generation.
    #[inline]
    pub fn get_lump(&self, id: LumpId) -> Result<&LumpData, KernelError> {
        let slot = self.lump_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a lump by handle.
    #[inline]
    pub fn get_lump_mut(&mut self, id: LumpId) -> Result<&mut LumpData, KernelError> {
        let slot = self.lump_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))
    }

    /// Remove a lump, bumping the generation of its slot.
    pub(crate) fn remove_lump(&mut self, id: LumpId) -> Result<LumpData, KernelError> {
        let slot = self.lump_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Lump", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Lump", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Lump", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_lump_head;
        self.free_lump_head = Some(id.index());
        self.active_lump_count -= 1;
        Ok(data)
    }

    /// Iterate over all active lumps, yielding `(LumpId, &LumpData)` pairs.
    pub fn iter_lumps(&self) -> impl Iterator<Item = (LumpId, &LumpData)> {
        self.lump_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((LumpId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active lumps.
    pub fn lump_count(&self) -> usize { self.active_lump_count }

    /// Indices of all active (occupied) lump slots.
    pub fn active_lump_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.lump_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of lump at slot index, or None if vacant/out-of-bounds.
    pub fn lump_generation(&self, index: usize) -> Option<u32> {
        self.lump_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of lump at slot index, or None if vacant/out-of-bounds.
    pub fn lump_version(&self, index: usize) -> Option<u32> {
        self.lump_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }

    // ── Region ──────────────────────────────────────────────────

    /// Insert a new region, returning its handle.
    pub(crate) fn insert_region(&mut self, data: RegionData) -> RegionId {
        let (index, gen) = Self::insert_slot(&mut self.region_slots, &mut self.free_region_head, data);
        self.active_region_count += 1;
        RegionId::new(index, gen)
    }

    /// Get a region by handle, validating the generation.
    #[inline]
    pub fn get_region(&self, id: RegionId) -> Result<&RegionData, KernelError> {
        let slot = self.region_slots.get(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        slot.data.as_ref()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))
    }

    /// Get a mutable reference to a region by handle.
    #[inline]
    pub fn get_region_mut(&mut self, id: RegionId) -> Result<&mut RegionData, KernelError> {
        let slot = self.region_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        slot.version += 1;
        slot.data.as_mut()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))
    }

    /// Remove a region, bumping the generation of its slot.
    pub(crate) fn remove_region(&mut self, id: RegionId) -> Result<RegionData, KernelError> {
        let slot = self.region_slots.get_mut(id.index() as usize)
            .ok_or_else(|| cold_err_bounds("Region", id.index(), id.generation()))?;
        validate_generation(slot.generation, id.generation(), "Region", id.index())?;
        let data = slot.data.take()
            .ok_or_else(|| cold_err_deleted("Region", id.index(), id.generation(), slot.generation))?;
        slot.generation += 1;
        slot.next_free = self.free_region_head;
        self.free_region_head = Some(id.index());
        self.active_region_count -= 1;
        Ok(data)
    }

    /// Iterate over all active regions, yielding `(RegionId, &RegionData)` pairs.
    pub fn iter_regions(&self) -> impl Iterator<Item = (RegionId, &RegionData)> {
        self.region_slots.iter().enumerate().filter_map(|(i, slot)| {
            let data = slot.data.as_ref()?;
            Some((RegionId::new(i as u32, slot.generation), data))
        })
    }

    /// Count of active regions.
    pub fn region_count(&self) -> usize { self.active_region_count }

    /// Indices of all active (occupied) region slots.
    pub fn active_region_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.region_slots.iter().enumerate()
            .filter_map(|(i, s)| s.data.as_ref().map(|_| i))
    }

    /// Generation of region at slot index, or None if vacant/out-of-bounds.
    pub fn region_generation(&self, index: usize) -> Option<u32> {
        self.region_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.generation))
    }

    /// Version of region at slot index, or None if vacant/out-of-bounds.
    pub fn region_version(&self, index: usize) -> Option<u32> {
        self.region_slots.get(index).and_then(|s| s.data.as_ref().map(|_| s.version))
    }
}

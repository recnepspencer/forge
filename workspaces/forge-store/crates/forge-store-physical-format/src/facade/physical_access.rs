use super::{PlatformPhysicalFacade, PlatformPhysicalFacadeDenial};
use crate::layout_access::{
    allocation_family::{AdmittedAllocationLayoutFamily, AllocationLayoutFamilyHome},
    extent_family::{AdmittedExtentLayoutFamily, ExtentLayoutFamilyHome},
    fragmentation_family::{AdmittedFragmentationLayoutFamily, FragmentationLayoutFamilyHome},
    frame_family::{AdmittedFrameLayoutFamily, FrameLayoutFamilyHome},
    free_space_family::{AdmittedFreeSpaceLayoutFamily, FreeSpaceLayoutFamilyHome},
    manifest_family::{AdmittedManifestLayoutFamily, ManifestLayoutFamilyHome},
    page_family::{AdmittedPageLayoutFamily, PageLayoutFamilyHome},
    root_discovery_family::{AdmittedRootDiscoveryLayoutFamily, RootDiscoveryLayoutFamilyHome},
    segment_family::{AdmittedSegmentLayoutFamily, SegmentLayoutFamilyHome},
};

impl PlatformPhysicalFacade {
    pub fn page_layout(
        &mut self,
    ) -> Result<AdmittedPageLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = PageLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedPageLayoutFamily::new(self, admission))
    }

    pub fn frame_layout(
        &mut self,
    ) -> Result<AdmittedFrameLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = FrameLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedFrameLayoutFamily::new(self, admission))
    }

    pub fn segment_layout(
        &mut self,
    ) -> Result<AdmittedSegmentLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = SegmentLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedSegmentLayoutFamily::new(self, admission))
    }

    pub fn extent_layout(
        &mut self,
    ) -> Result<AdmittedExtentLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = ExtentLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedExtentLayoutFamily::new(self, admission))
    }

    pub fn root_manifest_layout(
        &mut self,
    ) -> Result<AdmittedRootDiscoveryLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = RootDiscoveryLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedRootDiscoveryLayoutFamily::new(self, admission))
    }

    pub fn manifest_index_layout(
        &mut self,
    ) -> Result<AdmittedManifestLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = ManifestLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedManifestLayoutFamily::new(self, admission))
    }

    pub fn allocation_layout(
        &mut self,
    ) -> Result<AdmittedAllocationLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = AllocationLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedAllocationLayoutFamily::new(self, admission))
    }

    pub fn free_space_layout(
        &mut self,
    ) -> Result<AdmittedFreeSpaceLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = FreeSpaceLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedFreeSpaceLayoutFamily::new(self, admission))
    }

    pub fn fragmentation_layout(
        &mut self,
    ) -> Result<AdmittedFragmentationLayoutFamily<'_>, PlatformPhysicalFacadeDenial> {
        let admission = FragmentationLayoutFamilyHome::physical().admit()?;
        Ok(AdmittedFragmentationLayoutFamily::new(self, admission))
    }
}

use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiCompositionRootKind, WorthUiCompositionRootMountAuthoritySet,
    WorthUiCompositionRootMountDenial, WorthUiCompositionRootMountDenialCode,
    WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport,
    WorthUiCompositionRootMountResolvedAuthority, WorthUiCompositionRootReceipt,
    WorthUiPageHostPlan, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

impl WorthUiRuntimeHost {
    pub fn admit_composition_root_mount(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
        root: &WorthUiCompositionRootReceipt,
    ) -> Result<WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport> {
        let mosaic_legality = self.admit_mosaic_placement_for_page(page_host_plan);
        let authorities = WorthUiCompositionRootMountAuthoritySet::from_page_plan(
            page_host_plan.clone(),
            mosaic_legality,
        );
        self.admit_composition_root_mount_with_authority(&authorities, root)
    }

    pub fn admit_composition_root_mount_with_authority(
        &self,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
        root: &WorthUiCompositionRootReceipt,
    ) -> Result<WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport> {
        if !authorities.mosaic_legality().admitted() {
            return Err(self.mosaic_placement_denied_report(authorities, root));
        }
        match root.kind() {
            WorthUiCompositionRootKind::Surface => self.admit_surface_root_mount(authorities, root),
            WorthUiCompositionRootKind::PageContentSlot => {
                self.admit_page_content_slot_root_mount(authorities, root)
            }
            WorthUiCompositionRootKind::ComponentInstance
            | WorthUiCompositionRootKind::PortalEntry
            | WorthUiCompositionRootKind::CollectionItem
            | WorthUiCompositionRootKind::DiagnosticPanel => {
                self.admit_external_root_mount(authorities, root)
            }
        }
    }

    fn admit_surface_root_mount(
        &self,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
        root: &WorthUiCompositionRootReceipt,
    ) -> Result<WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport> {
        let surface_id = parse_surface_id(root, root.authority_identity())?;
        let Some(surface) = self.inspect_active_surface_descriptor(&surface_id) else {
            return Err(report_denial(WorthUiCompositionRootMountDenial::new(
                WorthUiCompositionRootMountDenialCode::MissingSurface,
                root,
                surface_id.as_str(),
                vec![root.fact_id().clone()],
            )));
        };
        let authority = WorthUiCompositionRootMountResolvedAuthority::Surface {
            surface_id: surface_id.clone(),
            component_id: surface.component_id().clone(),
        };
        let mut consumed_facts = surface_root_facts(root, &surface_id);
        consumed_facts.extend(
            authorities
                .mosaic_legality()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        Ok(root_mount_receipt(
            self,
            root,
            authority,
            consumed_facts,
            0,
            0,
            1,
        ))
    }

    fn admit_page_content_slot_root_mount(
        &self,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
        root: &WorthUiCompositionRootReceipt,
    ) -> Result<WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport> {
        let Some(slot_mount) = authorities
            .page_host_plan()
            .resolve_slot_mount(root.authority_identity())
        else {
            return Err(report_denial(WorthUiCompositionRootMountDenial::new(
                WorthUiCompositionRootMountDenialCode::MissingPageSlot,
                root,
                root.authority_identity(),
                vec![root.fact_id().clone()],
            )));
        };
        let surface_id = parse_surface_id(root, slot_mount.surface_id())?;
        let Some(surface) = self.inspect_active_surface_descriptor(&surface_id) else {
            return Err(report_denial(WorthUiCompositionRootMountDenial::new(
                WorthUiCompositionRootMountDenialCode::MissingSurface,
                root,
                surface_id.as_str(),
                vec![root.fact_id().clone()],
            )));
        };
        let authority = WorthUiCompositionRootMountResolvedAuthority::PageContentSlot {
            page_name: slot_mount.page_name().to_owned(),
            slot_name: slot_mount.slot_name().to_owned(),
            surface_id: surface_id.clone(),
            component_id: surface.component_id().clone(),
        };
        let mut consumed_facts = vec![root.fact_id().clone()];
        consumed_facts.extend(slot_mount.consumed_facts().iter().cloned());
        consumed_facts.extend(
            authorities
                .mosaic_legality()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        Ok(root_mount_receipt(
            self,
            root,
            authority,
            consumed_facts,
            1,
            0,
            1,
        ))
    }

    fn admit_external_root_mount(
        &self,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
        root: &WorthUiCompositionRootReceipt,
    ) -> Result<WorthUiCompositionRootMountReceipt, WorthUiCompositionRootMountReport> {
        let Some(external) = authorities.external_authority(root.kind(), root.authority_identity())
        else {
            return Err(missing_root_authority_report(
                missing_authority_code(root.kind()),
                root,
            ));
        };
        let authority = WorthUiCompositionRootMountResolvedAuthority::External {
            kind: external.kind(),
            authority_identity: external.authority_identity().to_owned(),
            surface_id: external.surface_id().clone(),
            component_id: external.component_id().clone(),
        };
        let mut consumed_facts = vec![root.fact_id().clone()];
        consumed_facts.extend(external.consumed_facts().iter().cloned());
        consumed_facts.extend(
            authorities
                .mosaic_legality()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        Ok(root_mount_receipt(
            self,
            root,
            authority,
            consumed_facts,
            0,
            0,
            1,
        ))
    }

    fn mosaic_placement_denied_report(
        &self,
        authorities: &WorthUiCompositionRootMountAuthoritySet,
        root: &WorthUiCompositionRootReceipt,
    ) -> WorthUiCompositionRootMountReport {
        let mut facts = vec![root.fact_id().clone()];
        facts.extend(
            authorities
                .mosaic_legality()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        report_denial(WorthUiCompositionRootMountDenial::new(
            WorthUiCompositionRootMountDenialCode::MosaicPlacementDenied,
            root,
            authorities.mosaic_legality().page_name(),
            facts,
        ))
    }
}

fn parse_surface_id(
    root: &WorthUiCompositionRootReceipt,
    raw_surface_id: &str,
) -> Result<SurfaceId, WorthUiCompositionRootMountReport> {
    SurfaceId::new(raw_surface_id).map_err(|_| {
        report_denial(WorthUiCompositionRootMountDenial::new(
            WorthUiCompositionRootMountDenialCode::InvalidSurfaceId,
            root,
            raw_surface_id,
            vec![root.fact_id().clone()],
        ))
    })
}

fn root_mount_receipt(
    runtime: &WorthUiRuntimeHost,
    root: &WorthUiCompositionRootReceipt,
    authority: WorthUiCompositionRootMountResolvedAuthority,
    facts: Vec<WorthUiRuntimeFactId>,
    page_slot_lookup_count: usize,
    page_slot_scan_count: usize,
    surface_lookup_count: usize,
) -> WorthUiCompositionRootMountReceipt {
    let query_graph_execution = runtime
        .graph_authority()
        .plan_composition_topology_graph_operation(root.root_id().as_str(), facts.clone())
        .into_execution_receipt();
    WorthUiCompositionRootMountReceipt::new(
        root.clone(),
        authority,
        facts,
        query_graph_execution,
        page_slot_lookup_count,
        page_slot_scan_count,
        surface_lookup_count,
    )
}

fn surface_root_facts(
    root: &WorthUiCompositionRootReceipt,
    surface_id: &SurfaceId,
) -> Vec<WorthUiRuntimeFactId> {
    vec![
        root.fact_id().clone(),
        WorthUiRuntimeFactId::surface_mount(surface_id),
        WorthUiRuntimeFactId::authored_mount_component_selection(surface_id.as_str()),
        WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
    ]
}

fn report_denial(denial: WorthUiCompositionRootMountDenial) -> WorthUiCompositionRootMountReport {
    WorthUiCompositionRootMountReport::denied(vec![denial])
}

fn missing_root_authority_report(
    code: WorthUiCompositionRootMountDenialCode,
    root: &WorthUiCompositionRootReceipt,
) -> WorthUiCompositionRootMountReport {
    report_denial(WorthUiCompositionRootMountDenial::new(
        code,
        root,
        root.authority_identity(),
        vec![root.fact_id().clone()],
    ))
}

fn missing_authority_code(
    kind: WorthUiCompositionRootKind,
) -> WorthUiCompositionRootMountDenialCode {
    match kind {
        WorthUiCompositionRootKind::ComponentInstance => {
            WorthUiCompositionRootMountDenialCode::MissingComponentInstance
        }
        WorthUiCompositionRootKind::PortalEntry => {
            WorthUiCompositionRootMountDenialCode::MissingPortalEntry
        }
        WorthUiCompositionRootKind::CollectionItem => {
            WorthUiCompositionRootMountDenialCode::MissingCollectionItem
        }
        WorthUiCompositionRootKind::DiagnosticPanel => {
            WorthUiCompositionRootMountDenialCode::MissingDiagnosticPanel
        }
        WorthUiCompositionRootKind::Surface => {
            WorthUiCompositionRootMountDenialCode::MissingSurface
        }
        WorthUiCompositionRootKind::PageContentSlot => {
            WorthUiCompositionRootMountDenialCode::MissingPageSlot
        }
    }
}

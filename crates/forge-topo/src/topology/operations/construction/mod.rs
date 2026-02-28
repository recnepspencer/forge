//! Feature-level modeling operators (composite).
//!
//! DOMAIN: Extrude, revolve, sweep, loft/skin, offset/shell/thicken,
//! fillet/chamfer/blend, and draft/deform operations.
//!
//! OPERATORS (from operators-list.md §L):
//! - L1: Extrude (ExtrudeFace, ExtrudeLoop, ExtrudeWire, ExtrudeWithCaps, etc.)
//! - L2: Revolve (RevolveFace, RevolveLoop, RevolveWithCaps, etc.)
//! - L3: Sweep (SweepFaceAlongPath, SweepProfileAlongSpine, SweepWithGuides, etc.)
//! - L4: Loft/Skin (LoftBetweenProfiles, SkinSurfaceFromCurves, FillHoleWithSurface, etc.)
//! - L5: Offset/Shell/Thicken (OffsetFace/Shell/Sheet, ShellBody, ThickenSheet, etc.)
//! - L6: Fillet/Chamfer/Blend (FilletConstantRadius, ChamferDistance, BlendFaces, etc.)
//! - L7: Draft/Deform (DraftFacesNeutralPlane, TaperExtrude, BendSheet, etc.)
//!
//! DEPENDENCIES: `euler`, `algorithms`, `arena`, `handles`, `forge-geom`

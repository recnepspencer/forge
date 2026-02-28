
A. Classic Euler Operators (2-Manifold Solids)

MVFS

KVFS

MEV

KEV

MVE

KVE

MEF

KEF

MEKR

KEMR

MFKRH

KFMRH

MFV

KFV

KEMR

MEKR

KEV

KVE

KEF

KFV

B. Core Topological Entity Lifecycle (Bodies, Components, Lumps, Shells)
B1. Body / Model

CreateBody

DestroyBody

CloneBody

DetachBody

AttachBody

SplitBody

MergeBodies

B2. Components

CreateComponent

DestroyComponent

SplitComponent

MergeComponents

ExtractComponent

ReattachComponent

B3. Lumps

CreateLump

DestroyLump

SplitLump

MergeLumps

ExtractLump

InsertLump

RehomeLump

B4. Shells

CreateShell

DestroyShell

SplitShell

MergeShells

ExtractShell

InsertShell

PromoteShell

DemoteShell

RehomeShell

C. Face / Loop / Edge / Vertex Lifecycle
C1. Vertices

CreateVertex

DestroyVertex

CloneVertex

SplitVertex

MergeVertices

DetachVertex

AttachVertex

C2. Edges

CreateEdge

DestroyEdge

CloneEdge

SplitEdge

MergeEdges

DetachEdge

AttachEdge

ReverseEdge

C3. Faces

CreateFace

DestroyFace

CloneFace

SplitFace

MergeFaces

DetachFace

AttachFace

ReverseFace

C4. Loops

CreateLoop

DestroyLoop

CloneLoop

SplitLoop

MergeLoops

DetachLoop

AttachLoop

ReverseLoop

D. Boundary Editing Primitives (Loop Wiring)

InsertEdgeIntoLoop

RemoveEdgeFromLoop

InsertVertexIntoEdge

RemoveVertexFromEdge

SpliceLoopAtVertex

UnspliceLoopAtVertex

SpliceLoopAtEdge

UnspliceLoopAtEdge

ReplaceLoopEdgeChain

ReplaceLoopVertex

RerouteLoopAcrossFace

SwapLoopOrderOnFace

PromoteInnerLoop

DemoteOuterLoop

SetLoopContainment

RecomputeLoopContainment

E. Generalized Non-Manifold Topology (Radial-Edge / Uses)
E1. Use Entities

CreateEdgeUse

DestroyEdgeUse

CreateCoedge

DestroyCoedge

CreateVertexUse

DestroyVertexUse

CreateLoopUse

DestroyLoopUse

CreateFaceUse

DestroyFaceUse

RehomeCoedge

RehomeEdgeUse

RehomeVertexUse

E2. Radial Cycles

InsertRadialUse

RemoveRadialUse

SpliceRadialCycle

UnspliceRadialCycle

RotateRadialCycle

CanonicalizeRadialCycle

SwapRadialNeighbors

SplitRadialEdge

MergeRadialEdges

DetachUseFromEdge

AttachUseToEdge

ReorderRadialUses

E3. Non-Manifold Vertex Disks

CreateVertexDisk

DestroyVertexDisk

SplitVertexDisk

MergeVertexDisks

DetachDiskFromVertex

AttachDiskToVertex

ReassignFaceUmbrella

CanonicalizeDiskOrder

E4. Non-Manifold Sewing / Gluing

GlueFaces

UnglueFaces

GlueEdges

UnglueEdges

GlueVertices

UnglueVertices

PinchEdge

UnpinchEdge

PinchVertex

UnpinchVertex

F. Regions / Cellular Topology (Volumes)
F1. Regions

CreateRegion

DestroyRegion

SplitRegion

MergeRegions

ExtractRegion

InsertRegion

RehomeRegion

F2. Region Boundaries

BindShellToRegion

UnbindShellFromRegion

AddRegionBoundaryFace

RemoveRegionBoundaryFace

SwapRegionAdjacencyAcrossFace

FlipRegionSenseAcrossFace

F3. Internal Walls

InsertInternalWall

RemoveInternalWall

SplitRegionWithWall

MergeRegionsAcrossWall

ExtractWallAsSheet

ConvertWallToBoundary

F4. Outside Region

CreateOutsideRegion

BindOutsideRegion

UnbindOutsideRegion

ReclassifyOutsideAdjacency

G. Sheets / Wires / Laminar Topology
G1. Wire Bodies

CreateWireBody

DestroyWireBody

AddWireEdge

RemoveWireEdge

SplitWireEdge

MergeWireEdges

AttachWireToVertex

DetachWireFromVertex

G2. Sheet Bodies

CreateSheetBody

DestroySheetBody

AddSheetFace

RemoveSheetFace

SplitSheetFace

MergeSheetFaces

ConvertSheetToSolid

ConvertSolidToSheet

G3. Laminar Edges / Open Boundaries

MarkEdgeLaminar

UnmarkEdgeLaminar

CreateBoundaryLoop

DestroyBoundaryLoop

SplitLaminarEdge

MergeLaminarEdges

SewLaminarEdges

UnsewLaminarEdges

PromoteLaminarToManifold

DemoteManifoldToLaminar

H. Parametric B-Rep Coupling (Topo↔Geom Bindings)
H1. Face↔Surface

AttachSurfaceToFace

DetachSurfaceFromFace

ReplaceSurfaceOnFace

CopySurfaceToFace

SwapSurfaceParameterization

NormalizeSurfaceParameterization

SetFaceSurfaceSense

FlipFaceSurfaceSense

H2. Edge↔3D Curve

AttachCurveToEdge

DetachCurveFromEdge

ReplaceCurveOnEdge

CopyCurveToEdge

ReverseEdgeCurveSense

ReparameterizeEdgeCurve

NormalizeEdgeCurveDomain

SetEdgeCurveSense

H3. Coedge↔PCurve (Trim)

AttachPCurveToCoedge

DetachPCurveFromCoedge

ReplacePCurveOnCoedge

CopyPCurveToCoedge

ReversePCurveSense

ReparameterizePCurve

NormalizePCurveDomain

SetCoedgeSense

H4. Coupled Split / Merge (Atomic)

SplitEdgeAndCurves

MergeEdgesAndCurves

SplitCoedgeAndPCurve

MergeCoedgesAndPCurve

SplitFaceAndTrimNetwork

MergeFacesAndTrimNetwork

SplitVertexAndUses

MergeVerticesAndUses

H5. Trim Network Editing

InsertTrimLoop

RemoveTrimLoop

SplitTrimLoop

MergeTrimLoops

InsertTrimSegment

RemoveTrimSegment

StitchTrimEndpoints

UnstitchTrimEndpoints

SnapTrimEndpoints

RebuildTrimsFrom3DCurves

Rebuild3DCurvesFromTrims

H6. Seams / Periodicity

CreateSeamEdge

RemoveSeamEdge

MoveSeamEdge

SplitSeamEdge

MergeSeamEdges

CreatePoleSingularity

RemovePoleSingularity

I. Degeneracy / Collapse / Singularity Operators
I1. Edge / Vertex

CollapseEdgeToVertex

ExpandVertexToEdge

RemoveZeroLengthEdge

RemoveDanglingEdge

MergeCoincidentVerticesTopological

SplitMergedVertex

ReplaceVertexWithEdgeChain

I2. Face

CollapseFaceToEdge

CollapseFaceToVertex

RemoveZeroAreaFace

RemoveSliverFace

RemoveNeedleFace

MergeCoplanarFacesTopological

I3. Loop

RemoveDegenerateLoop

CollapseLoopToEdge

CollapseLoopToVertex

RemoveTinyHoleLoop

MergeNestedLoops

I4. Singularities

IntroduceConeTip

RemoveConeTip

IntroduceSpherePole

RemoveSpherePole

ConvertDegenerateEdgeToSingularity

ConvertSingularityToDegenerateEdge

J. Boolean / Imprint / Intersection Surgery (Composite)
J1. Intersection Construction

IntersectFaceFace

IntersectEdgeFace

IntersectEdgeEdge

IntersectCurveSurface

IntersectSurfaceSurface

BuildIntersectionCurves

BuildIntersectionVertices

BuildIntersectionGraph

CleanIntersectionGraph

ClassifyIntersectionGraph

J2. Imprinting

ImprintVertexOnEdge

ImprintVertexOnFace

ImprintEdgeOnFace

ImprintCurveOnFace

ImprintCurveNetworkOnFace

ImprintFaceOnFace

ImprintCoplanarOverlap

ImprintCoincidentFaces

ImprintSeamAware

ImprintToleranceAware

J3. Splitting Along Imprints

SplitIntersectedEdges

SplitIntersectedCoedges

SplitFacesByImprintLoops

SplitShellsByImprint

SplitRegionsByImprint

J4. Classification / Selection

ClassifyFacesKeepDiscard

ClassifyRegionsKeepDiscard

MarkKeepFaces

MarkDiscardFaces

SelectResultShells

SelectResultRegions

J5. Deletion / Extraction

DeleteDiscardTopology

ExtractKeptShells

ExtractKeptRegions

RemoveRedundantImprintEdges

RemoveRedundantImprintLoops

J6. Stitch / Merge / Resolve

StitchOpenBoundaries

SewLaminarBoundaries

MergeCoincidentEdges

MergeCoincidentCoedges

MergeCoincidentVertices

ResolveRadialConflicts

CanonicalizeGlobalRadials

CleanupDanglingTopology

J7. Finalization

PromoteToManifoldIfPossible

NormalizeForExport

ValidateAndRepairBooleanResult

K. Sewing / Healing / Repair (Composite)
K1. Sewing

SewEdges

SewCoedges

SewSheets

UnsewEdges

UnsewSheets

SewWithTolerance

SewByTopologyOnly

K2. Gap / Crack / Hole Healing

HealVertexGaps

HealEdgeGaps

HealMicroHoles

PatchHoleWithFace

RemovePatchAndReheal

K3. Remove-and-Heal

RemoveFaceAndHeal

RemoveEdgeAndHeal

RemoveVertexAndHeal

ExtendNeighborSurfaces

RebuildBoundaryAtIntersection

K4. Artifact Cleanup

DetectSlivers

RemoveSlivers

CollapseShortEdgesUnderTolerance

RemoveTinyLoopsUnderTolerance

SimplifyEdgeChains

SimplifyTrimChains

K5. Refit / Rebuild

RefitSurfaceToBoundary

RefitCurveToEndpoints

RebuildTrimsForConsistency

RebuildSeamsOnPeriodicFaces

RebuildPCurvesFromSurfaces

L. Construction / Feature-Level Modeling (Composite)
L1. Extrude

ExtrudeFace

ExtrudeLoop

ExtrudeWire

ExtrudeWithCaps

ExtrudeToNext

ExtrudeThroughAll

L2. Revolve

RevolveFace

RevolveLoop

RevolveWithCaps

RevolveToAngle

RevolveToFace

L3. Sweep

SweepFaceAlongPath

SweepProfileAlongSpine

SweepWithGuides

SweepWithTwist

SweepWithScaleLaw

SweepToSurface

L4. Loft / Skin

LoftBetweenProfiles

LoftWithGuides

LoftWithContinuity

LoftToPoint

LoftToCurve

SkinSurfaceFromCurves

BoundarySurfaceFromCurves

FillHoleWithSurface

L5. Offset / Shell / Thicken

OffsetFace

OffsetShell

OffsetSheet

OffsetWithSelfIntersectionResolution

ShellBody

ShellWithRemovedFaces

ShellVariableThickness

ThickenSheet

ThickenWithSideWalls

L6. Fillet / Chamfer / Blend

FilletConstantRadius

FilletVariableRadius

FilletEdgeChain

FilletWithSetbacks

FilletWithHoldLines

FilletPropagate

RemoveFilletAndHeal

ChamferDistance

ChamferAngle

ChamferTwoDistance

ChamferChain

RemoveChamferAndHeal

BlendFaces

BlendEdgeNetwork

G1BlendPatch

G2BlendPatch

L7. Draft / Deform

DraftFacesNeutralPlane

DraftFacesAboutEdge

TaperExtrude

BendSheet

WarpSurfaceWithConstraints

M. Global Editing / Topology Operations (Composite)

SliceSolidWithPlane

SliceSolidWithSurface

SectionWithPlane

SectionWithSurface

CutWithSheet

CutWithWire

UniteBodies

SeparateBodies

DisconnectAtFaces

DisconnectAtEdges

DisconnectAtVertices

ExtractConnectedComponent

MergeCoplanarFaces

MergeTangentFaces

RemoveRedundantEdges

SimplifyTopology

NormalizeLoopContainment

RebuildFaceLoopsFromUses

N. Transform / Copy / Pattern (Structural + Composite)

CopyBody

CopyLump

CopyShell

CopyFace

CopyEdge

CopyVertex

TransformBody

TransformShell

TransformFaceGeometryOnly

TransformTopologyOnly

MirrorBody

PatternLinear

PatternCircular

PatternByPoints

InstanceBody

DeinstanceBody

O. Validation / Consistency / Normalization

ValidateManifoldness

ValidateRadialCycles

ValidateVertexDisks

ValidateLoopClosure

ValidateFaceOrientation

ValidateShellClosure

ValidateRegionAdjacency

ValidateUseConsistency

ValidateCurveBindings

ValidateSurfaceBindings

ValidateTrimConsistency

ValidateSeamConsistency

ValidateToleranceContracts

NormalizeEntityOrdering

CanonicalizeTraversalOrder

RebuildAdjacencyCaches

RebuildSpatialCaches

P. Topological Query / Navigation (Non-Mutating)
P1. Adjacency

GetVertexEdges

GetEdgeVertices

GetEdgeFaces

GetFaceEdges

GetFaceLoops

GetLoopCoedges

GetCoedgeEdge

GetUseRadialNeighbors

GetShellFaces

GetLumpShells

GetBodyLumps

P2. Incidence / Membership

IsVertexOnEdge

IsEdgeOnFace

IsLoopOnFace

IsFaceOnShell

IsShellOnLump

IsEdgeLaminar

IsEdgeManifold

IsEdgeNonManifold

IsFaceSheet

IsFaceSolidBoundary

P3. Orientation / Sense

GetCoedgeSense

GetFaceSense

GetLoopSense

OrientShellOutward

OrientFaceConsistently

ComputeEdgeOrientationOnFace

P4. Containment / Classification

PointInFace

PointInLoop

LoopContainsLoop

FaceContainsPoint

ShellContainsPoint

RegionContainsPoint

ClassifyFaceSideOfPlane

ClassifyPointWinding

DetectSelfIntersectionTopology

P5. Connectivity

FindConnectedComponents

FindEdgeChain

FindBoundaryChains

FindLoopIslands

FindNonManifoldJunctions

FindDanglingTopology

FindDuplicateEntities

Q. Geometry Library Operators (Curves / Surfaces)
Q1. Evaluation

EvalCurvePoint

EvalCurveDerivatives

EvalSurfacePoint

EvalSurfaceDerivatives

EvalNormal

EvalCurvature

Q2. Closest Point / Projection

ClosestPointOnCurve

ClosestPointOnSurface

ProjectPointToCurve

ProjectPointToSurface

InvertSurfacePointToUV

InvertCurvePointToT

Q3. Intersection

IntersectCurves

IntersectCurveSurface

IntersectSurfaces

IntersectSurfacePlane

IntersectCurvePlane

IntersectAnalyticAnalytic

IntersectAnalyticNURBS

IntersectNURBSNURBS

Q4. Construction

BuildLine

BuildCircle

BuildEllipse

BuildPlane

BuildCylinder

BuildCone

BuildSphere

BuildTorus

BuildNURBSCurve

BuildNURBSSurface

BuildBlendSurface

Q5. Modification

OffsetCurve

OffsetSurface

ExtendCurve

ExtendSurface

TrimCurve

TrimSurface

SplitCurve

SplitSurface

ReparameterizeCurve

ReparameterizeSurface

RefineKnotVector

ElevateDegree

ReduceDegree

RefineSurfaceKnots

RefitCurveToPoints

RefitSurfaceToPoints

Q6. Continuity / Matching

EnforceG0

EnforceG1

EnforceG2

MatchSurfaceBoundaries

MatchCurveEndpoints

BuildCornerSetbackPatch

R. Tolerance / Robustness / Numerical Policy Operators

SetModelTolerance

GetModelTolerance

SetEdgeTolerance

SetVertexTolerance

SetFaceTolerance

PropagateTolerances

NormalizeTolerances

TightenTolerances

RelaxTolerances

SnapPointToToleranceGrid

QuantizeCoordinates

CanonicalizeFloatingValues

RobustCompare

RobustOrient2D

RobustOrient3D

RobustInSphere

RobustSegmentIntersection

RobustPolygonWinding

AdaptivePrecisionFallback

ExactArithmeticFallback

SymbolicPerturbationFallback

S. Spatial Acceleration / Index Maintenance

BuildAABBTree

UpdateAABBTree

RefitAABBTree

RebuildAABBTree

BuildBVH

UpdateBVH

BuildRTree

UpdateRTree

BuildSpatialHash

UpdateSpatialHash

BuildFaceSamplingCache

BuildEdgeSamplingCache

BuildCurveBBoxCache

BuildSurfaceBBoxCache

QueryAABBOverlap

QueryRayCast

QueryClosestEntity

QueryWithinDistance

T. Derived Data / Caches / Projections

ComputeBoundingBox

ComputeFaceArea

ComputeEdgeLength

ComputeVertexValence

ComputeShellClosureStatus

ComputeRegionVolume

ComputeMassProperties

ComputeCentroid

ComputeInertiaTensor

BuildTopologySignature

HashTopology

HashGeometry

BuildStableExportOrder

BuildPersistentNamingMap

UpdatePersistentNamingMap

U. Meshing / Tessellation / Visualization Projections

TessellateFace

TessellateShell

TessellateBody

TessellateWithChordTolerance

TessellateWithAngleTolerance

TessellateAdaptive

GenerateRenderMesh

GenerateAnalysisMesh

RebuildMeshAfterEdit

BuildSDFPreview

UpdateSDFPreview

ExtractIsosurface

V. Import / Export / Translation Pipelines
V1. Import

ImportSTEP

ImportIGES

ImportParasolid

ImportSAT

ImportSTL

ImportOBJ

ImportBREP

DetectUnits

ConvertUnits

StitchImportedSheets

HealImportedTopology

InferMissingTrims

InferMissingCurves

ResolveImportedSeams

RebuildImportedRadials

V2. Export

ExportSTEP

ExportIGES

ExportParasolid

ExportSAT

ExportSTL

ExportOBJ

ExportBREP

ExportWithHealing

ExportWithNormalization

ExportWithPersistentNaming

W. Transactions / Journaling / Rollback

BeginTransaction

CommitTransaction

RollbackTransaction

SaveCheckpoint

RestoreCheckpoint

BeginSubtransaction

CommitSubtransaction

RollbackSubtransaction

RecordJournalEntry

ReplayJournal

CompressJournal

PruneJournal

MergeJournals

ValidateJournalDeterminism

X. Provenance / Lineage / Identity Management

AllocateEntityId

RetireEntityId

RemapEntityIds

ForkEntityLineage

MergeEntityLineage

RecordEntityParentage

RecordOpSignature

RecordDecisionLog

RecordPolicyOutcome

AttachDebugTrace

DetachDebugTrace

BuildMinimalReproSlice

Y. Diagnostics / Debugging / Introspection

DumpTopologyGraph

DumpRadialCycles

DumpVertexDisks

DumpLoopStructure

DumpTrimNetwork

DumpParamMappings

TraceOperation

TraceInvariants

TracePolicyDecisions

TraceNumericalFallbacks

ValidateAndExplainFailure

LocalizeFailureToOperator

GenerateDeltaReport

GenerateTopologyDiff

GenerateGeometryDiff

Z. Kernel Policy / Configuration / Capability Switching

SetBooleanStrategy

SetHealingLevel

SetSewingPolicy

SetImprintPolicy

SetDegeneracyPolicy

SetExactnessPolicy

SetDeterminismPolicy

SetCanonicalizationPolicy

SetPerformanceBudget

SetTimeoutPolicy

EnableNonManifoldIntermediates

DisableNonManifoldIntermediates

EnableSheetModeling

DisableSheetModeling

1) Coincidence and Partial-Overlap Surgery

ResolveEdgeEdgePartialOverlap

SplitEdgeAtOverlapInterval

MergeCollinearEdgeIntervals

ResolveFaceFaceCoplanarOverlapIslands

ExtractCoplanarOverlapLoops

NormalizeCoplanarOverlapWinding

ResolveCoincidentButOppositeSenseEdges

ResolveCoincidentEdgesDifferentParameterization

ReconcileAnalyticCoincidence (line/arc vs NURBS)

DetectAndSplitAtNearCoincidenceJunctions

RemoveMicroBridgeEdges

ConvertOverlapToSharedTopology (shared edge promotion)

ConvertSharedTopologyToSeparate (de-share / de-stitch)

2) T-Junctions and Non-Manifold “Almost-Manifold” Fixups

DetectTJunctions

PromoteTJunctionToVertexSplit (introduce explicit vertex)

InsertVertexOnEdgeForTJunction

ImprintTJunctionAcrossFaces

ResolveTJunctionInTrimSpace

ResolveHangingCoedge

ResolveDanglingUseInRadialCycle

RepairRadialCycleBrokenLink

RepairVertexDiskBrokenUmbrella

ReassignFacesBetweenVertexDisks

3) Near-Tangent and Ambiguous Classification Repairs

ClassifyWithTangentBiasPolicy

ReclassifyNearTangentIntersectionSegments

SplitIntersectionAtTangentEvent

RemoveSpuriousTangentSpikes

CollapseNearZeroAngleWedges

NormalizeIntersectionGraphAtTangencies

EnforceConsistentInsideOutsideAcrossTangentRuns

ResolveNearCoincidentNormalsConflict

4) Seams, Periodic Surfaces, and Poles (the “quiet killers”)

UnwrapPCurveAcrossSeam

WrapPCurveToCanonicalPeriod

RelocateSeamAwayFromTrimLoops

RebuildTrimLoopsAcrossSeam

SplitTrimAtSeamCrossing

MergeTrimAcrossSeamCrossing

ResolveSeamDoubleCounting (duplicate coedges)

ResolvePoleNeighborhoodTopology

ConvertPoleSingularityToSeamPatch

ConvertSeamPatchToPoleSingularity

NormalizePeriodicParameterDomains (U/V)

RepairPeriodicSurfaceOrientationMismatch

5) Trim-Network Pathologies (UV ≠ XYZ)

DetectTrimSelfIntersection

SplitTrimLoopAtSelfIntersection

ResolveTrimLoopFigureEight

RemoveTrimMicroLoops

SnapTrimEndpointsWithArcLengthPolicy

RebuildPCurveFrom3DCurveWithTolerance

Rebuild3DCurveFromPCurveWithTolerance

ReconcileDualTrimsBetweenAdjacentFaces

FixTrimSenseMismatchAcrossSharedEdge

FixTrimParameterMonotonicity

ResolveTrimCornerCusps

RepairTrimTopologyAfterSurfaceRefit

6) Degeneracy Beyond “CollapseEdge”

CollapseShortEdgeWithNeighborRewire

CollapseVertexStarToSingleEdge

DeleteZeroAreaFaceWithBoundaryRebridge

RemoveNeedleFaceAndPropagateCollapse

RemoveSliverStripAndRestitch

MergeNearlyCoplanarFacesWithGuardRails

ConvertTinyLoopToVertexSingularity

ResolveTwoVertexLoop (edge doubled back)

ResolveOneEdgeLoop (periodic edge loop)

RepairPinchedFaceUmbrella

7) Boolean-Specific Rare Surgery

ResolveIntersectionGraphNonPlanarKnot

ResolveGraphWithDuplicateVerticesSameLocation

MergeIntersectionVerticesWithinTolerance

SplitIntersectionEdgesAtNearMissEvents

DeleteDanglingIntersectionSpurs

Re-routeIntersectionAcrossFaceBoundaries

ForceManifoldizationPass (best-effort)

DemoteToSheetResultIfNonManifoldPersistent

RepairAfterClassificationFlip (local reclass)

StabilizeCoplanarBooleanResults (planar special-case)

8) Sewing/Healing “Last Mile” Helpers

HealGapByRedistributingParameterization

HealGapByEdgeExtensionAndRetrim

HealGapByFaceExtrapolationPatch

ResolveSewingWithMismatchedEdgeSampling

ResolveSewingWithMultipleCandidateMatches

SelectBestSewPairByEnergyScore

RepairSewingCreatesInvertedLoop

RepairSewingCreatesNonSimpleLoop

RepairSewingCreatesMicroFace

PostSewCanonicalizeRadialsAndDisks

9) Import Pathology Operators (STEP/IGES Reality)

InferMissingPCurvesFrom3D

InferMissing3DCurvesFromPCurves

RepairBrokenEdgeTwinRelationships

RepairBrokenCoedgePartnerLinks

RebuildFaceLoopFromEdgeSoup

DetectAndFixNonClosedLoops

FixFaceOrientationInconsistentWithSurface

ResolveDuplicateEntitiesFromImporter

PromoteEdgeSoupToWireBody

PromoteSheetSoupToSheetBody

TolerantMergeImportVertices

TolerantMergeImportEdges

CleanupLevel_0_Minimal

CleanupLevel_1_Standard

CleanupLevel_2_Aggressive

10) Persistent Naming / Identity Preservation (rare, but life-saving)

BuildPersistentNamingSeeds

PropagatePersistentNamesThroughSplit

PropagatePersistentNamesThroughMerge

ResolveNameConflictsAfterBoolean

RemapNamesAfterHeal

RepairDanglingNameReferences

ExtractStableSubshapeSignatures

ReconcileIDsAfterTopologyNormalization

11) Reference / Integrity Repair (the “why did this pointer break” kit)

AuditAndRepairTwinPointers

AuditAndRepairNextPrevLinks

AuditAndRepairRadialLinks

AuditAndRepairUseOwnership

RepairOrphanedEntities

RepairDoubleOwnedEntities

RepairLoopNotOnFace

RepairCoedgeNotInLoop

RepairFaceNotInShell

RepairShellNotInLump

RepairRegionAdjacencyMismatches

MB-T (Topology / Invariants / Determinism / Scale)
MB-T1 / MB-C* (500-step chains + per-step invariants)

ValidateEulerGeneralizedPerStep

ValidateManifoldnessPerStep

ValidateRadialCyclesPerStep

ValidateVertexDisksPerStep

ValidateUseGraphIntegrity (twin/next/prev/radial ownership)

ValidateRegionAdjacencyPerStep

ComputeTopologyHashCanonical (order-independent)

CanonicalizeEntityOrderingDeterministic

CanonicalizeTraversalOrderDeterministic

BeginTransaction / CommitTransaction / RollbackTransaction

SaveCheckpoint / RestoreCheckpoint

ReplayJournalDeterministic

CompressJournalDeterministic (stable compression)

MB-T2 (near-degenerate face injection, “binary line” validation)

ComputeFaceAreaRobust (scaled / exact / interval)

ClassifyFaceAsDegenerateByPolicy (thresholding is a policy op)

RemoveZeroAreaFaceWithRebridge

RemoveSliverFaceAndRestitch

CollapseFaceToEdge

CollapseFaceToVertex

CollapseShortEdgesUnderTolerance (topo + geom coupled)

RemoveDegenerateLoop

RemoveNeedleFace

RepairAfterDegenerateDeletion (rewire loops/uses)

PolicyRequired_DegenerateResolution (structured)

MB-T3 (Genus-10 solid, generalized Euler correct)

ValidateEulerWithGenus (tracks components/shells/regions/genus)

ComputeGenusFromTopology

ValidateShellClosurePerShell

ValidateRegionCountAndAdjacency

ValidateLoopCountsPerFace

MB-T4 (multi-shell + 20 voids, per-shell validation)

ValidatePerShellOrientationAndClosure

ValidateInnerOuterShellClassification

BindShellToRegion / UnbindShellFromRegion

PromoteShell / DemoteShell (outer↔inner)

ValidateRegionBoundaryConsistency

MB-T5 (orientation chaos import → deterministic canonical orientation)

OrientShellOutwardDeterministic

OrientFaceConsistentlyDeterministic

FixFaceOrientationInconsistentWithSurface

RecomputeLoopContainmentDeterministic

NormalizeLoopContainmentDeterministic

CanonicalizeRadialCycle

CanonicalizeVertexDiskOrder

RepairImportedTopologyOrientation (batch)

CleanupLevel_0/1/2 (deterministic tiers)

MB-T6 (near-non-manifold stress, caught or clean)

DetectNearNonManifoldConfiguration

PredictManifoldnessAfterSurgery (preflight)

EnforceNonManifoldPolicyGate (allow/deny intermediate)

InsertRadialUse / RemoveRadialUse

ResolveRadialConflicts

RepairRadialCycleBrokenLink

SeparateVertexDisks / MergeVertexDisks

PolicyRequired_NonManifoldDecision (structured)

MB-T7 (1e12 extent + 1e-9 feature, no overflow in area/volume)

ComputeAreaScaleSafe (local frame / scaling)

ComputeVolumeScaleSafe (local frame / scaling)

LocalCoordinateFramePush

LocalCoordinateFramePop

NormalizeCoordinatesForOperation (conditioning)

DeconditionResultCoordinates

AdaptivePrecisionFallback (for measurement ops too)

MB-D (Dual-Path Cross-Check)
MB-D1 / MB-D2 (planar + curved dual path agree or structured disagreement)

DualPathBooleanExecute (primary + reference path)

DualPathCompareTopology

DualPathCompareFaceCentroids

DualPathCompareClassificationLabels

DualPathCompareIntersectionGraphs

BuildStructuredDisagreementReport

LocalizeDisagreementToDecision (decision log slice)

PolicyRequired_DualPathDisagreement (structured)

MB-D3 (near-tangent cylinder union, ambiguity caught)

DetectNearTangentContact

SplitIntersectionAtTangentEvent

ClassifyWithTangentBiasPolicy

ReclassifyNearTangentSegments

PolicyRequired_TangentAmbiguity (structured)

MB-D4 (coplanar face lies on both boundaries, no false positives)

DetectCoplanarContact

ExtractCoplanarOverlapLoops

ResolveFaceFaceCoplanarOverlapIslands

NormalizeCoplanarOverlapWinding

CoplanarBooleanSpecialCaseDispatch

PolicyRequired_CoplanarIntent (structured)

MB-D5 (perf budget)

BuildAABBTree / RefitAABBTree (incremental)

IntersectionCandidateCullingFastPath

CacheIntersectionResultsAcrossChain (bounded)

EarlyExitNoOpDetection

BudgetEnforceBoolean (timeouts, staged fallback)

MB-D6 (scale extreme dual path across 18 orders)

ScaleConditionAndRunDualPath

CompareResultsInNormalizedSpace

PolicyRequired_ScaleConditioningFailure

MB-N (Numerics / Predicates / Precision Pipeline)
MB-N1 (orient3d divergences caught + classified)

Orient3D_Float

Orient3D_Interval

Orient3D_RationalExact

ComparePredicateResultsAndClassifyDivergence

RecordPredicateFallbackEvent

PolicyRequired_PredicateAmbiguity

MB-N2 (near-coincident faces boolean resolved)

DetectNearCoincidentFaces

ResolveCoincidentEdgesDifferentParameterization

ReconcileAnalyticCoincidence (analytic↔NURBS)

SplitEdgeAtOverlapInterval

ConvertOverlapToSharedTopology

PolicyRequired_CoincidenceResolution

MB-N3 (no accumulated float drift changes topo decisions)

CanonicalizeFloatingValues (quantize policy)

SnapPointToToleranceGrid (policy)

RobustCompareWithHysteresis (decision stability)

DecisionLogForEveryTopologicalChoice

DeterministicTieBreakersEverywhere

MB-N4 (scale sweep invariant topology)

NormalizeScaleForOperation

DeNormalizeScaleAfterOperation

TopologyHashCanonical (scale-invariant comparison)

PolicyRequired_ScaleVarianceDetected

MB-N5 (condition number stress, nearly parallel planes)

DetectIllConditionedIntersection

IntersectWithAdaptivePrecision

IntersectWithExactFallback

PolicyRequired_IllConditionedIntersection

MB-N6 (bit-growth budget bounded)

RationalBitBudgetEnforcer

RationalReduceAndNormalize

SwitchToIntervalWhenSafe

PolicyRequired_BitBudgetExceeded

MB-C (Chain Corruption / Determinism / Selectors)
MB-C1..C7 (chains + determinism)

All MB-T1 items +

GenerateDeltaReportPerStep

ValidateAndExplainFailurePerStep

BuildMinimalReproSlice (for any divergence)

MB-C8 (selectors referencing previous features)

PersistentNamingSeed

PropagatePersistentNamesThroughSplit

PropagatePersistentNamesThroughMerge

ResolveNameConflictsAfterBoolean

SelectorResolveDeterministic

PolicyRequired_SelectorAmbiguity

MB-F (Fillets)
MB-F1 / MB-F5 (multi-edge junction corner patches)

BuildFilletCornerPatch (n-way)

ResolveFilletJunctionTopology

ResolveSeamAndTrimAtJunction

EnforceG1AcrossPatchSeams

PatchSelectionDeterministic

MB-F2 (variable radius smooth transition)

BuildVariableRadiusSpineLaw

RefitFilletSurfaceWithContinuity

DetectFilletSelfIntersection

ResolveFilletSelfIntersection (trim/retopologize)

MB-F3 (radius exceeds face width → PolicyRequired with candidates)

DetectOverconsumedSupportFaces

EnumerateReconstructionStrategies

PredictManifoldnessForStrategy

PolicyRequired_FilletOverrun (consumed IDs + candidates)

MB-F4 (micro-fillet at macro scale)

LocalCoordinateFramePush (micro-region)

NormalizeCoordinatesForFillet

ComputeFilletInLocalSpace

DeconditionFilletResult

ScaleInvariantPrecisionPolicy

MB-F6 (near-tangent edges)

DetectNearTangentEdges

ClassifyWithTangentBiasPolicy

PolicyRequired_FilletNearTangency

MB-F7 (fillet chain validation)

ValidateLayer1AfterEachFilletStep

RepairAfterCollapseEvents (fillet degeneracy)

MB-F8 (fillet surfaces participate in Boolean, dual-path agrees)

RebuildTrimsAfterFillet

ReconcileDualTrimsBetweenAdjacentFaces

DualPathCompareIntersectionGraphs (curved)

What’s still “missing” that helps even more

If you want the last 2% to be less miserable, add these operator packs explicitly (they matter directly to your MB list):

Integrity repair kit (for chains):
AuditAndRepairTwinPointers, AuditAndRepairNextPrevLinks, AuditAndRepairRadialLinks, RepairOrphanedEntities, RepairDoubleOwnedEntities

Coplanar / overlap interval surgery (for MB-D4 / MB-N2):
SplitEdgeAtOverlapInterval, MergeCollinearEdgeIntervals, ExtractCoplanarOverlapLoops, ConvertOverlapToSharedTopology, DeShareSharedTopology

Tangent event normalization (for MB-D3 / MB-N5 / MB-F6):
SplitIntersectionAtTangentEvent, RemoveSpuriousTangentSpikes, NormalizeIntersectionGraphAtTangencies

Local conditioning transforms as first-class ops (for MB-T7 / MB-D6 / MB-F4):
LocalCoordinateFramePush/Pop, NormalizeCoordinatesForOperation, DeconditionResultCoordinates, ComputeAreaScaleSafe/ComputeVolumeScaleSafe
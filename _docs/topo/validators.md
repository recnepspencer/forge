) Pointer and reference integrity (the #1 common set)

ValidateTwinSymmetry (edge/coedge partner links are mutual)

ValidateNextPrevSymmetry (halfedge/coedge ring is consistent)

ValidateCycleClosure (walk next until you return; no early nulls)

ValidateNoDanglingHandles (every referenced ID exists)

ValidateOwnership (every coedge belongs to exactly one loop; loop to one face; face to one shell, etc.)

ValidateNoOrphans (every entity reachable from body roots)

2) Loop and boundary sanity (cheap and constant)

ValidateLoopClosure

ValidateLoopHasMinimumCardinality (no 1-edge loops unless explicitly allowed)

ValidateNoDuplicateCoedgesInLoop (unless singularity rules allow)

ValidateEdgeEndpointsMatchLoopVertices

ValidateInnerOuterLoopFlagsConsistent

3) Face-level sanity (the most common “surface topology” checks)

ValidateFaceHasAtLeastOneLoop

ValidateLoopOrientationConsistentWithFaceSense

ValidateFaceAdjacencyConsistency (neighbor faces across shared edges agree)

ValidateNoFaceWithBrokenBoundary (boundary chain missing links)

4) Shell/body closure + orientation (common on solid ops, imports, booleans)

ValidateShellWatertightness (solids: every boundary edge has proper adjacency)

ValidateBoundaryEdgesAreLaminarOnly (sheets/wires)

ValidateConsistentShellOrientation (outer vs inner shell sense)

ValidateNoInsideOutShells (quick region/sense sanity)

5) Manifold / NMT checks (common if you allow NMT intermediates)

ValidateEdgeManifoldStateMatchesUseCount (laminar/manifold/non-manifold)

ValidateRadialCycleClosure (for edges with >2 uses)

ValidateRadialCycleUniqueness (no duplicate uses in the cycle)

ValidateVertexDiskPartition (if you model vertex disks)

6) Degeneracy classification (common when you do fillets/booleans/healing)

ValidateDegenerateFlagsConsistent (zero-length edges, zero-area faces)

ValidateNoUnexpectedZeroLengthEdges

ValidateNoUnexpectedZeroAreaFaces

ValidateShortEdgePolicyApplied (below threshold edges handled consistently)

7) Minimal parametric binding checks (common once NURBS enters)

ValidateCurveBoundToEdge (3D curve exists where required)

ValidatePCurveBoundToCoedge (p-curve exists where required)

ValidateTrimLoopClosureInUV (basic closure, not full self-intersection)

ValidateSenseConsistency (coedge sense ↔ p-curve sense)

8) Cache/index staleness (common for correctness + performance)

ValidateAdjacencyCacheMatchesGroundTruth (or at least staleness flags)

ValidateAABB/BVHContainsGeometry (bounds contain evaluated samples)

ValidateCacheStalenessFlagsAfterMutation (dirty propagation works)

9) Determinism guards (common if you care about replay)

ValidateCanonicalOrderingStable (entity ordering / traversal order)

ValidateHashStability (topology hash invariant to iteration order)

ValidateTieBreakerCoverage (no “hashmap iteration decides” paths)


1) Reference integrity and ownership

ValidateNoDanglingHandles

ValidateGenerationalIdMatchesStorage

ValidateSingleOwnerPerEntity (use/loop/face ownership)

ValidateNoDoubleOwnedEntities

ValidateNoOrphanEntities (unreachable from body roots)

ValidateBidirectionalLinks (every backref exists and matches)

ValidateAcyclicContainmentGraph (body→lump→shell→face→loop→coedge)

2) Half-edge / loop wiring invariants

ValidateTwinSymmetry

ValidateNextPrevSymmetry

ValidateLoopClosure

ValidateLoopIsSimpleTopologically (no repeated coedge/vertex unless singularity)

ValidateEdgeEndpointsMatchCoedgeVertices

ValidateConsistentEdgeSenseAcrossCoedges

ValidateFaceLoopMembershipComplete (every coedge belongs to exactly one loop)

3) Radial-edge invariants (NMT core)

ValidateRadialCycleClosure

ValidateRadialCycleUniqueness (no duplicate uses in a cycle)

ValidateRadialNeighborConsistency

ValidateRadialOrderingDeterminism (canonical tie-breakers)

ValidateEdgeUseCountMatchesEdgeState (laminar/manifold/non-manifold)

ValidateNoBrokenRadialSplices

4) Vertex-disk / umbrella invariants

ValidateVertexDiskPartition (faces grouped into correct disks)

ValidateDiskClosure (disk boundary wiring)

ValidateDiskOrderingDeterminism

ValidateNoCrossDiskCoedges

ValidatePinchPointConsistency (allowed NMT states only)

5) Shell / body closure and orientation

ValidateShellWatertightness (solid shells)

ValidateBoundaryIsLaminarOnly (sheet bodies)

ValidateConsistentShellOrientation (outward for outer, inward for inner)

ValidateInnerShellContainment (void shells are inside outer shell)

ValidateNoInsideOutShells (orientation + region adjacency agree)

ValidateNoSelfIntersectingShellTopology (topo-level)

6) Region / cellular topology invariants

ValidateRegionAdjacencyGraph (faces separate exactly two regions when applicable)

ValidateOutsideRegionConnectivity

ValidateNoRegionLeaks (closed shells bound finite regions)

ValidateRegionBoundaryCompleteness

ValidateInternalWallConsistency (if enabled)

ValidateRegionCountAgainstShellConfig (sanity)

7) Euler / genus / generalized characteristics

ValidateEulerClassic (V−E+F−L etc. for chosen schema)

ValidateEulerGeneralizedWithRegions

ValidateGenusComputationConsistency (independent derivations agree)

ValidatePerComponentEuler (each connected component)

8) Degeneracy classification and “binary line”

ValidateDegeneracyPolicyConsistency (same input → same classification)

ValidateAreaVolumeComputationRobust (no overflow/underflow paths)

ValidateNoZeroLengthEdgesUnlessMarkedDegenerate

ValidateNoZeroAreaFacesUnlessMarkedDegenerate

ValidateSingularityEncodingConsistency (poles/tips/seams)

9) Parametric (NURBS) binding invariants

ValidateEveryCoedgeHasPCurveWhenRequired

ValidatePCurveSenseMatchesCoedgeSense

ValidateTrimLoopClosureInUV

ValidateTrimLoopIsSimpleInUV (no self-intersections unless allowed)

ValidateUVInDomainWithPeriodHandling

ValidateSeamCrossingAccounting (no double-counting on periodic faces)

ValidateConsistentSharedEdgeDualTrims (face A/B trims are compatible)

ValidateCurveSurfaceInversionResiduals (xyz↔uv within tolerance)

10) Intersection / imprint graph invariants (boolean prep)

ValidateIntersectionGraphConnectivity

ValidateNoDanglingIntersectionSpurs (unless classified and allowed)

ValidateConsistentVertexMergesInGraph (near-coincident collapse rules)

ValidateTangentEventEncoding (tangent segments flagged consistently)

ValidateCoplanarOverlapLoopExtraction (winding + containment consistent)

11) Numerical / predicate pipeline validators

ValidatePredicateDivergenceClassification (float vs interval vs exact)

ValidateIntervalBoundsSoundness

ValidateFallbackEscalationPolicy (when to go float→interval→exact)

ValidateConditionNumberTriggers (nearly-parallel, near-tangent)

ValidateBitBudgetAccounting (exact rationals bounded as required)

12) Determinism validators (the chain killers)

ValidateCanonicalOrderingStable (entities, loops, radials, disks)

ValidateHashStabilityAcrossRuns

ValidateJournalReplayExactness

ValidateTieBreakerCoverage (no “iteration-order decides” paths)

ValidateStableFloatingNormalization (quantize/hysteresis rules)

13) Cache / index consistency validators

ValidateAABBTreeCoversAllEntities

ValidateBVHRefitCorrectness (bounds contain geometry)

ValidateSpatialCacheStalenessFlags

ValidateAdjacencyCacheMatchesGroundTruth

ValidateCurvatureCacheStaleness

ValidateMassPropsCacheStaleness

14) Persistent naming / selector validators

ValidatePersistentNameUniqueness

ValidateNameSurvivalThroughSplitMerge (expected mapping exists)

ValidateSelectorResolutionDeterminism

ValidateNoDanglingNameReferences

15) Import sanity / “soup recovery” validators

ValidateImporterWiring (twins, loops, senses)

ValidateNoDuplicateCoincidentEntities (per tolerance policy)

ValidateMissingTrimInferenceCompleteness

ValidateSeamRebuildConsistency

ValidateCleanupLevelDeterminism (same input → same output)
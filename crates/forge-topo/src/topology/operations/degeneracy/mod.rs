//! Degeneracy, collapse, and singularity operators.
//!
//! DOMAIN: Collapsing edges/faces/loops to lower-dimensional entities,
//! removing zero-length/zero-area entities, merging coincident entities,
//! and managing singularities (cone tips, sphere poles).
//!
//! OPERATORS (from operators-list.md §I):
//! - I1: Edge/Vertex (CollapseEdgeToVertex, ExpandVertexToEdge, RemoveZeroLengthEdge, etc.)
//! - I2: Face (CollapseFaceToEdge/Vertex, RemoveZeroAreaFace, RemoveSliverFace, etc.)
//! - I3: Loop (RemoveDegenerateLoop, CollapseLoopToEdge/Vertex, RemoveTinyHoleLoop, etc.)
//! - I4: Singularities (IntroduceConeTip, IntroduceSpherePole, ConvertDegenerateEdgeToSingularity, etc.)
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`

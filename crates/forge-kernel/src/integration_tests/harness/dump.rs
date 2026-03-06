//! OBJ mesh export for failure diagnostics.
//!
//! DOMAIN: When a test fails, dumps the solid to a viewable OBJ file
//! so the developer can inspect the geometry visually. Writes to
//! `target/test-dumps/`.

use std::fmt::Write;
use std::path::PathBuf;

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::GeometryView;

/// Dump a `SolidEnvelope` to OBJ format for visual inspection.
///
/// Writes to `target/test-dumps/{name}.obj`. Creates the directory
/// if it doesn't exist. Returns the path to the written file.
pub fn dump_to_obj(env: &SolidEnvelope, name: &str) -> Result<PathBuf, std::io::Error> {
    let dir = PathBuf::from("target/test-dumps");
    std::fs::create_dir_all(&dir)?;

    let path = dir.join(format!("{name}.obj"));
    let content = envelope_to_obj(env);
    std::fs::write(&path, content)?;

    Ok(path)
}

/// Convert a `SolidEnvelope` to OBJ string.
fn envelope_to_obj(env: &SolidEnvelope) -> String {
    let arena = env.topology().arena();
    let geom = env.geometry();
    let mut obj = String::new();

    writeln!(obj, "# Forge test dump").unwrap();
    writeln!(
        obj,
        "# Faces: {}, Vertices: {}, Edges: {}",
        arena.face_count(),
        arena.vertex_count(),
        arena.edge_count()
    )
    .unwrap();
    writeln!(obj).unwrap();

    // Map VertexId → 1-based OBJ index
    let mut vid_to_idx = std::collections::HashMap::new();
    let mut idx = 1u32;

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            writeln!(obj, "v {:.8} {:.8} {:.8}", pos[0], pos[1], pos[2]).unwrap();
            vid_to_idx.insert(vid, idx);
            idx += 1;
        }
    }

    writeln!(obj).unwrap();

    // Emit face normals
    for (fid, _) in arena.iter_faces() {
        if let Some(plane) = geom.get_face_plane(fid) {
            let n = plane.normal();
            writeln!(obj, "vn {:.8} {:.8} {:.8}", n[0], n[1], n[2]).unwrap();
        }
    }

    writeln!(obj).unwrap();

    // Emit faces
    let mut normal_idx = 1u32;
    for (fid, _) in arena.iter_faces() {
        let hes = arena.halfedges_of_face(fid);
        let indices: Vec<u32> = hes
            .iter()
            .filter_map(|he_id| {
                let he = arena.get_half_edge(*he_id).ok()?;
                vid_to_idx.get(&he.origin()).copied()
            })
            .collect();

        if indices.is_empty() {
            continue;
        }

        let has_normal = geom.get_face_plane(fid).is_some();

        write!(obj, "f").unwrap();
        for v_idx in &indices {
            if has_normal {
                write!(obj, " {}//{}", v_idx, normal_idx).unwrap();
            } else {
                write!(obj, " {}", v_idx).unwrap();
            }
        }
        writeln!(obj).unwrap();

        if has_normal {
            normal_idx += 1;
        }
    }

    obj
}

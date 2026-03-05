import os
import re

invariants = [
    "radial_reciprocity",
    "next_prev_reciprocity",
    "no_dangling_refs",
    "generational_freshness",
    "face_has_loop",
    "loop_min_cardinality",
    "no_duplicate_coedges",
    "face_loop_membership",
    "vertex_continuity",
    "edge_endpoints_match",
    "single_loop_owner",
    "no_orphan_half_edges",
    "acyclic_containment",
    "inner_outer_consistency",
    "radial_cycle_uniqueness",
    "radial_neighbor_consistency",
    "no_broken_radial_splices",
    "face_adjacency_consistency",
    "no_broken_face_boundary",
    "boundary_edges_laminar",
    "disk_entries_alive",
    "disk_partition_correct",
    "disk_closure",
    "no_cross_disk_coedges",
    "per_component_euler",
    "side_car_coherence",
    "index_coherence",
]

contract_template = "    const INVARIANT_CONTRACT: InvariantContract = InvariantContract {\n"
for inv in invariants:
    contract_template += f"        {inv}: InvariantRelation::MayBreak,\n"
contract_template += "    };"


def replace_contract(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Pattern for conservative_contract!()
    pattern1 = r"const INVARIANT_CONTRACT:\s*InvariantContract\s*=\s*crate::conservative_contract!\(\);"
    
    # Pattern for the other agent's verbose closure block
    pattern2 = r"const INVARIANT_CONTRACT:\s*InvariantContract\s*=\s*InvariantContract\s*\{\s*relation:\s*\|id\|\s*match\s*id\s*\{.*?\},\s*\};\s*"

    pattern3 = r"const INVARIANT_CONTRACT:\s*crate::validators::invariant_id::InvariantContract\s*=\s*crate::conservative_contract!\(\);"

    new_content = re.sub(pattern1, contract_template, content, flags=re.DOTALL)
    new_content = re.sub(pattern2, contract_template, new_content, flags=re.DOTALL)
    new_content = re.sub(pattern3, contract_template, new_content, flags=re.DOTALL)

    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Updated: {filepath}")

def main():
    topo_dir = "/Users/spenstar/Documents/programming/forge workspace/Forge/crates/forge-topo/src"
    for root, _, files in os.walk(topo_dir):
        for file in files:
            if file.endswith(".rs"):
                filepath = os.path.join(root, file)
                replace_contract(filepath)

if __name__ == "__main__":
    main()

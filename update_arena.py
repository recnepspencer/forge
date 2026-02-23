import re

with open('crates/forge-topo/src/arena/eval.rs', 'r') as f:
    content = f.read()

# Add imports if missing
if 'use crate::lineage_store::LineageStore;' not in content:
    content = content.replace('use crate::attributes::AttributeStore;', 'use crate::attributes::AttributeStore;\nuse crate::lineage_store::LineageStore;\nuse forge_core::{EntityRef, EntityKind};')

# Helper function to modify inserts
def modify_insert(kind_str, method_name, entity_kind, entity_id_type, data_type):
    global content
    pattern = r'(pub fn ' + method_name + r'\(&mut self, (mut )?data: ' + data_type + r'\) -> ' + entity_id_type + r' \{.*?^    \})'
    
    def replacer(m):
        old_impl = m.group(1)
        # Change signature
        new_impl = old_impl.replace('data: ' + data_type + ') -> ' + entity_id_type, 
                                  'data: ' + data_type + ', mut ls: Option<&mut LineageStore>) -> ' + entity_id_type)
        if 'mut ls: Option' not in old_impl:
            # Add lineage store logic before returning
            ret_statement = entity_id_type + r'::new(index, gen)'
            new_ret = f'''let id = {entity_id_type}::new(index, gen);
        if let Some(store) = ls.as_deref_mut() {{
            if let Some(lin) = self.{kind_str}_slots[index as usize].data.as_ref().unwrap().lineage().cloned() {{
                store.record_creation(EntityRef::new(EntityKind::{entity_kind}, id.index()), lin);
            }}
        }}
        id'''
            new_impl = new_impl.replace(ret_statement, new_ret)
        return new_impl
    
    content = re.sub(pattern, replacer, content, flags=re.MULTILINE | re.DOTALL)

# Modify removals
def modify_remove(kind_str, method_name, entity_kind, entity_id_type, data_type):
    global content
    pattern = r'(pub fn ' + method_name + r'\(&mut self, id: ' + entity_id_type + r'\) -> Result<' + data_type + r', KernelError> \{.*?^    \})'
    
    def replacer(m):
        old_impl = m.group(1)
        new_impl = old_impl.replace('id: ' + entity_id_type + ') -> Result<', 
                                  'id: ' + entity_id_type + ', mut ls: Option<&mut LineageStore>) -> Result<')
        if 'mut ls: Option' not in old_impl:
            # Add lineage tracking before Ok(data)
            ret_statement = r'Ok(data)'
            new_ret = f'''if let Some(store) = ls.as_deref_mut() {{
            let _ = store.record_deletion(EntityRef::new(EntityKind::{entity_kind}, id.index()));
        }}
        Ok(data)'''
            new_impl = new_impl.replace(ret_statement, new_ret)
        return new_impl
    
    content = re.sub(pattern, replacer, content, flags=re.MULTILINE | re.DOTALL)

# Handle all standard entities
modify_insert('face', 'insert_face', 'Face', 'FaceId', 'FaceData')
modify_remove('face', 'remove_face', 'Face', 'FaceId', 'FaceData')

modify_insert('half_edge', 'insert_half_edge', 'HalfEdge', 'HalfEdgeId', 'HalfEdgeData')
modify_remove('half_edge', 'remove_half_edge', 'HalfEdge', 'HalfEdgeId', 'HalfEdgeData')

modify_insert('vertex', 'insert_vertex', 'Vertex', 'VertexId', 'VertexData')
modify_remove('vertex', 'remove_vertex', 'Vertex', 'VertexId', 'VertexData')

modify_insert('loop', 'insert_loop', 'Loop', 'LoopId', 'LoopData')
modify_remove('loop', 'remove_loop', 'Loop', 'LoopId', 'LoopData')

modify_insert('shell', 'insert_shell', 'Shell', 'ShellId', 'ShellData')
modify_remove('shell', 'remove_shell', 'Shell', 'ShellId', 'ShellData')

modify_insert('edge', 'insert_edge', 'Edge', 'EdgeId', 'EdgeData')
modify_remove('edge', 'remove_edge', 'Edge', 'EdgeId', 'EdgeData')

# Special case for insert_half_edge_pair
pattern_pair = r'(pub fn insert_half_edge_pair\([\s\S]*?\) -> \(HalfEdgeId, HalfEdgeId\) \{[\s\S]*?^    \})'
def replacer_pair(m):
    old_impl = m.group(1)
    new_impl = old_impl.replace('mut data_b: HalfEdgeData,', 'mut data_b: HalfEdgeData,\n        mut ls: Option<&mut LineageStore>,')
    if 'mut ls: Option' not in old_impl:
        ret_statement = r'\(HalfEdgeId::new(base, gen_a), HalfEdgeId::new(base \+ 1, gen_b)\)'
        new_ret = f'''let id_a = HalfEdgeId::new(base, gen_a);
        let id_b = HalfEdgeId::new(base + 1, gen_b);
        if let Some(store) = ls.as_deref_mut() {{
            if let Some(lin) = self.half_edge_slots[base as usize].data.as_ref().unwrap().lineage().cloned() {{
                store.record_creation(EntityRef::new(EntityKind::HalfEdge, id_a.index()), lin);
            }}
            if let Some(lin) = self.half_edge_slots[(base + 1) as usize].data.as_ref().unwrap().lineage().cloned() {{
                store.record_creation(EntityRef::new(EntityKind::HalfEdge, id_b.index()), lin);
            }}
        }}
        (id_a, id_b)'''
        new_impl = new_impl.replace(ret_statement, new_ret)
    return new_impl
content = re.sub(pattern_pair, replacer_pair, content, flags=re.MULTILINE)

with open('crates/forge-topo/src/arena/eval.rs', 'w') as f:
    f.write(content)


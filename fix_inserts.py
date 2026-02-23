import os
import re

directories = [
    'crates/forge-topo/src',
]

patterns = [
    r'insert_face', r'insert_half_edge', r'insert_half_edge_pair',
    r'insert_vertex', r'insert_loop', r'insert_shell', r'insert_edge',
    r'remove_face', r'remove_half_edge', r'remove_vertex', r'remove_loop',
    r'remove_shell', r'remove_edge'
]

# We want to replace calls like:
# arena.insert_face(data)
# with:
# arena.insert_face(data, None)
#
# But for operators, we want to replace:
# draft.arena_mut().insert_face(data)
# with:
# let (arena, store) = draft.elements_mut();
# arena.insert_face(data, Some(store))
# ... this is too complex for simple regex.

# Let's just blindly add , None to all matches of `.insert_X(...)` and `.remove_X(...)`
# and then manually fix the Euler operators where we actually want `Some(store)`.

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    for p in patterns:
        # Match `.method(args)`
        # Handle newlines inside args by matching up to the matching closing paren.
        # This is a bit tricky with regex, we can use a small parser.
        idx = 0
        while True:
            # find next `.p(`
            match = re.search(r'\.' + p + r'\s*\(', content[idx:])
            if not match:
                break
            start = idx + match.end()
            
            # find matching parenthesis
            depth = 1
            curr = start
            while curr < len(content):
                if content[curr] == '(':
                    depth += 1
                elif content[curr] == ')':
                    depth -= 1
                    if depth == 0:
                        break
                curr += 1
            
            if depth == 0:
                # content[start:curr] is the arguments
                args = content[start:curr]
                # If it already has two arguments (contains a comma at the top level? no, could be nested).
                # Assume if the file was just modified, it doesn't have `None` yet, except maybe in `eval.rs` where we added `mut ls: Option...`
                # Let's skip `eval.rs` definition lines. It doesn't have `.insert_face(` as a method call usually, 
                # except maybe `self.insert_face...`
                
                # Check if it's already fixed (ends with `None` or `store`)
                if not re.search(r',\s*(?:None|Some\([^)]+\)|store|ls(?:_store)?)\s*$', args):
                    # We need to insert `, None`
                    content = content[:curr] + ', None' + content[curr:]
                    idx = curr + 6 # advance past `, None)`
                else:
                    idx = curr + 1
            else:
                idx = start + 1

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)

for root, _, files in os.walk(directories[0]):
    for file in files:
        if file.endswith('.rs'):
            fix_file(os.path.join(root, file))


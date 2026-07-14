# Graph And Nodes

Start here if you need to understand the graph itself.

A graph is the map of what depends on what.

In a commerce app, that can look like this:

- `product_price`
- `shipping_quote`
- `checkout_summary`

If `checkout_summary` depends on `product_price` and `shipping_quote`, the
graph is where you state that.

## Main Surfaces

- `SignalGraph`
- `graph.node()`
- `NodeBuilder`
- `graph.set_dependencies(...)`
- `DependencyEdge`

## Mental Model

Use these terms:

- source nodes: raw inputs or facts
- computed nodes: results built from other nodes
- dependencies: which upstream nodes matter to which downstream nodes

Do not treat nodes like random containers. A node should stand for a real thing
in the system.

## Rule

If you can name the thing clearly, it is probably a good node:

- product price
- search index shard
- document preview
- compiler diagnostics for one file

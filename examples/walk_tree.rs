use hwloc2::{Topology, TopologyObject};

/// Walk the topology in a tree-style and print it.
fn main() {
    let topo = Topology::new().unwrap();

    println!("*** Printing overall tree");
    print_children(&topo, topo.object_at_root(), 0);
}

fn print_children(topo: &Topology, obj: &TopologyObject, depth: usize) {
    let padding = " ".repeat(2 * depth);
    println!(
        "{}{}: #{}\t(type='{:?}')",
        padding,
        obj,
        obj.os_index(),
        obj.object_type()
    );

    for i in 0..obj.arity() {
        print_children(topo, obj.children()[i as usize], depth + 1);
    }
}

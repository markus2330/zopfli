use libc::{size_t, c_int, c_char, c_uint};

// Bounded package merge algorithm, based on the paper
// "A Fast and Space-Economical Algorithm for Length-Limited Coding
// Jyrki Katajainen, Alistair Moffat, Andrew Turpin".

/// Nodes forming chains. Also used to represent leaves.
#[repr(C)]
pub struct Node {
  weight: size_t,     // Total weight (symbol count) of this chain.
  tail: *const Node,  // Previous node(s) of this chain, or 0 if none.
  count: c_int,       // Leaf symbol index, or number of leaves before this chain.
  inuse: c_char,      // Tracking for garbage collection.
}

/// Memory pool for nodes.
#[repr(C)]
pub struct NodePool {
  nodes: *const Node,  // The pool.
  next: *const Node,   // Pointer to a possibly free node in the pool.
  size: c_int,         // Size of the memory pool.
}


/// Initializes a chain node with the given values and marks it as in use.
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn InitNode(weight: size_t, count: c_int, tail: *const Node, node_ptr: *mut Node) {
    let node = unsafe {
        assert!(!node_ptr.is_null());
        &mut *node_ptr
    };

    node.weight = weight;
    node.count = count;
    node.tail = tail;
    node.inuse = 1;
}

/// Converts result of boundary package-merge to the bitlengths. The result in the
/// last chain of the last list contains the amount of active leaves in each list.
/// chain: Chain to extract the bit length from (last chain from last list).
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ExtractBitLengths(chain: *const Node, leaves: *const Node, bitlengths: *mut c_uint) {
    if chain.is_null() {
        return;
    }
    let mut node_ptr = chain;
    while !node_ptr.is_null() {
        let node = unsafe {
           &*node_ptr
        };
        for i in 0..node.count {
            unsafe {
                let leaf = &*leaves.offset(i as isize);
                *bitlengths.offset(leaf.count as isize) += 1;
            }
        }
        node_ptr = node.tail;
    }
}

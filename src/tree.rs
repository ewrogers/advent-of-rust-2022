// Tree structures are notoriously difficult in Rust,
// mainly due to the strict borrow checker that often
// results in Rc<RefCell<T>> approaches or worse...

// This uses a vector to store all tree nodes by index instead,
// avoiding all the headaches of lifetimes and references
#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub nodes: Vec<Node<T>>,
}

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub index: usize,
    pub value: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn new(index: usize, value: T) -> Self {
        Self {
            index,
            value,
            parent: None,
            children: vec![],
        }
    }
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    // Gets the number of nodes in the tree
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    // Attempts to get the index of a node with the value, or inserts it
    pub fn find_or_add_node(&mut self, value: T) -> usize {
        // First check if the value is stored already
        if let Some(index) = self.find_node(&value) {
            return index;
        }

        // Otherwise add the new to the tree and return the new index
        let index = self.nodes.len();
        self.nodes.push(Node::new(index, value));
        index
    }

    // Attempts to find an existing node with the value, returning the index (if found)
    pub fn find_node(&self, value: &T) -> Option<usize> {
        for node in &self.nodes {
            if &node.value == value {
                return Some(node.index);
            }
        }
        None
    }

    // Attempts to find an existing node matching the predicate, returning the index (if true)
    pub fn find_node_by<P>(&self, filter: P) -> Option<usize>
    where
        P: Fn(&Node<T>) -> bool,
    {
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            if filter(node) {
                return Some(node.index);
            }
        }
        None
    }

    pub fn set_parent_child(&mut self, parent: usize, child: usize) {
        if let Some(prev_parent) = self.nodes[child].parent {
            self.nodes[prev_parent].children.remove(child);
        }

        if !self.nodes[parent].children.contains(&child) {
            self.nodes[parent].children.push(child);
        }
        self.nodes[child].parent = Some(parent);
    }

    pub fn traverse<F>(&self, index: usize, f: &mut F)
    where
        F: FnMut(&Node<T>, usize),
    {
        self.traverse_with_depth(index, f, 0)
    }

    fn traverse_with_depth<F>(&self, index: usize, f: &mut F, depth: usize)
    where
        F: FnMut(&Node<T>, usize),
    {
        let node = &self.nodes[index];
        f(node, depth);

        for child in &node.children {
            self.traverse_with_depth(*child, f, depth + 1);
        }
    }
}

// Linked lists can be annoying in Rust with Box<T> or worse...
// This uses a vector to store all linked nodes by index instead,
// avoiding all the headaches of lifetimes and references
#[derive(Debug, Default)]
pub struct ArenaLinkedList<T> {
    pub nodes: Vec<LinkedListNode<T>>,
}

#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub index: usize,
    pub value: T,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl<T> ArenaLinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    #[must_use]
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = Self {
            nodes: Vec::with_capacity(vec.len()),
        };

        for item in vec {
            list.push(item);
        }

        list
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.len() < 1
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn head(&self) -> Option<&LinkedListNode<T>> {
        self.nodes.first()
    }

    #[must_use]
    pub fn tail(&self) -> Option<&LinkedListNode<T>> {
        self.nodes.last()
    }

    // Gets the first value of the list
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        match self.nodes.first() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    // Gets the last value of the list
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        match self.nodes.last() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    // Gets an immutable reference to a value within the list
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.nodes.get(index) {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    // Gets a mutable reference to a value within the list
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.nodes.get_mut(index) {
            Some(node) => Some(&mut node.value),
            None => None,
        }
    }

    // Pushes a new value to the end of the list, returning the index of the new item
    pub fn push(&mut self, value: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(LinkedListNode {
            index,
            value,
            prev: if index > 0 { Some(index - 1) } else { None },
            next: None,
        });

        if index > 0 {
            self.nodes[index - 1].next.replace(index);
        }
        index
    }

    // Pops the last value off the list, returning it
    pub fn pop(&mut self) -> Option<T> {
        let node = self.nodes.pop()?;

        let last = self.nodes.len() - 1;
        self.nodes[last].next = None;

        Some(node.value)
    }

    // Traverses the linked list from head to tail
    pub fn traverse<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        let Some(mut current) = self.nodes.first() else {
            return;
        };

        loop {
            func(&current.value);

            current = match current.next {
                Some(index) => &self.nodes[index],
                None => break,
            }
        }
    }

    // Traverses the linked list from tail to head (reversed)
    pub fn traverse_rev<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        let Some(mut current) = self.nodes.last() else {
            return;
        };

        loop {
            func(&current.value);

            current = match current.prev {
                Some(index) => &self.nodes[index],
                None => break,
            }
        }
    }
}

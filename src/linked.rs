// Linked lists can be annoying in rust with Box<T> or worse...
// This uses a vector to store all linked nodes by index instead,
// avoiding all the headaches of lifetimes and references
#[derive(Debug, Default)]
pub struct ArenaLinkedList<T>
where
    T: PartialEq,
{
    pub nodes: Vec<LinkedListNode<T>>,
}

#[derive(Debug)]
pub struct LinkedListNode<T>
where
    T: PartialEq,
{
    pub index: usize,
    pub value: T,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl<T> ArenaLinkedList<T>
where
    T: PartialEq,
{
    pub fn is_empty(&self) -> bool {
        self.nodes.len() < 1
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn head(&self) -> Option<&LinkedListNode<T>> {
        self.nodes.first()
    }

    pub fn tail(&self) -> Option<&LinkedListNode<T>> {
        self.nodes.last()
    }

    // Gets the first value of the list
    pub fn first(&self) -> Option<&T> {
        match self.nodes.first() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    // Gets the last value of the list
    pub fn last(&self) -> Option<&T> {
        match self.nodes.last() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

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

    pub fn pop(&mut self) -> Option<T> {
        let node = match self.nodes.pop() {
            Some(node) => node,
            None => return None,
        };

        let last = self.nodes.len() - 1;
        self.nodes[last].next = None;

        Some(node.value)
    }

    // Traverses the linked list from head to tail
    pub fn traverse<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        let mut current = match self.nodes.first() {
            Some(node) => node,
            None => return,
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
        let mut current = match self.nodes.last() {
            Some(node) => node,
            None => return,
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

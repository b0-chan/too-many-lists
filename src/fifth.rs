use std::ptr;

struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Box<Self> {
        Box::new(Self {
            elem,
            next: None
        })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut()
        }
    }

    fn push(&mut self, elem: T) {
        let mut new_tail = Node::new(elem);

        let raw_tail= &mut *new_tail as *mut _;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }
}
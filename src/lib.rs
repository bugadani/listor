use std::ops::{Index, IndexMut};

enum Entry<T> {
    Occupied(T),
    Vacant,
}

struct Node<T> {
    data: Entry<T>,
    prev: usize,
    next: usize,
}

pub struct Listor<T> {
    elements: Vec<Node<T>>,
    /// Number of occupied entries.
    count: usize,
    /// Index of first occupied entry.
    head: usize,
    /// Index of last occupied entry.
    tail: usize,
    /// False if the listor can grow.
    bounded: bool,
}

pub struct Iter<'a, T> {
    listor: &'a Listor<T>,
    current: usize,
}

impl<T> Default for Listor<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Listor<T> {
    fn create(capacity: usize, bounded: bool) -> Self {
        Self {
            elements: (0..capacity)
                .map(|i| Node {
                    data: Entry::Vacant,
                    prev: i.saturating_sub(1),
                    next: i.saturating_add(1).min(capacity - 1),
                })
                .collect(),
            count: 0,
            head: 0,
            tail: 0,
            bounded,
        }
    }

    /// Creates a new, unbounded, empty listor.
    pub fn new() -> Self {
        Self::create(0, false)
    }

    /// Creates a new, unbounded listor with `capacity` vacant entries.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::create(capacity, false)
    }

    /// Creates a new, bounded listor with `capacity` vacant entries.
    pub fn bounded(capacity: usize) -> Self {
        Self::create(capacity, true)
    }

    /// Removes all elements from the Listor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    ///
    /// listor.push_back(1).unwrap();
    /// listor.push_back(2).unwrap();
    /// listor.push_back(3).unwrap();
    ///
    /// listor.clear();
    ///
    /// assert_eq!(0, listor.len());
    /// ```
    pub fn clear(&mut self) {
        let max_next = self.elements.len() - 1;
        for (idx, node) in self.elements.iter_mut().enumerate() {
            *node = Node {
                data: Entry::Vacant,
                prev: idx.saturating_sub(1),
                next: idx.saturating_add(1).min(max_next),
            };
        }
        self.count = 0;
        self.head = 0;
        self.tail = 0;
    }

    /// Returns the index where the next inserted item will be placed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::<usize>::new();
    /// let idx = listor.next_vacant_index();
    /// assert_eq!(Some(0), idx);
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::bounded(3);
    /// listor.push_back(0);
    /// let idx = listor.next_vacant_index();
    /// assert_eq!(Some(1), idx);
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::<usize>::bounded(0);
    /// let idx = listor.next_vacant_index();
    /// assert_eq!(None, idx);
    /// ```
    pub fn next_vacant_index(&self) -> Option<usize> {
        if let Some(idx) = self.next_free_idx() {
            Some(idx)
        } else if self.bounded {
            None
        } else {
            Some(self.elements.len())
        }
    }

    fn next_free_idx(&self) -> Option<usize> {
        if let Some(tail) = self.elements.get(self.tail) {
            if let Entry::Vacant = tail.data {
                return Some(self.tail);
            } else if tail.next != self.tail {
                return Some(tail.next);
            }
        }
        None
    }

    fn allocate(&mut self) -> Option<usize> {
        if let Some(idx) = self.next_free_idx() {
            self.count += 1;
            Some(idx)
        } else if self.bounded {
            None
        } else {
            let idx = self.elements.len();

            if let Some(tail) = self.elements.get_mut(self.tail) {
                tail.next = idx;
            }
            self.elements.push(Node {
                data: Entry::Vacant,
                prev: self.tail,
                next: idx,
            });

            self.count += 1;
            Some(idx)
        }
    }

    /// Pushes an element to the back of the list and returns the index at which it can be accessed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// let idx = listor.push_back(5).unwrap();
    /// assert_eq!(5, listor[idx]);
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::bounded(2);
    /// assert_eq!(Ok(0), listor.push_back(5));
    /// assert_eq!(Ok(1), listor.push_back(6));
    /// assert_eq!(Err(7), listor.push_back(7));
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_back(5);
    /// listor.push_back(6);
    /// listor.push_back(7);
    /// assert_eq!(Some(7), listor.pop_back());
    /// assert_eq!(Some(6), listor.pop_back());
    /// assert_eq!(Some(5), listor.pop_back());
    /// assert_eq!(None, listor.pop_back());
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_back(5);
    /// listor.push_back(6);
    /// listor.push_back(7);
    /// assert_eq!(Some(5), listor.pop_front());
    /// assert_eq!(Some(6), listor.pop_front());
    /// assert_eq!(Some(7), listor.pop_front());
    /// assert_eq!(None, listor.pop_front());
    /// ```
    pub fn push_back(&mut self, item: T) -> Result<usize, T> {
        match self.allocate() {
            Some(idx) => {
                // The allocated element is guaranteed to be our tail.
                self.tail = idx;
                self.elements[idx].data = Entry::Occupied(item);

                Ok(idx)
            }

            None => Err(item),
        }
    }

    /// Pushes an element to the front of the list and returns the index at which it can be accessed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// let idx = listor.push_front(5).unwrap();
    /// assert_eq!(5, listor[idx]);
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::bounded(2);
    /// assert_eq!(Ok(0), listor.push_front(5));
    /// assert_eq!(Ok(1), listor.push_front(6));
    /// assert_eq!(Err(7), listor.push_front(7));
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_front(5);
    /// listor.push_front(6);
    /// listor.push_front(7);
    /// assert_eq!(Some(5), listor.pop_back());
    /// assert_eq!(Some(6), listor.pop_back());
    /// assert_eq!(Some(7), listor.pop_back());
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_front(5);
    /// listor.push_front(6);
    /// listor.push_front(7);
    /// assert_eq!(Some(7), listor.pop_front());
    /// assert_eq!(Some(6), listor.pop_front());
    /// assert_eq!(Some(5), listor.pop_front());
    /// ```
    pub fn push_front(&mut self, item: T) -> Result<usize, T> {
        match self.allocate() {
            Some(idx) => {
                self.elements[idx].data = Entry::Occupied(item);

                if idx != self.head {
                    self.remove_node(idx);
                    self.insert_before(idx, self.head);

                    self.head = idx;
                }
                Ok(idx)
            }

            None => Err(item),
        }
    }

    fn remove_node(&mut self, idx: usize) {
        let node = &self.elements[idx];

        let prev = node.prev;
        let next = node.next;

        match (prev == idx, next == idx) {
            (true, true) => {}
            (false, true) => {
                // removing a tail
                self.elements[prev].next = prev;
            }
            (true, false) => {
                // removing a head
                self.elements[next].prev = next;
            }
            (false, false) => {
                self.elements[prev].next = next;
                self.elements[next].prev = prev;
            }
        }
    }

    fn insert_before(&mut self, idx: usize, next: usize) {
        let node = &self.elements[next];

        let prev = node.prev;

        if prev == next {
            // we want to insert before head
            self.elements[next].prev = idx;
            self.elements[idx].next = next;
            self.elements[idx].prev = idx;
        } else {
            self.insert_between(idx, prev, next);
        }
    }

    fn insert_after(&mut self, idx: usize, prev: usize) {
        let node = &self.elements[prev];

        let next = node.next;

        if prev == next {
            // we want to insert after tail
            self.elements[prev].next = idx;
            self.elements[idx].prev = next;
            self.elements[idx].next = idx;
        } else {
            self.insert_between(idx, prev, next);
        }
    }

    fn insert_between(&mut self, idx: usize, prev: usize, next: usize) {
        self.elements[next].prev = idx;
        self.elements[idx].next = next;

        self.elements[idx].prev = prev;
        self.elements[prev].next = idx;
    }

    /// Pops an element off the back of the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_back(5);
    /// listor.push_back(6);
    ///
    /// assert_eq!(Some(6), listor.pop_back());
    /// assert_eq!(Some(5), listor.pop_back());
    /// assert_eq!(None, listor.pop_back());
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove(self.tail)
    }

    /// Pops an element off the beginning of the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_back(5);
    /// listor.push_back(6);
    ///
    /// assert_eq!(Some(5), listor.pop_front());
    /// assert_eq!(Some(6), listor.pop_front());
    /// assert_eq!(None, listor.pop_front());
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(self.head)
    }

    /// Returns the number of occupied entries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    /// listor.push_back(5);
    /// assert_eq!(1, listor.len());
    /// ```
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::bounded(2);
    /// listor.push_back(5);
    /// assert_eq!(1, listor.len());
    /// listor.push_back(5);
    /// assert_eq!(2, listor.len());
    /// listor.push_back(5);
    /// assert_eq!(2, listor.len());
    /// listor.pop_back();
    /// assert_eq!(1, listor.len());
    /// ```
    pub fn len(&self) -> usize {
        self.count
    }

    /// Removes an element from the list.
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    ///
    /// assert_eq!(None, listor.remove(4));
    ///
    /// listor.push_back(5);
    /// let idx = listor.push_back(6).unwrap();
    /// listor.push_back(7);
    ///
    /// assert_eq!(Some(6), listor.remove(idx));
    ///
    /// assert_eq!(Some(5), listor.pop_front());
    /// assert_eq!(Some(7), listor.pop_front());
    /// assert_eq!(None, listor.pop_front());
    /// ```
    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if let Some(node) = self.elements.get_mut(idx) {
            match std::mem::replace(&mut node.data, Entry::Vacant) {
                Entry::Vacant => None,
                Entry::Occupied(item) => {
                    if self.head != self.tail {
                        if idx == self.tail {
                            self.tail = node.prev;
                        } else {
                            if idx == self.head {
                                self.head = node.next;
                            }
                            self.remove_node(idx);
                            self.insert_after(idx, self.tail);
                        }
                    }

                    self.count -= 1;
                    Some(item)
                }
            }
        } else {
            None
        }
    }

    /// Returns a reference to the first value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    ///
    /// listor.push_back(5);
    /// listor.push_back(6);
    ///
    /// assert_eq!(Some(&5), listor.peek_front());
    /// assert_eq!(Some(&5), listor.peek_front());
    ///
    /// listor.pop_front();
    ///
    /// assert_eq!(Some(&6), listor.peek_front());
    /// ```
    pub fn peek_front(&self) -> Option<&T> {
        self.get(self.head)
    }

    /// Returns a reference to the last value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    ///
    /// listor.push_back(5);
    /// listor.push_back(6);
    ///
    /// assert_eq!(Some(&6), listor.peek_back());
    /// assert_eq!(Some(&6), listor.peek_back());
    ///
    /// listor.pop_back();
    ///
    /// assert_eq!(Some(&5), listor.peek_back());
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        self.get(self.tail)
    }

    /// Returns a reference to the indexed value.
    pub fn get(&self, idx: usize) -> Option<&T> {
        match self.elements.get(idx)?.data {
            Entry::Vacant => None,
            Entry::Occupied(ref element) => Some(element),
        }
    }

    /// Returns a mutable reference to the indexed value.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        match self.elements.get_mut(idx)?.data {
            Entry::Vacant => None,
            Entry::Occupied(ref mut element) => Some(element),
        }
    }

    /// Iterate over the elements, from front to back.
    ///
    /// # Example
    ///
    /// ```rust
    /// use listor::Listor;
    ///
    /// let mut listor = Listor::new();
    ///
    /// listor.push_back(5);
    /// listor.push_back(6);
    ///
    /// let mut iter = listor.iter();
    ///
    /// assert_eq!(Some(&5), iter.next());
    /// assert_eq!(Some(&6), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            listor: self,
            current: self.head,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.listor.elements.get(self.current) {
            self.current = if node.next == self.current {
                self.listor.len()
            } else {
                node.next
            };

            match &node.data {
                Entry::Occupied(data) => Some(data),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<T> Index<usize> for Listor<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Out of bounds access")
    }
}

impl<T> IndexMut<usize> for Listor<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Out of bounds access")
    }
}

#[cfg(test)]
mod test {
    use crate::Listor;

    #[test]
    fn test_unbounded_reuses_indexes() {
        let mut listor = Listor::new();

        let _ = listor.push_back(4);
        let idx = listor.push_back(5).unwrap();
        let _ = listor.push_back(6);
        let _ = listor.push_back(7);

        // remove three elements
        listor.pop_front();
        listor.pop_back();
        listor.remove(idx);

        // re-insert three elements
        let _ = listor.push_back(9);
        let _ = listor.push_front(8);
        let _ = listor.push_back(7);

        assert_eq!(4, listor.len());

        for i in 0..4 {
            assert!(listor.get(i).is_some())
        }
    }

    #[test]
    fn remove_preserves_iteration_order() {
        let mut listor = Listor::new();

        let idx1 = listor.push_back(4).unwrap();
        let _ = listor.push_back(5);
        let _ = listor.push_back(6);
        let idx2 = listor.push_back(7).unwrap();

        listor.remove(idx1);
        listor.remove(idx2);

        let _ = listor.push_back(8);

        let mut iter = listor.iter();

        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&6), iter.next());
        assert_eq!(Some(&8), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn removing_only_element() {
        let mut listor = Listor::bounded(4);

        let idx1 = listor.push_back(4).unwrap();

        listor.remove(idx1);

        let _ = listor.push_back(8);
        let _ = listor.push_back(9);
        let _ = listor.push_back(10);
        let _ = listor.push_back(11);

        let mut iter = listor.iter();

        assert_eq!(Some(&8), iter.next());
        assert_eq!(Some(&9), iter.next());
        assert_eq!(Some(&10), iter.next());
        assert_eq!(Some(&11), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn removing_middle_element() {
        let mut listor = Listor::bounded(4);

        let _ = listor.push_back(7);
        let idx1 = listor.push_back(8).unwrap();

        let _ = listor.push_back(9);
        let _ = listor.push_back(10);

        listor.remove(idx1);

        let _ = listor.push_back(11);

        assert_eq!(Some(7), listor.pop_front());
        assert_eq!(Some(9), listor.pop_front());
        assert_eq!(Some(10), listor.pop_front());
        assert_eq!(Some(11), listor.pop_front());
        assert_eq!(None, listor.pop_front());
    }

    #[test]
    fn pop_front_preserves_iteration_order() {
        let mut listor = Listor::new();

        let _ = listor.push_back(4);
        let _ = listor.push_back(5);
        let _ = listor.push_back(6);
        let _ = listor.push_back(7);
        let _ = listor.push_back(8);

        listor.pop_front();
        listor.pop_front();
        listor.pop_back();

        let _ = listor.push_back(9);

        let mut iter = listor.iter();

        assert_eq!(Some(&6), iter.next());
        assert_eq!(Some(&7), iter.next());
        assert_eq!(Some(&9), iter.next());
        assert_eq!(None, iter.next());
    }
}

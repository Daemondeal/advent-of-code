pub struct Heap<T> {
    array: Vec<(T, f32)>,
}

impl<T: Eq> Heap<T> {
    pub fn new() -> Self {
        Heap { array: vec![] }
    }

    pub fn insert(&mut self, element: T, value: f32) {
        self.array.push((element, value));
        self.shift_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let size = self.len();

        if size == 0 {
            None
        } else {
            self.array.swap(0, size - 1);
            let element = self.array.pop().unwrap();

            self.shift_down(0);

            Some(element.0)
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        for (test, _) in &self.array {
            if test.eq(element) {
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    fn shift_up(&mut self, mut i: usize) {
        let mut parent = i / 2;
        while i > 0 && self.array[parent].1 < self.array[i].1 {
            self.array.swap(parent, i);
            i = parent;
            parent = i / 2;
        }
    }

    fn shift_down(&mut self, i: usize) {
        let left = 2 * i;
        let right = 2 * i + 1;

        let smallest = if left < self.len() && self.array[left].1 < self.array[i].1 {
            left
        } else {
            i
        };

        let smallest = if right < self.len() && self.array[right].1 < self.array[i].1 {
            right
        } else {
            smallest
        };

        if smallest != i {
            self.array.swap(i, smallest);
            self.shift_down(smallest);
        }
    }
}

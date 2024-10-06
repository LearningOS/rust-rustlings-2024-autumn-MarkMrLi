/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        
        self.count += 1;
        println!("heap_len: {}",self.len());
        let mut now_ind = self.count;
        self.items.push(value);
        while (self.comparator)(&self.items[now_ind],&self.items[self.parent_idx(now_ind)]) && now_ind > 1{
            let pa_ind = self.parent_idx(now_ind);
            self.items.swap(now_ind, pa_ind);
            now_ind = pa_ind;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> 

    {
        //TODO
        if self.is_empty(){
            None
        } else {
            let ret = Some(self.items[1]);
            let last_ind = self.len();
            self.items.swap(last_ind, 1);
            self.items.pop();
            self.count -= 1;

            let mut now_ind = 1;
            loop {
                if self.children_present(now_ind) {
                    if self.right_child_idx(now_ind) <= self.count{
                        let left_ind = self.left_child_idx(now_ind);
                        let right_ind = self.right_child_idx(now_ind);
                        let smaller_child_ind = if (self.comparator)(&self.items[left_ind],&self.items[right_ind]) {
                            left_ind
                        } else {
                            right_ind
                        };
                        if (self.comparator)(&self.items[smaller_child_ind],&self.items[now_ind]){
                            self.items.swap(now_ind, smaller_child_ind);
                            now_ind = smaller_child_ind;
                        } else {
                            break;
                        }
                    }
                    else {
                        let smaller_child_ind = self.left_child_idx(now_ind);
                        if (self.comparator)(&self.items[smaller_child_ind],&self.items[now_ind]){
                            self.items.swap(now_ind, smaller_child_ind);
                            now_ind = smaller_child_ind;
                            break;
                        } else {
                            break;
                        }
                    }
                }
                else {
                    break;
                }
            }
            ret
        }
		
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
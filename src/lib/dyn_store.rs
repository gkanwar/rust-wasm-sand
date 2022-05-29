use std::default::Default;

pub struct DynStore<T: Default> {
    store: Vec<T>,
    free: Vec<bool>,
    count: usize,
}

impl<T: Default> DynStore<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            store: Vec::<T>::with_capacity(capacity),
            free: Vec::<bool>::with_capacity(capacity),
            count: 0,
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.store.len() <= i || self.free[i] {
            None
        } else {
            Some(&mut self.store[i])
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if self.store.len() <= i || self.free[i] {
            None
        } else {
            Some(&self.store[i])
        }
    }

    pub fn add(&mut self, obj: T) -> usize {
        let i = self.alloc_one();
        self.store[i] = obj;
        return i;
    }

    fn alloc_one(&mut self) -> usize {
        if self.count == self.store.len() {
            self.resize(std::cmp::max(64, 2 * self.store.len()));
        }
        assert!(self.count < self.store.len());
        let i = self.free.iter().position(|&x| x).expect("No free elt");
        self.free[i] = false;
        self.count += 1;
        return i;
    }

    fn resize(&mut self, new_size: usize) {
        assert!(new_size > self.store.len());
        self.store.resize_with(new_size, Default::default);
        self.free.resize(new_size, true);
    }

    pub fn delete(&mut self, i: usize) {
        assert!(!self.free[i]);
        self.free[i] = true;
        self.count -= 1;
    }

    pub fn iter(&self) -> DynStoreIter<'_, T> {
        DynStoreIter {
            store: self,
            last_ind: None,
        }
    }
}

pub struct DynStoreIter<'a, T: Default> {
    store: &'a DynStore<T>,
    last_ind: Option<usize>,
}

impl<'a, T: Default> Iterator for DynStoreIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let next_ind: usize = match self.last_ind {
            None => 0,
            Some(i) => i + 1,
        };
        if next_ind == self.store.free.len() {
            None
        } else {
            let rest_free = &self.store.free[next_ind..];
            let opt_i = rest_free.iter().position(|&x| !x);
            match opt_i {
                Some(i) => {
                    self.last_ind = Some(i + next_ind);
                    Some(&self.store.store[i + next_ind])
                }
                None => None,
            }
        }
    }
}

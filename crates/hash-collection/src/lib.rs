#![feature(generic_associated_types)]
#![feature(in_band_lifetimes)]

use std::collections::hash_map::{Iter};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct HashCollection<T: Hash + Eq> {
    inner: HashMap<T, usize>
}

impl<T: Hash + Eq> HashCollection<T> {
    pub fn new() -> Self {
        HashCollection {
            inner: HashMap::new(),
        }
    }
}

impl<T: Hash + Eq> HashCollection<T> {
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains_key(value)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn insert(&mut self, value: T) {
        match self.inner.get(&value) {
            None => {
                self.inner.insert(value, 1);
            }
            Some(&i) => {
                self.inner.insert(value, i + 1);
            }
        }
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(i) = self.inner.get_mut(value) {
            if *i == 1 {
                self.inner.remove(value);
            } else {
                *i = *i - 1;
            }
        }
    }
}

impl<V: Hash + Eq, I: IntoIterator<Item = V>> From<I> for HashCollection<V> {
    fn from(iterator: I) -> Self {
        let mut hash_collection = HashCollection::new();
        for value in iterator {
            hash_collection.insert(value);
        }
        hash_collection
    }
}

struct HashCollectionIteratorState<'a, T: Hash + Eq> {
    key: &'a T,
    value: usize,
}

pub struct HashCollectionIterator<'a, T: Hash + Eq> {
    base_iterator: Iter<'a, T, usize>,
    state: Option<HashCollectionIteratorState<'a, T>>
}

impl<T: Hash + Eq> Iterator for HashCollectionIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            None => {
                match self.base_iterator.next() {
                    None => None,
                    Some((key, &value)) => {
                        self.state = Some(HashCollectionIteratorState {
                            key,
                            value: value - 1,
                        });
                        Some(key)
                    }
                }
            }
            Some(iterator_state) => {
                match iterator_state.value {
                    0 => {
                        match self.base_iterator.next() {
                            None => None,
                            Some((key, &value)) => {
                                self.state = Some(HashCollectionIteratorState {
                                    key,
                                    value: value - 1,
                                });
                                Some(key)
                            }
                        }
                    },
                    _ => {
                        iterator_state.value -= 1;
                        Some(iterator_state.key)
                    }
                }
            }
        }
    }
}

impl<T: Hash + Eq> IntoIterator for &'a HashCollection<T> {
    type Item = &'a T;
    type IntoIter = HashCollectionIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        HashCollectionIterator {
            base_iterator: self.inner.iter(),
            state: None,
        }
    }
}

impl<T: Hash + Eq> Hash for HashCollection<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in self {
            value.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::HashCollection;

    #[test]
    fn iter_through_values() {
        let hash_collection: HashCollection<i32> = HashCollection {
            inner: HashMap::from([
                (32, 3),
                (56, 1),
            ])
        };

        let mut sum = 0;
        for value in &hash_collection {
            sum += value;
        }
        assert_eq!(sum, 3 * 32 + 56);
        let hash_collection: HashCollection<i32> = HashCollection {
            inner: HashMap::from([
                (18, 2),
                (32, 3),
                (56, 1),
            ])
        };

        let mut sum = 0;
        for value in &hash_collection {
            sum += value;
        }
        assert_eq!(sum, 18 * 2 + 3 * 32 + 56);
    }

    #[test]
    fn remove() {
        let mut hash_collection: HashCollection<i32> = HashCollection {
            inner: HashMap::from([
                (32, 3),
                (56, 1),
            ])
        };

        hash_collection.remove(&32);
        assert_eq!(hash_collection.inner, HashMap::from([
            (32, 2),
            (56, 1),
        ]));


        hash_collection.remove(&56);
        assert_eq!(hash_collection.inner, HashMap::from([
            (32, 2),
        ]));


        hash_collection.remove(&3);
        assert_eq!(hash_collection.inner, HashMap::from([
            (32, 2),
        ]));
    }

    #[test]
    fn insert() {
        let mut hash_collection: HashCollection<i32> = HashCollection {
            inner: HashMap::from([
                (32, 3),
                (56, 1),
            ])
        };

        hash_collection.insert(18);
        assert_eq!(hash_collection.inner, HashMap::from([
            (32, 3),
            (56, 1),
            (18, 1),
        ]));


        hash_collection.insert(56);
        assert_eq!(hash_collection.inner, HashMap::from([
            (32, 3),
            (56, 2),
            (18, 1),
        ]));
    }

    #[test]
    fn contains() {
        let hash_collection: HashCollection<i32> = HashCollection {
            inner: HashMap::from([
                (32, 3),
                (56, 1),
            ])
        };

        assert_eq!(hash_collection.contains(&32), true);
        assert_eq!(hash_collection.contains(&56), true);
        assert_eq!(hash_collection.contains(&5), false);
    }

    #[test]
    fn from_iterator() {
        let vec = vec![32, 56, 18, 32];

        let collection = HashCollection {
            inner: HashMap::from([
                (32, 2),
                (56, 1),
                (18, 1),
            ])
        };
        assert_eq!(HashCollection::from(vec), collection);
    }
}

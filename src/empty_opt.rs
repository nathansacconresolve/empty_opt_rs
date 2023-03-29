use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList};

pub trait EmptyOpt<T> {
    fn empty_opt(self) -> Option<T>;
}

impl<T> EmptyOpt<Vec<T>> for Vec<T> {
    fn empty_opt(self) -> Option<Vec<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl EmptyOpt<String> for String {
    fn empty_opt(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<K, V> EmptyOpt<BTreeMap<K, V>> for BTreeMap<K, V> {
    fn empty_opt(self) -> Option<BTreeMap<K, V>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
impl<K, V> EmptyOpt<HashMap<K, V>> for HashMap<K, V> {
    fn empty_opt(self) -> Option<HashMap<K, V>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> EmptyOpt<BTreeSet<T>> for BTreeSet<T> {
    fn empty_opt(self) -> Option<BTreeSet<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> EmptyOpt<BinaryHeap<T>> for BinaryHeap<T> {
    fn empty_opt(self) -> Option<BinaryHeap<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> EmptyOpt<HashSet<T>> for HashSet<T> {
    fn empty_opt(self) -> Option<HashSet<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> EmptyOpt<LinkedList<T>> for LinkedList<T> {
    fn empty_opt(self) -> Option<LinkedList<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

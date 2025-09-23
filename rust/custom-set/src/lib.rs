#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>
where
    T: PartialEq + Clone + Ord,
{
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    vec: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: PartialEq + Clone + Ord,
{
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet { vec: Vec::new() };
        for item in input {
            set.add(item.clone());
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.vec.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.vec.push(element);
            self.vec.sort();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.intersection(other) == *self
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let vec: Vec<_> = self
            .vec
            .iter()
            .filter(|&el| other.contains(el))
            .cloned()
            .collect();

        CustomSet { vec }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let vec: Vec<_> = self
            .vec
            .iter()
            .filter(|&el| !other.contains(el))
            .cloned()
            .collect();

        CustomSet { vec }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut combined_set = self.clone();
        for element in &other.vec {
            combined_set.add(element.clone());
        }
        combined_set
    }
}

impl<T> Clone for CustomSet<T>
where
    T: PartialEq + Clone + Ord,
{
    fn clone(&self) -> Self {
        CustomSet {
            vec: self.vec.clone(),
        }
    }
}

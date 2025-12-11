use std::collections::HashMap;
use std::hash::Hash;

/// Simple injective bidirectional map between A and B using two hashmaps.
pub struct BiMap<A, B> {
    a_to_b: HashMap<A, B>,
    b_to_a: HashMap<B, A>,
}

impl<A, B> BiMap<A, B>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            a_to_b: HashMap::new(),
            b_to_a: HashMap::new(),
        }
    }

    /// Insert mapping A<->B. Returns Err if either side is already mapped to a different value.
    pub fn insert(&mut self, a: A, b: B) -> Result<(), &'static str> {
        if let Some(existing_b) = self.a_to_b.get(&a) {
            if existing_b != &b {
                return Err("A already mapped to a different B");
            }
            return Ok(());
        }
        if let Some(existing_a) = self.b_to_a.get(&b) {
            if existing_a != &a {
                return Err("B already mapped to a different A");
            }
            return Ok(());
        }
        self.a_to_b.insert(a.clone(), b.clone());
        self.b_to_a.insert(b, a);
        Ok(())
    }

    pub fn get_by_a(&self, a: &A) -> Option<&B> {
        self.a_to_b.get(a)
    }

    pub fn get_by_b(&self, b: &B) -> Option<&A> {
        self.b_to_a.get(b)
    }

    pub fn contains_a(&self, a: &A) -> bool {
        self.a_to_b.contains_key(a)
    }

    pub fn contains_b(&self, b: &B) -> bool {
        self.b_to_a.contains_key(b)
    }

    pub fn len(&self) -> usize {
        self.a_to_b.len()
    }

    pub fn is_empty(&self) -> bool {
        self.a_to_b.is_empty()
    }

    pub fn clear(&mut self) {
        self.a_to_b.clear();
        self.b_to_a.clear();
    }
}

/// An alternative to a hash map which is backed by a static array. Good for
/// small inputs.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct StaticMap<K, V>(Vec<(K, V)>);

impl<K, V> StaticMap<K, V>
where
    K: PartialEq,
{
    pub const fn new() -> Self {
        StaticMap(Vec::new())
    }

    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        if let Some(v) = self.get_mut(&key) {
            std::mem::swap(v, &mut value);
            Some(value)
        } else {
            self.0.push((key, value));
            None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }
}

impl<K, V> IntoIterator for StaticMap<K, V> {
    type Item = (K, V);

    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

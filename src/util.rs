use std::mem;

/// An iterator over an owned set of chars.
///
/// # Safty
/// I believe this struct to be safe because the String is
/// heap-allocated (stable address) and will never be modified
/// (stable address). `chars` will not outlive the struct, so
/// lying about the lifetime should be fine.
///
/// `Chars` doesn't have a destructor so it is safe to Drop.
///
/// source: https://stackoverflow.com/a/47207490
pub(crate) struct OwnedCharBuffer {
    _s: String,
    index: usize,
    next: Option<char>,
    /// the chars here are borrowed from the above string
    chars: std::str::Chars<'static>,
}

impl OwnedCharBuffer {
    pub fn new(s: String) -> Self {
        // safety: chars will not escape the scope of this object
        let chars = unsafe { mem::transmute(s.chars()) };
        Self {
            _s: s,
            index: 0,
            next: None,
            chars,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get_slice(&self, str: usize, end: usize) -> Option<&str> {
        debug_assert!(str <= end);
        if end > self._s.len() {
            None
        } else {
            Some(&self._s[str..end])
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(n) = self.next.as_ref() {
            Some(*n)
        } else {
            self.next = self.chars.next();
            self.next
        }
    }
}

impl Iterator for OwnedCharBuffer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().or_else(|| {
            self.index += 1;
            self.chars.next()
        })
    }
}

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

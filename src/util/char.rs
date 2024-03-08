use std::mem;
use std::str::Chars;

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
pub(crate) struct OwnedChars {
    _s: String,
    chars: Chars<'static>,
}

impl OwnedChars {
    pub fn new(s: String) -> Self {
        // safety: chars will escape the scope of this object
        let chars = unsafe { mem::transmute(s.chars()) };
        OwnedChars { _s: s, chars }
    }
}

impl Iterator for OwnedChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}

use streaming_iterator::StreamingIterator;

use crate::{internal::MatchIterator, package::Package};


pub struct Iter(MatchIterator);

impl Iterator for Iter {
    type Item = Package;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|h| h.to_package())
    }
}


pub enum Index {
    Name,
}

impl Index {
    pub fn find<S: AsRef<str>>(self, key: S) -> Iter {
        Iter(MatchIterator::new(self.into(), Some(key.as_ref())))
    }
}
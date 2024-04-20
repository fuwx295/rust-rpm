use crate::internal::iterator::MatchIterator;
use crate::internal::tag::DBIndexTag;
use crate::package::Package;
use streaming_iterator::StreamingIterator;

/// Iterator over the RPM database which returns `Package` structs.
pub struct Iter(MatchIterator);

impl Iterator for Iter {
    type Item = Package;

    /// Obtain the next header from the iterator.
    fn next(&mut self) -> Option<Package> {
        self.0.next().map(|h| h.to_package())
    }
}

/// Searchable fields in the RPM package headers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Index {
    /// Search by package name.
    Name,
}

impl Index {
    /// Find an exact match in the given index
    pub fn find<S: AsRef<str>>(self, key: S) -> Iter {
        Iter(MatchIterator::new(self.into(), Some(key.as_ref())))
    }
}

/// Find all packages installed on the local system.
pub fn installed_packages() -> Iter {
    Iter(MatchIterator::new(DBIndexTag::PACKAGES, None))
}

pub fn find_package<S: AsRef<str>>(key: S) -> Iter {
    Iter(MatchIterator::new(DBIndexTag::NAME, Some(key.as_ref())))
}

/// Find installed packages with a search key that exactly matches the given tag.
///
/// Panics if the glob contains null bytes.
pub fn find<S: AsRef<str>>(index: Index, key: S) -> Iter {
    index.find(key)
}
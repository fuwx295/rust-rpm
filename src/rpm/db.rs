use crate::rpm::internal::iterator::MatchIterator;
use super::internal::tag::DBIndexTag;
use super::package::Package;
use streaming_iterator::StreamingIterator;

/// Iterator over the RPM database which returns `Package` structs.
pub struct Iter {
    pub match_it: MatchIterator,
    pub mode: char,
}

impl Iterator for Iter {
    type Item = Package;

    /// Obtain the next header from the iterator.
    fn next(&mut self) -> Option<Package> {
        self.match_it.next().map(|h|h.to_package(self.mode))
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
        Iter {
            match_it: MatchIterator::new(self.into(), Some(key.as_ref())),
            mode: 'i',
        }
    }
}

/// Find all packages installed on the local system.
pub fn installed_packages(mode: char) -> Iter {
    Iter {
       match_it: MatchIterator::new(DBIndexTag::PACKAGES, None),
       mode,
    }
}

pub fn find_package<S: AsRef<str>>(key: S, mode: char) -> Iter {
    Iter {
        match_it: MatchIterator::new(DBIndexTag::NAME, Some(key.as_ref())),
        mode,
    }
}

/// Find installed packages with a search key that exactly matches the given tag.
///
/// Panics if the glob contains null bytes.
pub fn find<S: AsRef<str>>(index: Index, key: S) -> Iter {
    index.find(key)
}
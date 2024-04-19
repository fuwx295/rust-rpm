use std::{os::raw::c_void, ptr};

use super::{header::Header, ts::GlobalTS, tag::DBIndexTag};

use streaming_iterator::StreamingIterator;


pub struct MatchIterator {
    ptr: *mut librpm::rpmdbMatchIterator_s,
    txn: GlobalTS,
    next: Option<Header>,
    finished: bool,
}

impl MatchIterator {
    pub fn new(tag: DBIndexTag, key_opt: Option<&str>) -> Self {
        let mut txn = GlobalTS::create();
        let next = None;
        let finished = false;
    
        if let Some(key) = key_opt {
            if !key.is_empty() {
                let ptr = unsafe {
                    librpm::rpmtsInitIterator(
                        txn.as_mut_ptr(),
                        tag as librpm::rpm_tag_t,
                        key.as_ptr() as * const c_void,
                        key.len(),
                    )
                };
                return Self {
                    ptr,
                    txn,
                    next,
                    finished,
                };
            }
        }

        let ptr = unsafe {
            librpm::rpmtsInitIterator(
                txn.as_mut_ptr(),
                tag as librpm::rpm_tag_t,
                ptr::null(),
                0,
            )
        };

        Self {
            ptr,
            txn,
            next,
            finished,
        }
    }

}

impl StreamingIterator for MatchIterator {
    type Item = Header;

    fn advance(&mut self) {
        if self.finished {
            return;
        }
        let header_ptr = unsafe {
            librpm::rpmdbNextIterator(self.ptr)
        };
        if header_ptr.is_null() {
            self.finished = true;
            self.next = None;
        } else {
            self.next = Some(unsafe {
                Header::from_ptr(header_ptr)
            })
        }
    }

    fn get(&self) -> Option<&Self::Item> {
        self.next.as_ref()
    }
}

impl Drop for MatchIterator {
    fn drop(&mut self) {
        unsafe {
            librpm::rpmdbFreeIterator(self.ptr);
        }
    }
    
}


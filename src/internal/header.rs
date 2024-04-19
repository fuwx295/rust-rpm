use std::mem;

use super::{tag::Tag, td::TagData};

pub struct Header(*mut librpm::headerToken_s);

impl Header {
    pub unsafe fn from_ptr(ffi_header: librpm::Header) -> Self {
        assert!(!ffi_header.is_null());
        librpm::headerLink(ffi_header);
        Header(ffi_header)
    }
    pub fn get(&self, tag: Tag) -> Option<TagData<'_>> {
        let mut td: librpm::rpmtd_s = unsafe {
            mem::zeroed()
        };
        unsafe {
            librpm::rpmtdReset(&mut td);
        }
        let rc = unsafe {
           librpm::headerGet(
            self.0,
            tag as i32,
            &mut td,
            librpm::headerGetFlags_e_HEADERGET_MINMEM,
           ) 
        };
    }

}

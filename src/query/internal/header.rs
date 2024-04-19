pub struct Header(*mut librpm::headerTokens_s);

impl Header {
    pub unsafe fn from_ptr(ffi_header: librpm::Header) -> Self {
        assert!(!ffi_header.is_null());
        librpm::headerLink(ffi_header);
        Header(ffi_header)
    }
}

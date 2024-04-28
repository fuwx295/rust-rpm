use super::{tag::Tag, td::TagData};
use crate::rpm::Package;
use std::mem;

/// RPM package header
pub struct Header(*mut librpm_sys::headerToken_s);

impl Header {
    pub unsafe fn from_ptr(ffi_header: librpm_sys::Header) -> Self {
        assert!(!ffi_header.is_null());
        // Increment librpm's internal reference count for this header
        librpm_sys::headerLink(ffi_header);
        Header(ffi_header)
    }

    /// Get the data that corresponds to the given header tag.
    pub fn get(&self, tag: Tag) -> Option<TagData<'_>> {
        // Create a zeroed `rpmtd_s` and then immediately initialize it
        let mut td: librpm_sys::rpmtd_s = unsafe { mem::zeroed() };
        unsafe {
            librpm_sys::rpmtdReset(&mut td);
        }

        let rc = unsafe {
            librpm_sys::headerGet(
                self.0,
                tag as i32,
                &mut td,
                librpm_sys::headerGetFlags_e_HEADERGET_MINMEM,
            )
        };

        if rc == 0 {
            return None;
        }

        let data = match td.type_ {
            librpm_sys::rpmTagType_e_RPM_NULL_TYPE => TagData::Null,
            librpm_sys::rpmTagType_e_RPM_CHAR_TYPE => unsafe { TagData::char(&td) },
            librpm_sys::rpmTagType_e_RPM_INT8_TYPE => unsafe { TagData::int8(&td) },
            librpm_sys::rpmTagType_e_RPM_INT16_TYPE => unsafe { TagData::int16(&td) },
            librpm_sys::rpmTagType_e_RPM_INT32_TYPE => unsafe { TagData::int32(&td) },
            librpm_sys::rpmTagType_e_RPM_INT64_TYPE => unsafe { TagData::int64(&td) },
            librpm_sys::rpmTagType_e_RPM_STRING_TYPE => unsafe { TagData::string(&td) },
            librpm_sys::rpmTagType_e_RPM_STRING_ARRAY_TYPE => unsafe { TagData::string_array(&td) },
            librpm_sys::rpmTagType_e_RPM_I18NSTRING_TYPE => unsafe { TagData::i18n_string(&td) },
            librpm_sys::rpmTagType_e_RPM_BIN_TYPE => unsafe { TagData::bin(&td) },
            other => panic!("unsupported rpmtd tag type: {}", other),
        };

        Some(data)
    }

    /// Convert this `Header` into a `Package`
    pub fn to_package(&self, mode: char) -> Package {
        let mut pkg = Package::default();

        match mode {
            'b' | 'i' => {
                pkg.name = self.get(Tag::NAME).unwrap().as_str().unwrap().to_string();
                pkg.epoch = self
                    .get(Tag::EPOCH)
                    .map(|d| d.to_int32().unwrap().to_owned());
                pkg.version = self.get(Tag::VERSION).unwrap().as_str().unwrap().to_owned();
                pkg.release = self.get(Tag::RELEASE).unwrap().as_str().unwrap().to_owned();
                pkg.arch = self.get(Tag::ARCH).map(|d| d.as_str().unwrap().to_owned());
                if mode == 'i' {
                    pkg.installtime = self.get(Tag::INSTALLTIME).unwrap().to_int32().unwrap();
                    pkg.group = self.get(Tag::GROUP).unwrap().as_str().unwrap().into();
                    pkg.size = self.get(Tag::SIZE).unwrap().to_int32().unwrap() as i64;
                    pkg.license = self.get(Tag::LICENSE).unwrap().as_str().unwrap().to_owned();
                    pkg.signature = None;
                    pkg.sourcerpm = self.get(Tag::SOURCERPM).unwrap().as_str().unwrap().into();
                    pkg.buildtime = self.get(Tag::BUILDTIME).unwrap().to_int32().unwrap();
                    pkg.buildhost = self.get(Tag::BUILDHOST).unwrap().as_str().unwrap().into();
                    pkg.relocations = None;
                    pkg.packager = self
                        .get(Tag::PACKAGER)
                        .map(|d| d.as_str().unwrap().to_owned());
                    pkg.vendor = self
                        .get(Tag::VENDOR)
                        .map(|d| d.as_str().unwrap().to_owned());
                    pkg.url = self.get(Tag::URL).map(|d| d.as_str().unwrap().to_owned());
                    pkg.summary = self.get(Tag::SUMMARY).unwrap().as_str().unwrap().into();
                    pkg.description = self.get(Tag::DESCRIPTION).unwrap().as_str().unwrap().into();
                }
            }
            'c' => {}
            'r' => {}
            _ => {}
        }

        // Package {
        //     name: self.get(Tag::NAME).unwrap().as_str().unwrap().to_owned(),
        //     epoch: self
        //         .get(Tag::EPOCH)
        //         .map(|d| d.to_int32().unwrap().to_owned()),
        //     version: self.get(Tag::VERSION).unwrap().as_str().unwrap().to_owned(),
        //     release: self.get(Tag::RELEASE).unwrap().as_str().unwrap().to_owned(),
        //     arch: self.get(Tag::ARCH).map(|d| d.as_str().unwrap().to_owned()),
        //     installtime: self.get(Tag::INSTALLTIME).unwrap().to_int32().unwrap(),
        //     group: self.get(Tag::GROUP).unwrap().as_str().unwrap().into(),
        //     size: self.get(Tag::SIZE).unwrap().to_int32().unwrap() as i64,
        //     license: self.get(Tag::LICENSE).unwrap().as_str().unwrap().to_owned(),
        //     signature: self
        //         .get(Tag::DSAHEADER)
        //         .map(|d| d.as_str().unwrap().to_owned()),
        //     sourcerpm: self.get(Tag::SOURCERPM).unwrap().as_str().unwrap().into(),
        //     summary: self.get(Tag::SUMMARY).unwrap().as_str().unwrap().into(),
        //     description: self.get(Tag::DESCRIPTION).unwrap().as_str().unwrap().into(),
        //     buildtime: self.get(Tag::BUILDTIME).unwrap().to_int32().unwrap(),
        //     buildhost: self.get(Tag::BUILDHOST).unwrap().as_str().unwrap().into(),
        //     // relocations: self
        //     //     .get(Tag::PREFIXES)
        //     //     .map(|d| d.as_str_array().unwrap().to_owned()),
        //     relocations: None,
        //     packager: self
        //         .get(Tag::PACKAGER)
        //         .map(|d| d.as_str().unwrap().to_owned()),
        //     vendor: self
        //         .get(Tag::VENDOR)
        //         .map(|d| d.as_str().unwrap().to_owned()),
        //     url: self.get(Tag::URL).map(|d| d.as_str().unwrap().to_owned()),
        //     bugurl: self
        //         .get(Tag::BUGURL)
        //         .map(|d| d.as_str().unwrap().to_owned()),
        //     requirenevrs: self
        //         .get(Tag::CHANGELOGTEXT)
        //         .map(|d| d.as_str_array().unwrap().to_owned()),
        // }
        pkg
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        // Decrement librpm's internal reference count for this header
        unsafe {
            librpm_sys::headerFree(self.0);
        }
    }
}

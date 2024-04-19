
use super::tag::TagType;
use librpm::rpmtd_s;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::{slice, str};

pub enum TagData<'a> {
    Null,
    Char(char),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Str(&'a str),
    StrArray(Vec<&'a str>),
    I18NStr(&'a str),
    Bin(&'a [u8]),
}

impl<'a> TagData<'a> {
    pub unsafe fn char(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::CHAR as u32);
        let ix = if td.ix >= 0 { td.ix as isize } else { 0 };
        TagData::Char(*(td.data as *const char).offset(ix))
    }

    pub unsafe fn int8(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT8 as u32);
        let ix = if td.ix >= 0 { td.ix as isize } else { 0 };
        TagData::Int8(*(td.data as *const i8).offset(ix))
    }

    pub unsafe fn int16(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT16 as u32);
        let ix = if td.ix >= 0 { td.ix as isize } else { 0 };
        TagData::Int16(*(td.data as *const i16).offset(ix))
    }

    pub unsafe fn int32(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT32 as u32);
        let ix = if td.ix >= 0 { td.ix as isize } else { 0 };
        TagData::Int32(*(td.data as *const i32).offset(ix))
    }

    pub unsafe fn int64(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::INT64 as u32);
        let ix = if td.ix >= 0 { td.ix as isize } else { 0 };
        TagData::Int64(*(td.data as *const i64).offset(ix))
    }

    pub unsafe fn string(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::STRING as u32);
        let cstr = CStr::from_ptr(td.data as *const c_char);
        TagData::Str(str::from_utf8(cstr.to_bytes()).unwrap_or_else(|e| {
            panic!(
                "failed to decode RPM_STRING_TYPE as UTF-8 (tag: {}): {}",
                td.tag, e
            );
        }))
    }

    pub unsafe fn string_array(_td: &rpmtd_s) -> Self {
        panic!("RPM_STRING_ARRAY_TYPE unsupported!");
    }

    pub unsafe fn i18n_string(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::I18NSTRING as u32);
        let cstr = CStr::from_ptr(td.data as *const c_char);

        TagData::I18NStr(str::from_utf8(cstr.to_bytes()).unwrap_or_else(|e| {
            panic!(
                "failed to decode RPM_I18NSTRING_TYPE as UTF-8 (tag: {}): {}",
                td.tag, e
            );
        }))
    }

    pub unsafe fn bin(td: &rpmtd_s) -> Self {
        assert_eq!(td.type_, TagType::BIN as u32);

        assert!(
            !td.data.is_null(),
            "rpmtd.data is NULL! (tag type: {})",
            td.tag
        );

        assert_ne!(
            td.type_,
            TagType::NULL as u32,
            "can't get slice of NULL data (tag type: {})",
            td.tag
        );

        TagData::Bin(slice::from_raw_parts(
            td.data as *const u8,
            td.count as usize,
        ))
    }

    pub fn is_null(&self) -> bool {
        matches!(*self, TagData::Null)
    }

    pub fn to_char(&self) -> Option<char> {
        match *self {
            TagData::Char(c) => Some(c),
            _ => None,
        }
    }

    pub fn is_char(&self) -> bool {
        self.to_char().is_some()
    }

    pub fn to_int8(&self) -> Option<i8> {
        match *self {
            TagData::Int8(i) => Some(i),
            _ => None,
        }
    }

    pub fn is_int8(&self) -> bool {
        self.to_int8().is_some()
    }

    pub fn to_int16(&self) -> Option<i16> {
        match *self {
            TagData::Int16(i) => Some(i),
            _ => None,
        }
    }

    pub fn is_int16(&self) -> bool {
        self.to_int16().is_some()
    }

    pub fn to_int32(&self) -> Option<i32> {
        match *self {
            TagData::Int32(i) => Some(i),
            _ => None,
        }
    }

    pub fn is_int32(&self) -> bool {
        self.to_int32().is_some()
    }

    pub fn to_int64(&self) -> Option<i64> {
        match *self {
            TagData::Int64(i) => Some(i),
            _ => None,
        }
    }

    pub fn is_int64(&self) -> bool {
        self.to_int64().is_some()
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match *self {
            TagData::Str(s) => Some(s),
            TagData::I18NStr(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_str_array(&self) -> Option<&[&'a str]> {
        match *self {
            TagData::StrArray(ref sa) => Some(&sa[..]),
            _ => None,
        }
    }

    pub fn is_str_array(&self) -> bool {
        self.as_str_array().is_some()
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match *self {
            TagData::Bin(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_bytes(&self) -> bool {
        self.as_bytes().is_some()
    }
}
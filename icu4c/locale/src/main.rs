use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;

use intl_harness::locale::HarnessLocaleRuntime;

#[link(name = "icuuc")]
extern "C" {
    pub fn uloc_getLanguage_66(localeID: *const c_char, language: *mut c_char, languageCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    pub fn uloc_getScript_66(localeID: *const c_char, script: *mut c_char, scriptCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    pub fn uloc_getCountry_66(localeID: *const c_char, region: *mut c_char, regionCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    pub fn uloc_getVariant_66(localeID: *const c_char, variant: *mut c_char, variantCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
    pub fn uloc_canonicalize_66(localeID: *const c_char, name: *mut c_char, nameCapacity: i32, UErrorCode: *mut libc::c_int) -> i32;
}

#[derive(Debug, PartialEq)]
pub struct LanguageIdentifier {
    language: String,
    region: String,
    script: String,
    variant: String,
}


impl FromStr for LanguageIdentifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = CString::new(s).unwrap();
        let mut err = 0;
        let capacity = 8;

        let language = {
            let output = CString::default();
            let ptr = output.into_raw();
            unsafe {
                uloc_getLanguage_66(input.as_ptr(), ptr, capacity, &mut err);
            };
            unsafe { CString::from_raw(ptr) }.into_string().unwrap()
        };
        let region = {
            let output = CString::default();
            let ptr = output.into_raw();
            unsafe {
                uloc_getCountry_66(input.as_ptr(), ptr, capacity, &mut err);
            };
            unsafe { CString::from_raw(ptr) }.into_string().unwrap()
        };
        let script = {
            let output = CString::default();
            let ptr = output.into_raw();
            unsafe {
                uloc_getScript_66(input.as_ptr(), ptr, capacity, &mut err);
            };
            unsafe { CString::from_raw(ptr) }.into_string().unwrap()
        };
        let variant = {
            let output = CString::default();
            let ptr = output.into_raw();
            unsafe {
                uloc_getVariant_66(input.as_ptr(), ptr, capacity, &mut err);
            };
            unsafe { CString::from_raw(ptr) }.into_string().unwrap()
        };
        Ok(LanguageIdentifier {
            language,
            region,
            script,
            variant,
        })
    }
}

impl ToString for LanguageIdentifier {
    fn to_string(&self) -> String {
        let parts = &[self.language.as_str()];

        let p: Vec<_> = parts.iter().filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(*s)
            }
        }).collect();
        p.join("-")
    }
}


pub struct IcuLocale;

impl HarnessLocaleRuntime for IcuLocale {
    type Locale = LanguageIdentifier;

    fn canonicalize(input: &str) -> String {
        let input = CString::new(input).unwrap();
        let mut err = 0;
        let capacity = 30;

        let output = CString::default();
        let ptr = output.into_raw();
        unsafe {
            uloc_canonicalize_66(input.as_ptr(), ptr, capacity, &mut err);
        };
        unsafe { CString::from_raw(ptr) }.into_string().unwrap()
    }
}

fn main() {
    IcuLocale::run("../../harness/data");
}

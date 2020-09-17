use std::ffi::CString;
use std::os::raw::c_char;
use std::str::FromStr;

use intl_harness::plurals::HarnessPluralsRuntime;

#[link(name = "icui18n")]
extern "C" {
    pub fn uplrules_open_66(locale: *const c_char, UErrorCode: *mut libc::c_int) -> *mut libc::c_void;
    pub fn uplrules_close_66(uplrules: *mut libc::c_void);
    pub fn uplrules_select_66(uplrules: *mut libc::c_void, number: libc::c_double, keyword: *mut u16, capacity: libc::c_int, UErrorCode: *mut libc::c_int) -> libc::c_int;

    // U_CAPI int32_t U_EXPORT2
// uplrules_select(const UPluralRules *uplrules,
    //            double number,
    //            UChar *keyword, int32_t capacity,
    //            UErrorCode *status);
}

pub struct IcuPluralRules {
    langids: Vec<String>,
}

impl IcuPluralRules {
    fn new() -> Self {
        Self {
            langids: vec![],
        }
    }
}

impl HarnessPluralsRuntime for IcuPluralRules {
    fn prepare(&mut self, langids: &[String]) {
        for langid in langids {
            self.langids.push(langid.clone());
        }
    }

    fn select(&self, values: &[isize]) -> Vec<String> {
        for langid in &self.langids {
            let lid = CString::new(langid.as_str()).unwrap();
            let mut err = 0;
            let pr = unsafe { uplrules_open_66(lid.as_ptr(), &mut err) };

            for value in values {
                unsafe {
                    // let capacity = uplrules_select_66(pr, *value as f64, std::ptr::null_mut(), 0, &mut err);
                    // // check error.
                    let mut storage = vec![0u16; 255]; // May need +1? Not sure if icu null-terminates
                    let new_capacity = uplrules_select_66(pr, *value as f64, storage.as_mut_ptr(), 255, &mut err);
                    // // Probably assert that there's no error, and that new_capacity == capacity
                    // let r = String::from_utf16(&storage[0..new_capacity as usize]).unwrap();
                    // println!("{:#?}", r);
                }
                //     ;
                // unsafe {
                //     let mut keyword: [i8; 255] = [0; 255];
                //     let capacity = 255;
                //     let _ = uplrules_select_66(pr, *value as f64, keyword.as_mut_ptr(), capacity, &mut err);
                //     let r = std::ffi::CStr::from_ptr(keyword.as_ptr()).to_string_lossy();
                //     println!("{:#?}", r);
                // }
            }
            unsafe { uplrules_close_66(pr) };
        }
        vec![]
    }
}

fn main() {
    let mut runner = IcuPluralRules::new();
    runner.run("../../harness/data");
}

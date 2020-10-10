use std::ffi::CStr;
use std::os::raw::c_char;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use super::super::foundation::{id, NSInteger, BOOL, NO, YES};

/// Wrapper for a retained `NSNumber` object.
///
/// In general we strive to avoid using this in the codebase, but it's a requirement for moving
/// objects in and out of certain situations (e.g, `UserDefaults`).
#[derive(Debug)]
pub struct NSNumber(pub Id<Object>);

impl NSNumber {
    /// If we're vended an NSNumber from a method (e.g, `NSUserDefaults` querying) we might want to
    /// wrap it while we figure out what to do with it. This does that.
    pub fn wrap(data: id) -> Self {
        NSNumber(unsafe { Id::from_ptr(data) })
    }

    /// Constructs a `numberWithBool` instance of `NSNumber` and retains it.
    pub fn bool(value: bool) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(msg_send![class!(NSNumber), numberWithBool:match value {
                true => YES,
                false => NO
            }])
        })
    }

    /// Constructs a `numberWithInteger` instance of `NSNumber` and retains it.
    pub fn integer(value: i64) -> Self {
        NSNumber(unsafe {
            Id::from_ptr(msg_send![
                class!(NSNumber),
                numberWithInteger: value as NSInteger
            ])
        })
    }

    /// Constructs a `numberWithDouble` instance of `NSNumber` and retains it.
    pub fn float(value: f64) -> Self {
        NSNumber(unsafe { Id::from_ptr(msg_send![class!(NSNumber), numberWithDouble: value]) })
    }

    /// Returns the `objCType` of the underlying `NSNumber` as a Rust `&str`. This flag can be used
    /// to inform you how you should pull the underlying data out of the `NSNumber`.
    ///
    /// For more information:
    /// [https://nshipster.com/type-encodings/](https://nshipster.com/type-encodings/)
    pub fn objc_type(&self) -> &str {
        unsafe {
            let t: *const c_char = msg_send![&*self.0, objCType];
            let slice = CStr::from_ptr(t);
            slice.to_str().unwrap()
        }
    }

    /// Pulls the underlying `NSInteger` value out and passes it back as an `i64`.
    ///
    /// Note that this _does not check_ if the underlying type is actually this. You are
    /// responsible for doing so via the `objc_type()` method.
    pub fn as_i64(&self) -> i64 {
        unsafe {
            let i: NSInteger = msg_send![&*self.0, integerValue];
            i as i64
        }
    }

    /// Pulls the underlying `double` value out and passes it back as an `f64`.
    ///
    /// Note that this _does not check_ if the underlying type is actually this. You are
    /// responsible for doing so via the `objc_type()` method.
    pub fn as_f64(&self) -> f64 {
        unsafe { msg_send![&*self.0, doubleValue] }
    }

    /// Pulls the underlying `BOOL` value out and passes it back as a `bool`.
    ///
    /// Note that this _does not check_ if the underlying type is actually this. You are
    /// responsible for doing so via the `objc_type()` method.
    pub fn as_bool(&self) -> bool {
        let result: BOOL = unsafe { msg_send![&*self.0, boolValue] };

        match result {
            YES => true,
            NO => false,
            _ => unreachable!(),
        }
    }

    /// A helper method for determining if a given `NSObject` is an `NSNumber`.
    pub fn is(obj: id) -> bool {
        let result: BOOL = unsafe { msg_send![obj, isKindOfClass: class!(NSNumber)] };

        match result {
            YES => true,
            NO => false,
            _ => unreachable!(),
        }
    }
    /// Consumes and returns the underlying `NSNumber`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}

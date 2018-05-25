extern crate uuid;

#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate uuid_macro_impl;

#[doc(hidden)]
#[allow(unused_imports)]
use uuid_macro_impl::*;

proc_macro_expr_decl! {
    #[doc(hidden)]
    uuid_parts! => uuid_parts_impl
}

/// A literal syntax for generating UUID values.
///
/// ```
/// # extern crate uuid;
/// # #[macro_use]
/// # extern crate uuid_macro;
/// # use uuid::Uuid;
/// # fn main() {
/// #    let _ : Uuid = uuid!("6B29FC40-CA47-1067-B31D-00DD010662DA");
/// # }
/// ```
#[macro_export]
macro_rules! uuid {
    {$literal:expr} => {
        {
            use uuid;
            const BYTES: [u8; 16] = uuid_parts! {$literal};
            uuid::Uuid::from_uuid_bytes(BYTES)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::uuid::Uuid;
    #[cfg(feature = "nightly")]
    const CONST_UUID: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");

    #[cfg(feature = "nightly")]
    #[test]
    fn test_const_uuid_macro_build(){
        assert_eq!("f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4", format!("{:x}", CONST_UUID));
    }

    #[test]
    fn test_uuid_macro_build(){
        let u: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");
        assert_eq!("f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4", format!("{:x}", u));
    }
}

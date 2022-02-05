pub type FourCharCode = u32;
pub type ResType = FourCharCode; // ??

pub mod os {
    use super::FourCharCode;

    pub type Err = i16;
    pub type Status = i32;
    pub type Type = FourCharCode;

    pub const NO_ERR: Status = 0;
}

pub mod cf;
pub mod cg;
pub mod cm;
pub mod cv;
pub mod io;
pub mod vt;

pub fn test_asm() {
    unsafe {
        let null = cf::Null::value();
        let num = cf::Number::from_i16(0);
        let arr = cf::Array::from_type_refs(&[null, &num]).unwrap_unchecked();

        arr.show();
    }
}

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn it_works() {
        let f = {
            let null = cf::Null::value();
            null.show();
            let num = cf::Number::from_i16(0);
            let arr = cf::Array::from_type_refs(&[null, &num]).unwrap();

            let v = b"he".to_vec();
            let s = cf::String::create_with_bytes_no_copy(
                None,
                &v,
                2,
                cf::StringEncoding::UTF8,
                false,
                Some(cf::Allocator::null()),
            )
            .unwrap();
            // s
            let f = num;

            arr.show();
            // s.show();

            // println!("len {:?}", f.get_length());
            // f.show();
            arr
            // s.retained()
        };

        f.show()
    }
}

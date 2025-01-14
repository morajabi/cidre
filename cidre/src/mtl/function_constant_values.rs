use std::ffi::c_void;

use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(pub FnConstValues(ns::Id), MTL_FUNCTION_CONSTANT_VALUES);

impl FnConstValues {
    define_mtl!(reset);

    #[objc::msg_send(setConstantValue:type:atIndex:)]
    pub fn set_value_at(&mut self, val: *const c_void, type_: mtl::DType, at_index: ns::UInteger);

    #[objc::msg_send(setConstantValues:type:withRange:)]
    pub fn set_values(&mut self, values: *const c_void, type_: mtl::DType, with_range: ns::Range);

    #[objc::msg_send(setConstantValue:type:withName:)]
    pub fn set_value_with_name(&mut self, val: *const c_void, type_: mtl::DType, name: &ns::String);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_FUNCTION_CONSTANT_VALUES: &'static objc::Class<FnConstValues>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};

    #[test]
    fn basics() {
        let mut fcv = mtl::FnConstValues::new();
        let v = false;
        fcv.set_value_at(&v as *const bool as _, mtl::DType::Bool, 0);
        fcv.reset();
        let name = ns::String::with_str("name");
        fcv.set_value_with_name(&v as *const bool as _, mtl::DType::Bool, &name);
    }
}

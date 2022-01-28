use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@kbn/config-schema")]
extern "C" {
    pub type Type;

    #[wasm_bindgen(js_namespace = schema)]
    fn any() -> Type;
}

pub trait SchemaBlock: Into<Type> + private::Sealed {}

pub struct Schema;
impl Schema {
    pub fn any() -> impl SchemaBlock {
        AnySchemaBlock {}
    }
}

pub struct AnySchemaBlock;
impl SchemaBlock for AnySchemaBlock {}
impl Into<Type> for AnySchemaBlock {
    fn into(self) -> Type {
        any()
    }
}

/// The `SchemaBlock` trait is not supposed to be implemented outside of this crate, so we seal it.
mod private {
    pub trait Sealed {}
    impl Sealed for super::AnySchemaBlock {}
}

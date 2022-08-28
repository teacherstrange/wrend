use std::ops::{Deref, DerefMut};

use js_sys::{Array, Object};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{utils, AttributeLink};

pub type AttributeLinkJsInner = AttributeLink<String, String, String, Object>;

#[wasm_bindgen(js_name = AttributeLink)]
pub struct AttributeLinkJs(AttributeLinkJsInner);

#[wasm_bindgen(js_class = AttributeLink)]
impl AttributeLinkJs {
    pub fn vao_ids(&self) -> Array {
        let ids = self.deref().vao_ids();
        utils::strings_to_js_array(ids)
    }

    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    pub fn attribute_id(&self) -> String {
        self.deref().attribute_id().to_owned()
    }
}

impl AttributeLinkJs {
    pub fn inner(self) -> AttributeLinkJsInner {
        self.0
    }
}

impl Deref for AttributeLinkJs {
    type Target = AttributeLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributeLinkJs {
    fn deref_mut(&mut self) -> &mut AttributeLinkJsInner {
        &mut self.0
    }
}

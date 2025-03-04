use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{BufferCreateCallbackJs, BufferLink};

pub type BufferLinkJsInner = BufferLink<String>;

#[wasm_bindgen(inspectable, js_name = BufferLink)]
pub struct BufferLinkJs(BufferLinkJsInner);

#[wasm_bindgen(js_class = BufferLink)]
impl BufferLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer_id: String, buffer_create_callback: BufferCreateCallbackJs) -> Self {
        Self(BufferLinkJsInner::new(buffer_id, buffer_create_callback))
    }

    #[wasm_bindgen(js_name = bufferId)]
    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    #[wasm_bindgen(js_name = createBuffer)]
    pub fn create_buffer(&self, gl: WebGl2RenderingContext, now: f64) -> WebGlBuffer {
        self.deref().create_buffer(gl, now)
    }
}

impl BufferLinkJs {
    pub fn into_inner(self) -> BufferLinkJsInner {
        self.0
    }
}

impl Deref for BufferLinkJs {
    type Target = BufferLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BufferLinkJs {
    fn deref_mut(&mut self) -> &mut BufferLinkJsInner {
        &mut self.0
    }
}

impl From<BufferLinkJs> for BufferLinkJsInner {
    fn from(buffer_link_js: BufferLinkJs) -> Self {
        buffer_link_js.into_inner()
    }
}

use crate::{
    utils, StringArray, UniformCreateUpdateCallbackJs, UniformLink, UniformShouldUpdateCallbackJs,
};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;

pub type UniformLinkJsInner = UniformLink<String, String>;

#[wasm_bindgen(inspectable, js_name = UniformLink)]
pub struct UniformLinkJs(UniformLinkJsInner);

#[wasm_bindgen(js_class = UniformLink)]
impl UniformLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        program_ids: StringArray,
        uniform_id: String,
        initialize_callback: UniformCreateUpdateCallbackJs,
    ) -> Self {
        let program_ids = utils::js_array_to_vec_strings(&program_ids);
        Self(UniformLinkJsInner::new(
            program_ids,
            uniform_id,
            initialize_callback,
        ))
    }

    #[wasm_bindgen(js_name = programIds)]
    pub fn program_ids(&self) -> StringArray {
        utils::strings_to_js_array(self.deref().program_ids())
    }

    #[wasm_bindgen(js_name = uniformId)]
    pub fn uniform_id(&self) -> String {
        self.deref().uniform_id().to_owned()
    }

    #[wasm_bindgen(js_name = initializeCallback)]
    pub fn initialize_callback(&self) -> Option<UniformCreateUpdateCallbackJs> {
        self.deref()
            .initialize_callback()
            .js()
            .map(|c| (*c).clone())
    }

    #[wasm_bindgen(js_name = setInitializeCallback)]
    pub fn set_initialize_callback(&mut self, callback: UniformCreateUpdateCallbackJs) {
        self.deref_mut().set_initialize_callback(callback);
    }

    #[wasm_bindgen(js_name = shouldUpdateCallback)]
    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallbackJs> {
        self.deref()
            .should_update_callback()
            .and_then(|callback| callback.js_inner_owned())
    }

    #[wasm_bindgen(js_name = setShouldUpdateCallback)]
    pub fn set_should_update_callback(&mut self, callback: UniformShouldUpdateCallbackJs) {
        self.deref_mut().set_should_update_callback(callback);
    }

    #[wasm_bindgen(js_name = setUpdateCallback)]
    pub fn set_update_callback(&mut self, callback: UniformCreateUpdateCallbackJs) {
        self.deref_mut().set_update_callback(callback);
    }

    #[wasm_bindgen(js_name = updateCallback)]
    pub fn update_callback(&self) -> Option<UniformCreateUpdateCallbackJs> {
        self.deref()
            .update_callback()
            .and_then(|callback| callback.js_inner_owned())
    }

    #[wasm_bindgen(js_name = useInitCallbackForUpdate)]
    pub fn use_init_callback_for_update(&self) -> bool {
        self.deref().use_init_callback_for_update()
    }

    #[wasm_bindgen(js_name = setUseInitCallbackForUpdate)]
    pub fn set_use_init_callback_for_update(&mut self, use_init_callback_for_update: bool) {
        self.deref_mut()
            .set_use_init_callback_for_update(use_init_callback_for_update);
    }
}

impl From<UniformLinkJs> for UniformLinkJsInner {
    fn from(uniform_link_js: UniformLinkJs) -> Self {
        uniform_link_js.into_inner()
    }
}

impl UniformLinkJs {
    pub fn into_inner(self) -> UniformLinkJsInner {
        self.0
    }
}

impl From<UniformLinkJsInner> for UniformLinkJs {
    fn from(js_program_link_inner: UniformLinkJsInner) -> Self {
        Self(js_program_link_inner)
    }
}

impl Deref for UniformLinkJs {
    type Target = UniformLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UniformLinkJs {
    fn deref_mut(&mut self) -> &mut UniformLinkJsInner {
        &mut self.0
    }
}

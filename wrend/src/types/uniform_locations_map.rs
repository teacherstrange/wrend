use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const UNIFORM_LOCATIONS_mAP: &'static str = r#"
type UniformLocationsMap = Map<string, WebGLUniformLocation>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "UniformLocationsMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type UniformLocationsMap;
}

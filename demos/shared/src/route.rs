use yew_router::prelude::*;

#[derive(Copy, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/hello-quad")]
    HelloQuad,
    #[at("/hello-quad-animated")]
    HelloQuadAnimated,
    #[at("/game-of-life")]
    GameOfLife,
    #[at("/larger-than-life")]
    LargerThanLife,
    #[at("/simplex-noise")]
    SimplexNoise,
    #[at("/flow-field")]
    FlowField,
    #[at("/ray-tracer")]
    RayTracer,
    #[at("/recording-demo")]
    RecordingDemo,
    #[at("/kaleidoscope")]
    Kaleidoscope,
}

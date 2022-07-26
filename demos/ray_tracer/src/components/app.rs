use crate::components::canvas::Canvas;
use crate::components::global_listeners::GlobalListeners;
use crate::components::keyboard_listener::KeyboardListener;
use crate::components::menu::Menu;
use crate::state::app_context::AppContext;
use yew::{function_component, html, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ContextProvider<AppContext> context={AppContext::default()}>
            <KeyboardListener>
                <GlobalListeners>
                    <div class="ray-tracer">
                        <Canvas />
                        <Menu />
                    </div>
                </ GlobalListeners>
            </ KeyboardListener>
        </ContextProvider<AppContext>>

    }
}

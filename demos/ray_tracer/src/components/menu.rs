use crate::components::button::Button;
use crate::state::app_context::{AppContext, AppContextError};
use crate::state::ui_state_action::UiStateAction;
use shared::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let show_menu = app_context.ui_state.show_menu();

    // if menu is opened, make sure all keydown settings are `false`
    // to prevent unintended movement while the view is paused
    use_effect_with_deps(
        {
            let app_context = app_context.clone();
            move |show_menu| {
                if *show_menu {
                    app_context
                        .render_state
                        .borrow_mut()
                        .keydown_state_mut()
                        .set_all_false();
                }
                || {}
            }
        },
        show_menu,
    );

    if !show_menu {
        return html! {};
    }

    let handle_enable_button_click = {
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector("canvas")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            // there is a global listener that updates state in reaction to this
            canvas.request_pointer_lock();
        })
    };

    let handle_cancel_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            app_context.render_state.borrow_mut().set_is_paused(false);
            app_context
                .ui_state
                .dispatch(UiStateAction::SetShowMenu(false));
        })
    };

    let handle_save_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            app_context
                .render_state
                .borrow_mut()
                .set_should_save_image(true);
        })
    };

    let handle_reset_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            *app_context.render_state.borrow_mut() = Default::default();
        })
    };

    let handle_start_recording = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *app_context.renderer.borrow_mut() {
                renderer.start_recording();
                app_context
                    .ui_state
                    .dispatch(UiStateAction::SetIsRecording(true));
            }
        })
    };

    let handle_stop_recording = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *app_context.renderer.borrow_mut() {
                renderer.stop_recording();
                app_context
                    .ui_state
                    .dispatch(UiStateAction::SetIsRecording(false));
            }
        })
    };

    html! {
        <>
            <div class="underlay" />
            <div class="menu">
                <h2>{"Paused"}</h2>
                <p>{"Please enable first-person viewing mode"}</p>
                <Button onclick={handle_enable_button_click}>
                    {"Enable"}
                </Button>
                <Button onclick={handle_cancel_button_click}>
                    {"Cancel"}
                </Button>
                <Button onclick={handle_save_button_click}>
                    {"Save Image"}
                </Button>
                {if !app_context.ui_state.is_recording() {
                    html!{
                        <Button onclick={handle_start_recording}>
                            {"Start Recording"}
                        </Button>
                    }
                } else {
                    html!{
                        <Button onclick={handle_stop_recording}>
                            {"Stop Recording"}
                        </Button>
                    }
                }}
                <Button onclick={handle_reset_button_click}>
                    {"Reset"}
                </Button>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </div>
        </>
    }
}

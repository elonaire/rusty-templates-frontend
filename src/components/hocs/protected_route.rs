use yew::prelude::*;
use yew_router::prelude::*;

use crate::{app::{AppStateContext, Route, StateAction}, components::loading_spinner::LoadingSpinner, data::models::user::AuthDetails, utils::auth_interceptor::retrieve_new_token};

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps {
    pub component: Html,
}

#[function_component]
/// This component is a wrapper around the `component` prop. It checks if the user is authenticated
pub fn ProtectedRoute(props: &ProtectedRouteProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let token_is_loading = use_state(|| false); // State to track if token is being refreshed
    let navigator = use_navigator().unwrap();

    // An example of how you would handle an HTTP request to refresh tokens in your components using `wasm_bindgen_futures`
    let current_state_clone = current_state.clone();
    let token_is_loading_clone = token_is_loading.clone();
    let navigator_clone = navigator.clone();
    use_effect_with_deps(move |_| {
        let current_state_clone = current_state_clone.clone();
        let navigator_clone = navigator_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let auth_details = current_state_clone.auth_details.clone();
            if auth_details.token.is_empty() {
                token_is_loading_clone.set(true);
                let token = retrieve_new_token(&auth_details.token).await;
                match &token {
                    Ok(token) => {
                        current_state_clone.dispatch(StateAction::UpdateUserAuthInfo(AuthDetails {
                            token: if token.is_empty() { Default::default() } else { token.to_owned() },
                            user: Default::default(),
                            url: None
                        }));

                        if token.is_empty() {
                            token_is_loading_clone.set(false);
                            navigator_clone.push(&Route::SignIn);
                        }
                    },
                    Err(err) => log::error!("Error: {:?}", err),
                }

                token_is_loading_clone.set(false);
            }
        });
        || ()
    }, ());


    html! {
        <>
        {
            if *token_is_loading {
                return html!{ <LoadingSpinner /> };
            } else {
                props.component.clone()
            }
        }
        </>
    }
}

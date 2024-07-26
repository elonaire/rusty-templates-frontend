use yew::prelude::*;
use yew_router::components::Redirect;
use gloo_storage::{LocalStorage, Storage};

use crate::{app::{AppStateContext, Route, StateAction}, data::models::general::AuthDetails};

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps {
    pub component: Html,
}

#[function_component]
/// This component is a wrapper around the `component` prop. It checks if the user is authenticated
pub fn ProtectedRoute(props: &ProtectedRouteProps) -> Html {
    let stored_auth_data: Option<AuthDetails> = LocalStorage::get("auth_details").ok();
    let current_state = use_context::<AppStateContext>().unwrap();

    // An example of how you would handle an HTTP request to refresh tokens in your components using `wasm_bindgen_futures`
    // use_effect_with_deps(move |_| {
    //     let state_ctx_reducer = state_ctx_reducer.clone();
    //     wasm_bindgen_futures::spawn_local(async move {
    //         let auth_details = state_ctx_reducer.user_auth_details.clone();
    //         if auth_details.token.is_empty() {
    //             let token = retrieve_new_token().await;
    //             match &token {
    //                 Ok(token) => {
    //                     state_ctx_reducer.dispatch(StateAction::UpdateUserAuthInfo(AuthDetails {
    //                         token: if token.is_empty() { Default::default() } else { token.to_owned() },
    //                         user: Default::default(),
    //                     }));

    //                     log::info!("Token: {:?}", token);
    //                 },
    //                 Err(err) => log::error!("Error: {:?}", err),
    //             }
    //         }
    //     });
    //     || ()
    // }, ());
    let stored_auth_data_clone = stored_auth_data.clone();
    use_effect_with_deps(move |_| {
        match stored_auth_data_clone {
            Some(data) => {
                current_state.dispatch(StateAction::UpdateUserAuthInfo(
                    data
                ))
            },
            None => {
                current_state.dispatch(StateAction::UpdateUserAuthInfo(
                    AuthDetails::default()
                ))
            }
        }
    }, ());


    html! {
        <>
        {
            // if Some() {
            //     props.component.clone()
            // } else {
            //     html! { <Redirect<Route> to={Route::SignIn}/> }
            // }
            match stored_auth_data {
                Some(_data) => props.component.clone(),
                None => html! { <Redirect<Route> to={Route::SignIn}/> }
            }

        }
        </>
    }
}

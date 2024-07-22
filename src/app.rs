use std::rc::Rc;

use yew::prelude::*;
use crate::{views::{landing::Landing, store::StorePage}, data::models::user::User};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/store")]
    Store,
    #[at("/cart")]
    Cart,
    #[at("/sign-in")]
    SignIn,
    #[at("/sign-up")]
    SignUp,
    #[at("/account")]
    Account,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum StateAction {
    UpdateUserInfo(User),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppState {
    pub user_details: User,
}

impl Reducible for AppState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let updated_state = match action {
            StateAction::UpdateUserInfo(user) => {
                AppState {
                    user_details: user,
                    ..self.as_ref().clone()
                }
            }
        };

        Self { ..updated_state }.into()
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Landing => html! { <Landing /> },
        Route::Store => html! { <StorePage /> },
        Route::Cart => html! { <h1>{ "Cart" }</h1> },
        Route::SignIn => html! { <h1>{ "Sign In" }</h1> },
        Route::SignUp => html! { <h1>{ "Sign Up" }</h1> },
        Route::Account => html! { <h1>{ "Account" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| AppState::default());

    html! {
        <ContextProvider<AppStateContext> context={state}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AppStateContext>>
    }
}

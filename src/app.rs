use std::rc::Rc;

use yew::prelude::*;
use crate::{data::models::{order::Cart, template::Product, user::AuthDetails}, views::{cart::CartPage, landing::Landing, sign_in::SignInPage, sign_up::SignUpPage, store::StorePage, thankyou::ThankYouPage}};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
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
    #[at("/thankyou")]
    ThankYou,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppRoute {
    Route(Route),
}

pub enum StateAction {
    UpdateUserAuthInfo(AuthDetails),
    UpdateProducts(Vec<Product>),
    UpdateCart(Cart),
    UpdateCartProducts(Vec<Product>),
    UpdateCheckoutUrl(String),
    UpdateCartProductsIds(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppState {
    pub auth_details: AuthDetails,
    pub products: Vec<Product>,
    pub cart_products: Vec<Product>,
    pub cart: Cart,
    pub checkout_url: String,
    pub cart_products_ids: Vec<String>
}

impl Reducible for AppState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let updated_state = match action {
            StateAction::UpdateUserAuthInfo(user) => {
                log::info!("track here user: {:?}", user);
                AppState {
                    auth_details: user,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateProducts(products) => {
                log::info!("track here products: {:?}", products);
                AppState {
                    products,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateCart(cart) => {
                AppState {
                    cart,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateCartProducts(products) => {
                AppState {
                    cart_products: products,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateCheckoutUrl(checkout_url) => {
                AppState {
                    checkout_url,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateCartProductsIds(cart_products_ids) => {
                AppState {
                    cart_products_ids,
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
        Route::Cart => html! { <CartPage /> },
        Route::SignIn => html! { <SignInPage /> },
        Route::SignUp => html! { <SignUpPage /> },
        Route::Account => html! { <h1>{ "Account" }</h1> },
        Route::ThankYou => html! { <ThankYouPage /> },
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

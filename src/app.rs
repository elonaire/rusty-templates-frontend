use std::rc::Rc;

use yew::prelude::*;
use crate::{components::hocs::protected_route::ProtectedRoute, data::models::{order::{Cart, CartProduct, License}, template::Product, user::AuthDetails}, views::{account::AccountPage, cart::CartPage, details::TemplateDetails, landing::Landing, sign_in::SignInPage, sign_up::SignUpPage, store::StorePage, thankyou::ThankYouPage}};
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
    #[at("/template")]
    TemplateRoot,
    #[at("/template/*")]
    TemplateSlug,
    #[at("/thankyou")]
    ThankYou,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug, Eq)]
pub enum TemplateRoute {
    #[at("/template/details/:id")]
    TemplateDetails { id: String },
    #[not_found]
    #[at("/template/404")]
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
    UpdateCurrentProductDetails(Product),
    UpdateLicenses(Vec<License>),
    UpdateRawCartProducts(Vec<CartProduct>),
    UpdateOrderCartProducts(Vec<CartProduct>),
    UpdateGlobalError(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AppState {
    pub auth_details: AuthDetails,
    pub products: Vec<Product>,
    pub cart_products: Vec<Product>,
    pub cart: Cart,
    pub checkout_url: String,
    pub cart_products_ids: Vec<String>,
    pub current_product_details: Product,
    pub licenses: Vec<License>,
    pub raw_cart_products: Vec<CartProduct>,
    pub order_cart_products: Vec<CartProduct>,
    pub global_error: String,
}

impl Reducible for AppState {
    type Action = StateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let updated_state = match action {
            StateAction::UpdateUserAuthInfo(user) => {
                AppState {
                    auth_details: user,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateProducts(products) => {
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
            },
            StateAction::UpdateCurrentProductDetails(current_product_details) => {
                AppState {
                    current_product_details,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateLicenses(licenses) => {
                AppState {
                    licenses,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateRawCartProducts(raw_cart_products) => {
                AppState {
                    raw_cart_products,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateOrderCartProducts(order_cart_products) => {
                AppState {
                    order_cart_products,
                    ..self.as_ref().clone()
                }
            },
            StateAction::UpdateGlobalError(global_error) => {
                AppState {
                    global_error,
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
        Route::Account => html! { <ProtectedRoute component={html! { <AccountPage /> }} /> },
        // Route::TemplateDetails => html! { <TemplateDetails /> },
        Route::TemplateRoot | Route::TemplateSlug => html! { <Switch<TemplateRoute> render={template_switch} /> },
        Route::ThankYou => html! { <ThankYouPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub fn template_switch(routes: TemplateRoute) -> Html {
    match routes {
        TemplateRoute::TemplateDetails { id } => html! { <TemplateDetails {id} /> },
        TemplateRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound} />}
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

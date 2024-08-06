use yew::prelude::*;
use yew::function_component;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use crate::data::context::orders::get_cart;
use crate::data::context::orders::get_product_external_ids;
use crate::data::context::products::get_products_by_ids;
use crate::{app::{Route, AppStateContext}, components::badge::Badge};

#[function_component]
pub fn TopNav() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    // Create state to manage the visibility of the sidemenu
    let is_menu_open = use_state(|| false);
    let location = use_location().expect("No location available");
    println!("location: {:?}", location);

    // Function to toggle the visibility of the menu
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let current_state_clone_update = current_state.clone();
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let _cart = get_cart(&current_state_clone_update).await;

            let _cart_product_ids = get_product_external_ids(&current_state_clone_update).await;

            let _cart_products = get_products_by_ids(&current_state_clone_update).await;
        });
    }, current_state.clone());


    html! {
        <>
            <header class="bg-white shadow">
                <nav class="container mx-auto p-4 flex justify-between items-center">
                    // <h1 class="text-2xl font-bold">{"Rusty Templates"}</h1>
                    <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>
                        <img class="w-24" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="logo" />
                    </Link<Route>>
                    <div class="hidden md:flex items-center">
                        <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>{"Home"}</Link<Route>>
                        <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Store}>{"Templates"}</Link<Route>>
                        {
                            if current_state.auth_details.token.is_empty() {
                                html! {
                                    <Link<Route> classes={"px-6 py-1 rounded transition border-2 border-primary text-primary hover:bg-primary hover:text-white"} to={Route::SignIn}>{"Sign In"}</Link<Route>>
                                }
                            } else {
                                html! {
                                    <Link<Route> classes={"text-gray-700 px-4"} to={Route::Account}>
                                    <div class="py-1 px-4 ring-2 ring-primary rounded flex flex-row items-center gap-2">
                                            <p>{"My Account"}</p>
                                            <Icon class={classes!("text-primary")} width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={IconId::FontAwesomeSolidCircleUser}/>
                                        </div>
                                    </Link<Route>>
                                }
                            }
                        }
                        // <a href="#templates" class="text-gray-700 px-4">{"Templates"}</a>
                        // <a href="#contact" class="text-gray-700 px-4">{"Contact"}</a>

                        <Link<Route> classes={"text-gray-700 px-4"} to={Route::Cart}>
                            <Badge color={"bg-primary"} text={current_state.cart_products.len().clone().to_string()}>
                                <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                            </Badge>
                        </Link<Route>>
                    </div>
                    <button class="block md:hidden" onclick={toggle_menu.clone()}>
                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"/>
                        </svg>
                    </button>
                </nav>
            </header>
            <div class={format!("{} fixed inset-0 bg-gray-800 bg-opacity-85 z-50 p-6 flex flex-col space-y-4 {}",
                if *is_menu_open { "flex" } else { "hidden" },
                if *is_menu_open { "" } else { "hidden" })}>
                <button class="self-end text-white" onclick={toggle_menu.clone()}>
                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                </button>
                <Link<Route> classes={"text-white text-center text-xl"} to={Route::Landing}>{"Home"}</Link<Route>>
                <Link<Route> classes={"text-white text-center text-xl"} to={Route::Store}>{"Templates"}</Link<Route>>
                {
                    if current_state.auth_details.token.is_empty() {
                        html! {
                            <Link<Route> classes={"px-6 py-1 rounded transition border-2 border-primary text-primary hover:bg-primary hover:text-white text-center"} to={Route::SignIn}>{"Sign In"}</Link<Route>>
                        }
                    } else {
                        html! {}
                    }
                }
                // <a href="#templates" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Templates"}</a>
                // <a href="#contact" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Contact"}</a>
                <Link<Route> classes={"text-white flex items-center justify-center text-xl"} to={Route::Account}>
                    // <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::FontAwesomeSolidCircleUser}/>
                    <div class="py-1 px-4 ring-2 ring-primary rounded flex flex-row items-center gap-2">
                            <p>{"My Account"}</p>
                            <Icon class={classes!("text-primary")} width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::FontAwesomeSolidCircleUser}/>
                        </div>
                </Link<Route>>
                <Link<Route> classes={"text-white flex items-center justify-center text-xl"} to={Route::Cart}>
                    <Badge color={"bg-primary"} text={current_state.cart_products.len().clone().to_string()}>
                        <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                    </Badge>
                </Link<Route>>
            </div>
        </>
    }
}

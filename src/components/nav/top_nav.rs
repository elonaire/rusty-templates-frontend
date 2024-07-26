use yew::prelude::*;
use yew::function_component;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use crate::app::Route;

#[function_component]
pub fn TopNav() -> Html {
    // Create state to manage the visibility of the sidemenu
    let is_menu_open = use_state(|| false);

    // Function to toggle the visibility of the menu
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };


    html! {
        <>
            <header class="bg-white shadow">
                <nav class="container mx-auto p-4 flex justify-between items-center">
                    // <h1 class="text-2xl font-bold">{"Rusty Templates"}</h1>
                    <img class="w-24" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="logo" />
            <div class="hidden md:flex items-center">
                        <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>{"Home"}</Link<Route>>
                        <a href="#templates" class="text-gray-700 px-4">{"Templates"}</a>
                        <a href="#contact" class="text-gray-700 px-4">{"Contact"}</a>
                        <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>
                            <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                        </Link<Route>>
                    </div>
                    <button class="block md:hidden" onclick={toggle_menu.clone()}>
                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"/>
                        </svg>
                    </button>
                </nav>
            </header>
            <div class={format!("{} fixed inset-0 bg-gray-800 bg-opacity-75 z-50 p-6 flex flex-col space-y-4 {}",
                if *is_menu_open { "flex" } else { "hidden" },
                if *is_menu_open { "" } else { "hidden" })}>
                <button class="self-end text-white" onclick={toggle_menu.clone()}>
                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                    </svg>
                </button>
                <Link<Route> classes={classes!("text-white", "text-center", "text-xl")} to={Route::Landing}>{"Home"}</Link<Route>>
                <a href="#templates" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Templates"}</a>
                <a href="#contact" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Contact"}</a>
                <Link<Route> classes={classes!("text-white", "text-center", "text-xl")} to={Route::Landing}>
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                </Link<Route>>
            </div>
        </>
    }
}

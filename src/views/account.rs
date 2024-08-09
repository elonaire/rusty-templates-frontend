use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{
    app::AppStateContext,
    components::{nav::top_nav::TopNav, rating::Rating},
    data::{
        context::{
            orders::get_order_cart_products_by_status, products::get_products, users::get_new_token,
        },
        models::order::OrderStatus,
    },
};

#[derive(Clone, PartialEq)]
enum Tab {
    Notifications,
    Orders,
    DeleteAccount,
}

#[function_component]
pub fn AccountPage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let loading = use_state_eq(|| false);
    let active_tab = use_state(|| Tab::Orders);

    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| active_tab.set(tab))
    };

    let notifications_active = *active_tab == Tab::Notifications;
    let orders_active = *active_tab == Tab::Orders;
    let delete_active = *active_tab == Tab::DeleteAccount;

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(
        move |_| {
            loading_clone.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                if current_state_clone.products.is_empty() {
                    let _products = get_products(&current_state_clone).await;
                }

                if current_state_clone.auth_details.token.is_empty() {
                    let _new_token = get_new_token(&current_state_clone).await;
                }

                if current_state_clone.order_cart_products.is_empty() {
                    let _new_token = get_order_cart_products_by_status(
                        &current_state_clone,
                        OrderStatus::Confirmed,
                    )
                    .await;
                }

                loading_clone.set(false);
            });
        },
        (),
    );

    html! {
        <div class="bg-gray-100 min-h-svh font-jost-sans">
            <TopNav />
            <div class="container min-h-svh bg-gray-100 py-10 mx-auto px-2">
                <div class="mx-auto bg-white shadow-md rounded min-h-72">
                    <div class="flex bg-gray-200 p-4 rounded-t">
                        <TabButton
                            active={notifications_active}
                            label="Notifications"
                            ontabclick={on_tab_select.clone()}
                            tab={Tab::Notifications}
                        />
                        <TabButton
                            active={orders_active}
                            label="Orders"
                            ontabclick={on_tab_select.clone()}
                            tab={Tab::Orders}
                        />
                        <TabButton
                            active={delete_active}
                            label="Delete Account"
                            ontabclick={on_tab_select.clone()}
                            tab={Tab::DeleteAccount}
                        />
                    </div>
                    <div class="p-4">
                        { match *active_tab {
                            Tab::Notifications => html! { <Notifications /> },
                            Tab::Orders => html! { <Orders /> },
                            Tab::DeleteAccount => html! { <DeleteAccount /> },
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TabButtonProps {
    active: bool,
    label: &'static str,
    ontabclick: Callback<Tab>,
    tab: Tab,
}

#[function_component]
fn TabButton(props: &TabButtonProps) -> Html {
    let class = if props.active {
        "p-4 text-primary font-bold"
    } else {
        "p-4"
    };

    let ontabclick = {
        let tab = props.tab.clone();
        let ontabclick = props.ontabclick.clone();
        Callback::from(move |_| ontabclick.emit(tab.clone()))
    };

    html! {
        <button class={class} onclick={ontabclick}>
            { props.label }
        </button>
    }
}

#[function_component]
fn Notifications() -> Html {
    html! {
        <div>
            <h2 class="text-xl font-bold mb-4">{ "Notification Settings" }</h2>
            <p>{ "Modify your notification settings here." }</p>
            // Add your notification settings form here
        </div>
    }
}

#[function_component]
fn Orders() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

    let download_file_uri = option_env!("FILES_SERVICE_DOWNLOAD_URL")
        .expect("FILES_SERVICE_DOWNLOAD_URL env var not set");
    let view_file_uri =
        option_env!("FILES_SERVICE_VIEW_URL").expect("FILES_SERVICE_VIEW_URL env var not set");

    // use_effect_with_deps(move |_| {
    //     wasm_bindgen_futures::spawn_local(async move {

    //     });
    // }, current_state.clone());

    html! {
        <div class="mx-auto p-4">
            <h2 class="text-2xl font-bold mb-4">{ "Your Orders" }</h2>
            <p class="mb-4">{ "Here are your purchased templates." }</p>
            <div class="flex flex-col gap-2">
            {
                current_state.order_cart_products.iter().filter_map(|order| {
                    // Find the product that matches the order's product ID
                    current_state.products.iter().find(|product| {
                        product.id.as_ref().map_or(false, |id| id == &order.ext_product_id)
                    }).map(|product| {
                        html! {
                            <div class="flex flex-row border rounded shadow-lg space-x-4">
                                <div class="basis-1/3">
                                    <img src={format!("{}{}", view_file_uri, product.screenshot.clone().unwrap_or_else(|| String::from("fallback_image_url.png")))}
                                         alt={product.name.clone().unwrap_or_else(|| String::from("Unnamed Product"))}
                                         class="w-full h-auto object-cover rounded mb-2" />
                                </div>
                                <div class="basis-1/3 flex flex-col gap-2">
                                    <h3 class="text-lg font-semibold">{ product.name.clone().unwrap_or_else(|| String::from("Unnamed Product")) }</h3>
                                    <a href={format!("{}{}", download_file_uri, order.artifact.clone().unwrap_or_else(|| String::from("#")))} class="px-6 py-1  rounded border-2 border-primary text-primary hover:bg-primary hover:text-white w-1/2 flex items-center justify-center gap-2">
                                        <span>{ "Download" }</span><Icon class={""} width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::LucideDownload}/>
                                    </a>
                                </div>
                                <div class="basis-1/3 flex flex-col gap-2">
                                // <h3 class="text-lg font-semibold">{ "Submit a Review" }</h3>
                                // <Rating />
                                </div>
                            </div>
                        }
                    })
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}

#[function_component]
fn DeleteAccount() -> Html {
    html! {
        <div>
            <h2 class="text-xl font-bold mb-4">{ "Delete Account" }</h2>
            <p class="mb-4">{ "Warning: This action is irreversible. You will lose all your data." }</p>
            <button class="px-4 py-2 bg-red-600 text-white rounded">{ "Delete Account" }</button>
        </div>
    }
}

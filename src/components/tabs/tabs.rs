use yew::prelude::*;

use crate::components::nav::top_nav::TopNav;

#[derive(Clone, PartialEq)]
enum Tab {
    Notifications,
    Orders,
    DeleteAccount,
}

#[function_component]
pub fn AccountPage() -> Html {
    let active_tab = use_state(|| Tab::Orders);

    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| active_tab.set(tab))
    };

    let notifications_active = *active_tab == Tab::Notifications;
    let orders_active = *active_tab == Tab::Orders;
    let delete_active = *active_tab == Tab::DeleteAccount;

    html! {
        <div class="bg-gray-100 min-h-svh font-jost-sans">
            <TopNav />
            <div class="container min-h-svh bg-gray-100 py-10 mx-auto">
                <div class="mx-auto bg-white shadow-md rounded">
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
    html! {
        <div>
            <h2 class="text-xl font-bold mb-4">{ "Your Orders" }</h2>
            <p>{ "Here are your purchased templates." }</p>
            // Add your orders details here
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

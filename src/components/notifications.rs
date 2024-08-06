use crate::components::button::BasicButton;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub id: usize,
    pub message: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationsProps {
    pub notifications: Vec<Notification>,
    pub on_close: Callback<usize>,
}

#[function_component(Notifications)]
pub fn notifications(props: &NotificationsProps) -> Html {
    html! {
        <div class="md:w-80 space-y-1">
            { for props.notifications.iter().map(|notification| html! {
                <NotificationItem key={notification.id} notification={notification.clone()} on_close={props.on_close.clone()} />
            })}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct NotificationItemProps {
    notification: Notification,
    on_close: Callback<usize>,
}

#[function_component(NotificationItem)]
fn notification_item(props: &NotificationItemProps) -> Html {
    let onclose = {
        let id = props.notification.id;
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(id))
    };

    html! {
        <div class="bg-slate-50 shadow-md rounded p-4 flex justify-between items-center transition-opacity duration-300 ease-in-out hover:shadow-lg">
            <span>{ &props.notification.message }</span>
            <BasicButton icon={Some(yew_icons::IconId::BootstrapXLg)} style_ext={"ml-4 text-red-500 hover:text-red-700 focus:outline-none"} onclick={onclose} />
        </div>
    }
}

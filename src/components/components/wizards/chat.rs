use std::ops::Deref;

use crate::{
    app::AppStateContext,
    components::{
        badge::Badge,
        button::BasicButton,
        forms::input::{InputField, InputFieldType},
    },
    data::{
        context::{
            chat::{add_new_message, update_chat_head},
            init::set_messages,
        },
        models::general::{ChatMessage, User},
    },
    utils::time::format_timestamp,
};

use chrono::Utc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Chat Application
#[derive(Properties, PartialEq)]
pub struct ChatAppProps;

#[function_component]
pub fn ChatApp(_props: &ChatAppProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let show_contacts = use_state(|| true);

    let on_send = {
        let current_state_clone = current_state.clone();
        Callback::from(move |message: ChatMessage| {
            let _new_message = add_new_message(current_state_clone.clone(), message);
        })
    };

    let toggle_contacts = {
        let show_contacts = show_contacts.clone();
        Callback::from(move |_| {
            show_contacts.set(!*show_contacts);
        })
    };

    // This callback will be used to hide the contacts overlay when clicking outside
    let hide_contacts_overlay = {
        let show_contacts = show_contacts.clone();
        Callback::from(move |_| {
            if *show_contacts {
                show_contacts.set(false);
            }
        })
    };

    html! {
        <div class="h-svh flex flex-col bg-gray-100 relative" onclick={hide_contacts_overlay}>
            <ChatHeader />
            <MessageList />
            <MessageInput {on_send} />
            <BasicButton onclick={toggle_contacts.clone().reform(|e: MouseEvent| e.stop_propagation())} button_text={""} style_ext={"fixed bottom-20 right-4 bg-blue-500 text-white rounded-full py-4 px-4 shadow-lg"} icon={Some(yew_icons::IconId::BootstrapPlusLg)} />
            if *show_contacts {
                <ContactsOverlay toggle_contacts={toggle_contacts.clone()} />
            }
        </div>
    }
}

// Message List Component
#[derive(Properties, PartialEq)]
struct MessageListProps;

#[function_component]
fn MessageList(_props: &MessageListProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

    let state_clone_deps = current_state.clone();
    let state_clone_effect = current_state.clone();
    use_effect_with_deps(
        move |_| {
            let _messages = set_messages(state_clone_effect.clone());
        },
        (
            state_clone_deps.user_auth_details.clone(),
            state_clone_deps.current_chat.clone(),
            state_clone_deps.messages.clone(),
        ),
    );

    html! {
        <div class="flex-1 overflow-y-auto p-4 space-y-2 bg-slate-50">
            { for current_state.messages.iter().map(|msg| html! { <MessageItem message={msg.clone()} /> }) }
        </div>
    }
}

// Message Item Component(Single Message)
#[derive(Properties, PartialEq)]
struct MessageItemProps {
    message: ChatMessage,
}

#[function_component]
fn MessageItem(props: &MessageItemProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let is_sent = current_state.user_auth_details.user.id == props.message.sender_id;
    let sender_details = current_state.current_chat.clone();

    let (bg_color, text_align, img_margin) = if is_sent {
        ("bg-blue-500 text-white", "", "ml-4")
    } else {
        ("bg-gray-200 text-black", "text-left", "mr-4")
    };

    html! {
        <div class={format!("flex items-start mb-4 {}", if is_sent { "justify-end" } else { "" })}>
            { if !is_sent {
                html! {
                    <img src={sender_details.profile_picture.clone()} alt="Profile Image" class={format!("w-10 h-10 rounded-full object-cover {}", img_margin)} />
                }
            } else {
                html! {}
            }}
            <div class={format!("p-2 rounded-lg max-w-xs {} {}", bg_color, text_align)}>
                <div>{ &props.message.message }</div>
                <div class="text-xs text-gray-600 mt-1">{ format_timestamp(props.message.timestamp) }</div>
            </div>
            { if is_sent {
                html! {
                    <img src={current_state.user_auth_details.user.profile_picture.clone()} alt="Profile Image" class={format!("w-10 h-10 rounded-full object-cover {}", img_margin)} />
                }
            } else {
                html! {}
            }}
        </div>
    }
}

// Message Input Component
#[derive(Properties, PartialEq)]
struct MessageInputProps {
    on_send: Callback<ChatMessage>,
}

#[function_component]
fn MessageInput(props: &MessageInputProps) -> Html {
    let input_value = use_state(|| String::new());
    let current_state = use_context::<AppStateContext>().unwrap();

    let onsubmit = {
        let input_value_clone = input_value.clone();
        let on_send = props.on_send.clone();

        let state_clone = current_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !input_value_clone.is_empty() {
                let chat_message = ChatMessage {
                    message: (*input_value_clone).clone(),
                    sender_id: state_clone.user_auth_details.user.id.clone(),
                    recipient_id: state_clone.current_chat.id.clone(),
                    timestamp: Utc::now().timestamp(),
                };
                on_send.emit(chat_message);
                input_value_clone.set("".into());
            }
        })
    };

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            input_value.set(val);
        })
    };

    html! {
        <form class="p-4 flex items-center justify-stretch bg-gray-300" onsubmit={onsubmit}>
            <InputField autocomplete={"off"} ext_wrapper_styles={"basis-10/12"} ext_input_styles={"rounded-none rounded-l border-r-0 focus:ring-0"} oninput={oninput} initial_value={Some(input_value.deref().clone())} placeholder={"Type a message..."} name={"message"} field_type={InputFieldType::Text} />
            {
                if !input_value.is_empty() {
                    html!{
                        <BasicButton button_type={"submit"} button_text={""} style_ext={"bg-blue-500 hover:bg-blue-500 text-white h-full rounded-none rounded-r"} icon={Some(yew_icons::IconId::BootstrapSend)} />
                    }
                } else {
                    html!{
                        <BasicButton button_type={""} button_text={""} style_ext={"bg-blue-500 hover:bg-blue-500 text-white h-full rounded-none rounded-r"} icon={Some(yew_icons::IconId::BootstrapMic)} />
                    }
                }
            }
        </form>
    }
}

// Define the properties for the ChatHeader component
#[derive(Properties, PartialEq)]
struct ChatHeaderProps;

#[function_component]
fn ChatHeader(_props: &ChatHeaderProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    // Determine status text and color based on online status
    let (status_text, status_color) = if current_state.current_chat.online_status {
        ("Online", "text-green-400")
    } else {
        ("Offline", "text-gray-400")
    };

    html! {
        <header class="h-screen-1/13 p-4 bg-blue-500 text-white flex items-center justify-between">
            <div class="flex items-center space-x-4">
                <img src={current_state.current_chat.profile_picture.clone()} alt={format!("{} dp", current_state.current_chat.name)} class="w-10 h-10 rounded-full object-cover" />
                <div>
                    <p class="text-md font-semibold">{ &current_state.current_chat.name }</p>
                    <p class={format!("text-xs {}", status_color)}>
                        { status_text }
                    </p>
                </div>
            </div>

            <BasicButton button_text={""} style_ext={"text-xl font-bold focus:outline-none"} icon={Some(yew_icons::IconId::FeatherMoreVertical)} />
        </header>
    }
}

// Contact Component
#[derive(Properties, PartialEq, Clone)]
struct ContactProps {
    contact: User,
    toggle_contacts: Callback<()>,
}

#[function_component]
fn ContactComponent(props: &ContactProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let props_clone = props.clone();
    let current_state_clone = current_state.clone();

    let on_click = {
        let current_state_clone = current_state_clone.clone();
        // let props_clone = props_clone.clone();
        Callback::from(move |_| {
            // let contact_clone = props_clone.contact.clone();
            update_chat_head(current_state_clone.clone(), props_clone.contact.clone());
            props_clone.toggle_contacts.emit(());
        })
    };
    html! {
        <li onclick={on_click} class="flex items-center space-x-4 py-2 px-4 border-b hover:bg-blue-200 cursor-pointer">
            <img src={props.contact.profile_picture.clone()} alt={format!("Profile picture of {}", props.contact.name.clone())} class="w-8 h-8 object-contain rounded-full"/>
                <div class="w-full">
                <div class="flex flex-row items-center justify-between">
                    <p class="text-md font-semibold">{ props.contact.name.clone() }</p><Badge color={"green"} text={"9+"}></Badge>
                </div>
                <p class="text-xs text-gray-400">{ props.contact.email.clone() }</p>
            </div>
        </li>
    }
}

// Contacts Inset(Render Contacts)
#[derive(Properties, PartialEq)]
struct ContactsOverlayProps {
    toggle_contacts: Callback<()>,
}

#[function_component]
fn ContactsOverlay(props: &ContactsOverlayProps) -> Html {
    let stop_propagation = Callback::from(move |e: MouseEvent| e.stop_propagation());
    let current_state = use_context::<AppStateContext>().unwrap();

    html! {
        <>
        <div onclick={props.toggle_contacts.reform(|_| ())} class="absolute inset-0 h-svh w-full bg-gray-800 bg-opacity-75 z-60"></div>
            <div onclick={stop_propagation} class="absolute z-70 bg-slate-50 w-5/6 h-svh">
                <header class="h-screen-1/13 p-4 bg-blue-500 text-white font-bold">
                    { "Contacts" }
                </header>
                <ul class="h-screen-12/13">
                    { for current_state.contact_list.iter().filter(|contact| contact.id != current_state.user_auth_details.user.id).map(|contact| html! {
                        <ContactComponent toggle_contacts={props.toggle_contacts.clone()} contact={contact.clone().to_owned()} />
                    }) }
                </ul>
            </div>
        </>
    }
}

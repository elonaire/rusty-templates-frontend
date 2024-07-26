use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::modal::precursor_modal::PrecursorModal;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub use_case: UseCase,
    #[prop_or_default]
    pub on_click_primary: Callback<()>,
    #[prop_or_default]
    pub on_confirm: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
    #[prop_or_default]
    pub is_open: bool,
    #[prop_or("OK".to_owned())]
    pub primary_button_text: String,
}

#[allow(dead_code, unused)]
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum UseCase {
    Error,
    Success,
    Confirmation,
    Info,
    #[default]
    General,
}

#[function_component]
pub fn BasicModal(props: &ModalProps) -> Html {

    let on_click_primary = {
        let on_click_primary = props.on_click_primary.clone();
        Callback::from(move |_: MouseEvent| {
            on_click_primary.emit(());
        })
    };

    if !props.is_open {
        return html! {};
    }

    html! {
        <div class="fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-50 z-50">
            <div class="bg-slate-50 rounded shadow-lg transform -translate-y-1/4 w-full max-w-md m-2">
                <div class="flex items-center mb-4 border-b-[1px] p-4">
                    {
                        match &props.use_case {
                            UseCase::Error => html!{
                                <span class="text-red-500 mr-2">
                                    <Icon icon_id={IconId::BootstrapExclamationCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} />
                                </span>
                            },
                            UseCase::Success => html!{
                                <span class="text-green-500 mr-2">
                                    <Icon icon_id={IconId::BootstrapCheckCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} />
                                </span>
                            },
                            UseCase::Info => html!{
                                <span class="text-blue-500 mr-2">
                                    <Icon icon_id={IconId::BootstrapInfoCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} />
                                </span>
                            },
                            UseCase::Confirmation => html!{
                                <span class="text-yellow-500 mr-2">
                                    <Icon icon_id={IconId::BootstrapQuestionCircleFill} width={"1.5em".to_owned()} height={"1.5em".to_owned()} />
                                </span>
                            },
                            UseCase::General => html!{ },
                        }
                    }
                    <span class="font-semibold text-lg">{ &props.title }</span>
                </div>
                <div id="modal-content" class="mb-4 p-4">
                    <PrecursorModal>{ props.children.clone() }</PrecursorModal>
                </div>
                <div class="flex justify-end space-x-2 p-4">
                    {
                        if props.use_case == UseCase::Confirmation {
                            html!{ <button onclick={props.on_cancel.reform(|_| ())} class="px-4 py-2 bg-gray-400 text-white rounded">{"Cancel"}</button> }
                        } else {
                            html!()
                        }
                    }
                    <button onclick={on_click_primary} class="px-4 py-2 bg-blue-950 text-white rounded">{props.primary_button_text.clone()}</button>
                </div>
            </div>
        </div>
     }
}

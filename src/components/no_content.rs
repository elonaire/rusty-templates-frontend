use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
pub fn NoContent() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center h-full">
            <div class="flex flex-col items-center justify-center p-4">
                <p class="text-2xl font-semibold text-gray-300"><Icon icon_id={IconId::BootstrapInbox}/></p>
                <p class="text-sm text-gray-300">{"No Data!"}</p>
            </div>
        </div>
    }
}
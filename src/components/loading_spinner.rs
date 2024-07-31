use yew::prelude::*;



#[function_component]
pub fn LoadingSpinner() -> Html {
    html! {
        <div class="fixed inset-0 bg-gray-800 bg-opacity-75 flex justify-center items-center z-50">
            <div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-primary"></div>
        </div>
    }
}

use yew::prelude::*;
use crate::{app::AppStateContext, components::{button::BasicButton, nav::top_nav::TopNav}, data::context::users::get_new_token};

#[function_component]
pub fn ThankYouPage() -> Html {
    let loading = use_state_eq(|| false);
    let current_state = use_context::<AppStateContext>().unwrap();

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(move |_| {
        loading_clone.set(true);
        wasm_bindgen_futures::spawn_local(async move {

            if current_state_clone.auth_details.token.is_empty() {
               let _new_token = get_new_token(&current_state_clone).await;
            }

            loading_clone.set(false);
        });
    }, ());

    html! {
        <>
            <div class="bg-gray-100 min-h-svh font-jost-sans">
                <TopNav />
                <div class="container mx-auto py-10">
                    <div class="bg-white p-6 rounded m-auto shadow-lg w-full max-w-sm text-center">
                        <h1 class="text-2xl font-semibold text-gray-800">{ "Thank You!" }</h1>
                        <p class="mt-4 text-gray-600">{ "Your payment was successful." }</p>
                        <p class="mt-2 text-gray-600">{ "We appreciate your purchase!" }</p>
                        <p class="mt-2 text-gray-600">{ "You will receive a template download link in the email that you provided. In case of any problem, reach out to us via " }<strong>{"info@rustytemplates.com."}</strong></p>
                        // <button
                        //     class="mt-6 bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded"
                        //     onclick={Callback::from(|_| web_sys::window().unwrap().location().set_href("/").unwrap())}
                        // >
                        //     { "Return Home" }
                        // </button>
                        <BasicButton button_text={"Return Home"} style_ext={"mt-6 bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition"} />
                    </div>
                </div>
            </div>
        </>
    }
}

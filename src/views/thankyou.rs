use yew::prelude::*;
use crate::components::{nav::top_nav::TopNav, button::BasicButton};

#[function_component]
pub fn ThankYouPage() -> Html {
    html! {
        <>
            <TopNav />
            <div class="flex flex-col items-center justify-center bg-gray-100 h-screen-11/13">
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
        </>
    }
}

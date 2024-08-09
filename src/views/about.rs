use crate::{
    app::AppStateContext,
    components::{button::BasicButton, nav::top_nav::TopNav},
    data::context::users::get_new_token,
};
use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    let loading = use_state_eq(|| false);
    let current_state = use_context::<AppStateContext>().unwrap();
    let view_file_uri = option_env!("FILES_SERVICE_VIEW_PROD_URL")
        .expect("FILES_SERVICE_VIEW_PROD_URL env var not set");
    let dp_file = use_state_eq(|| "0f543cb7-3305-40bc-a442-410c66de2c7d".to_string());

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(
        move |_| {
            loading_clone.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                if current_state_clone.auth_details.token.is_empty() {
                    let _new_token = get_new_token(&current_state_clone).await;
                }

                loading_clone.set(false);
            });
        },
        (),
    );

    html! {
        <>
            <div class="bg-gray-100 min-h-svh font-jost-sans">
                <TopNav />
                <div class="container mx-auto py-10 px-2">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // About Info Column
                    <div>
                        <h1 class="text-4xl font-bold mb-6">{"About Rusty Templates"}</h1>
                        <p class="text-lg mb-4">
                            {"At Rusty Templates, we are dedicated to providing high-quality templates for Rust developers."}
                        </p>
                        <p class="text-lg mb-4">
                            {"Founded in 2024, our mission is to promote the development of web applications that are secure, reliable, and capable of delivering exceptional performance."}
                        </p>

                        <h2 class="text-2xl font-semibold mt-8 mb-4">{"Our Vision"}</h2>
                        <p class="text-lg mb-4">
                            {"Our vision is to explore and innovate in areas of the web that have been hindered by the limitations of current programming languages, particularly where performance is a concern."}
                        </p>
                        <p class="text-lg">
                            {"Join us on our journey!"}
                        </p>

                        // <div class="mt-8">
                        //     <BasicButton button_text="Explore Our Templates" />
                        // </div>
                    </div>

                    // Founders Info Column
                    <div class="overflow-y-auto max-h-[calc(100vh-200px)] border border-gray-300 rounded p-4">
                        <h2 class="text-2xl font-semibold mb-4">{"Meet The Team"}</h2>
                        <div class="space-y-6">
                        <div class="bg-white shadow-lg rounded flex-1 flex flex-row">
                            <div class="p-6 basis-2/3">
                                <h3 class="text-xl font-semibold text-gray-800">{"Elon Aseneka Idiong'o"}</h3>
                                <p class="text-gray-400 font-semibold">{"Founder"}</p>
                                <div class="max-h-24 overflow-y-auto">
                                    <p>{"I specialize in Web Technologies and I have a strong background in Rust programming."}</p>
                                    <p>{"While starting out as a Rust developer, I wanted to hit the ground running with my IoT idea but I couldn't find a marketplace for Rust templates and that is how Rusty Templates was born."}</p>
                                </div>
                            </div>
                            <div class="basis-1/3">
                                <img src={format!("{}{}", view_file_uri, (*dp_file).clone())} alt="mission" class="w-full h-full object-cover mb-2 rounded-r" />
                            </div>
                        </div>
                        </div>
                    </div>
                </div>
                </div>
            </div>
        </>
    }
}

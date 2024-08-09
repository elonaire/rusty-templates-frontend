use crate::{
    app::AppStateContext,
    components::{loading_spinner::LoadingSpinner, nav::top_nav::TopNav},
    data::{
        context::{products::serve_md_file, users::get_new_token},
        graphql::api_call::GraphQLResponse,
    },
};
use yew::prelude::*;

#[function_component]
pub fn FAQs() -> Html {
    let loading = use_state_eq(|| false);
    let current_state = use_context::<AppStateContext>().unwrap();
    let content = use_state_eq(|| String::new());

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    let content_clone = content.clone();
    use_effect_with_deps(
        move |_| {
            loading_clone.set(true);
            let tos_file = option_env!("FAQS_FILE").expect("FAQS_FILE env var not set");
            wasm_bindgen_futures::spawn_local(async move {
                if current_state_clone.auth_details.token.is_empty() {
                    let _new_token = get_new_token(&current_state_clone).await;
                }

                let tos_response = serve_md_file(tos_file.to_string()).await;

                match &tos_response {
                    GraphQLResponse::Data(data) => {
                        content_clone.set(data.serve_md_files.clone());
                        loading_clone.set(false);
                    }
                    GraphQLResponse::Error(_e) => {
                        // error_clone.set(tos_response.get_error().clone());
                        loading_clone.set(false);
                    }
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
                    {
                        if *loading {
                            html!{ <LoadingSpinner /> }
                        } else { html!{} }
                    }
                    <div class="md-body">
                        { Html::from_html_unchecked((*content).clone().into()) }
                    </div>
                </div>
            </div>
        </>
    }
}

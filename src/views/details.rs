use web_sys::window;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{
    app::{AppStateContext, Route, StateAction},
    components::{
        button::BasicButton, forms::radio_input::RadioInputField, loading_spinner::LoadingSpinner,
        nav::top_nav::TopNav,
    },
    data::{
        context::{
            orders::{
                add_to_cart, get_cart, get_licenses, get_product_total_sales, get_raw_cart_products,
            },
            products::get_product_by_slug,
            users::get_new_token,
        },
        graphql::api_call::GraphQLResponse,
        models::order::{CartOperation, UpdateCartPayload},
    },
};

#[derive(Clone, PartialEq, Eq, Debug, Properties)]
pub struct TemplateDetailsProps {
    pub id: String,
}

#[function_component]
pub fn TemplateDetails(props: &TemplateDetailsProps) -> Html {
    let selected_license = use_state(|| "".to_string());
    let current_state = use_context::<AppStateContext>().unwrap();
    let loading = use_state_eq(|| false);
    let total_sales = use_state_eq(|| 0);
    let navigator = use_navigator().unwrap();
    let view_file_uri =
        option_env!("FILES_SERVICE_VIEW_URL").expect("FILES_SERVICE_VIEW_URL env var not set");

    let on_license_change = {
        let selected_license = selected_license.clone();
        let current_state_clone = current_state.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value_clone = input.value();
            let current_state_clone = current_state_clone.clone();
            selected_license.set(value_clone.clone());
            wasm_bindgen_futures::spawn_local(async move {
                let payload = UpdateCartPayload {
                    external_product_id: current_state_clone
                        .current_product_details
                        .id
                        .clone()
                        .unwrap(),
                    cart_operation: CartOperation::AddProduct,
                    external_license_id: value_clone.clone(),
                };
                let _add_to_cart = add_to_cart(&current_state_clone, payload).await;
                // navigator_clone.push(&Route::Cart);
            });
        })
    };

    let onclick_buy = {
        let navigator_clone = navigator.clone();
        Callback::from(move |_e: MouseEvent| {
            navigator_clone.push(&Route::Cart);
        })
    };

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    let props_clone = props.clone();
    let total_sales_clone = total_sales.clone();
    use_effect_with_deps(
        move |_| {
            loading_clone.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let product_details = get_product_by_slug(&props_clone.id).await;

                match &product_details {
                    GraphQLResponse::Data(data) => {
                        let total_sales =
                            get_product_total_sales(data.get_product_by_slug.id.clone().unwrap())
                                .await;

                        // total_sales_clone
                        //     .set(total_sales.get_data().unwrap().get_product_total_sales);
                        // current_state_clone.dispatch(StateAction::UpdateCurrentProductDetails(
                        //     data.get_product_by_slug.clone(),
                        // ));
                        match total_sales.get_data() {
                            Some(sales) => {
                                total_sales_clone.set(sales.get_product_total_sales);
                            }
                            None => {
                                total_sales_clone.set(0);
                            }
                        }

                        current_state_clone.dispatch(StateAction::UpdateCurrentProductDetails(
                            data.get_product_by_slug.clone(),
                        ));
                    }
                    GraphQLResponse::Error(_e) => {
                        // error_clone.set(product_details.get_error().clone());
                    }
                }

                if current_state_clone.auth_details.token.is_empty() {
                    let _new_token = get_new_token(&current_state_clone).await;
                }

                if current_state_clone.licenses.is_empty() {
                    let _licenses = get_licenses(&current_state_clone).await;
                }

                if current_state_clone.cart.id.is_none() {
                    let _cart = get_cart(&current_state_clone).await;
                }

                if current_state_clone.cart.id.is_some() {
                    let _raw_products = get_raw_cart_products(
                        &current_state_clone,
                        current_state_clone.cart.id.clone().unwrap(),
                    )
                    .await;
                }

                loading_clone.set(false);
            });
        },
        (),
    );

    let current_state_clone_update = current_state.clone();
    let loading_clone = loading.clone();
    let selected_license_clone = selected_license.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading_clone.set(true);

                let selected_license_clone = selected_license_clone.clone();
                if current_state_clone_update
                    .current_product_details
                    .id
                    .is_some()
                    && current_state_clone_update.raw_cart_products.len() > 0
                {
                    if let Some(matched_product) = current_state_clone_update
                        .raw_cart_products
                        .iter()
                        .find(|p| {
                            p.ext_product_id
                                == current_state_clone_update
                                    .current_product_details
                                    .id
                                    .clone()
                                    .unwrap()
                        })
                    {
                        selected_license_clone.set(matched_product.license.clone());
                    }
                }
                loading_clone.set(false);
            });
        },
        current_state.clone(),
    );

    fn navigate_to_url_in_new_tab(url: &str) {
        if let Some(win) = window() {
            // Open the URL in a new tab
            win.open_with_url_and_target(url, "_blank")
                .expect("Failed to open new tab");
        }
    }

    let on_click_preview = {
        Callback::from(move |url: String| {
            Callback::from(move |_| {
                navigate_to_url_in_new_tab(url.as_str());
            })
        })
    };

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
                    <h1 class="text-2xl font-bold my-2">{current_state.current_product_details.name.clone()}</h1>
                    <div class="grid sm:grid-cols-1 md:grid-cols-8 gap-4">
                        // Screenshot Section
                        <div class="md:col-span-5 overflow-y-auto">
                            <div>
                                <img src={format!("{}{}", view_file_uri, current_state.current_product_details.screenshot.clone().unwrap_or("".into()))} alt="Template Screenshot" class="w-full h-auto object-cover rounded" />
                                // <p><strong>{"Application Layer: "}</strong>{current_state.current_product_details.application_layer.clone()}</p>
                                // <p><strong>{"Framework: "}</strong>{current_state.current_product_details.framework.clone()}</p>
                                // <p><strong>{"UI Framework: "}</strong>{current_state.current_product_details.ui_framework.clone()}</p>
                                <div class="mt-2 p-2 md-body">
                                    { Html::from_html_unchecked(current_state.current_product_details.product_details.clone().unwrap_or("".into()).into()) }
                                </div>
                            </div>
                        </div>

                        // License and Actions Section
                        <div class="md:col-span-3 flex flex-col gap-4">
                            <div class="border p-4 rounded shadow-md">
                                <h3 class="text-lg font-semibold mb-2">{ "Choose License: " }<span class="font-normal text-sm underline text-primary"><Link<Route> classes={"transition hover:text-gray-400"} to={Route::TermsOfService}>{"Read More"}</Link<Route>></span></h3>

                                <div class="flex flex-col justify-center space-y-2">
                                    {
                                        current_state.licenses.iter().map(|license|
                                            html!{
                                                <RadioInputField input_style_ext={"text-primary focus:border-primary focus:ring-indigo-200"} oninput={on_license_change.clone()} initial_value={license.id.clone()} id_attr={format!("license{}", license.name.clone())} label={license.short_description.clone()} name={"license"}>
                                                    <p class="font-bold text-xl">{format!("${}", current_state.current_product_details.price.clone().unwrap() * license.price_factor)}</p>
                                                </RadioInputField>
                                            }
                                        ).collect::<Html>()
                                    }
                                </div>
                            </div>

                            <BasicButton
                                button_text={"Buy Now".to_string()}
                                style_ext={"bg-primary text-white px-4 py-2 text-sm transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                                onclick={onclick_buy}
                                disabled={selected_license.is_empty()}
                            />

                            <BasicButton
                                button_text={"Live Preview".to_string()}
                                style_ext={"bg-secondary text-white px-4 py-2 text-sm hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                                onclick={on_click_preview.emit(current_state.current_product_details.preview_link.clone().unwrap_or("".to_string()))}
                            />

                            <div class="flex flex-row">
                                <div class="flex flex-row items-center gap-4 text-lg text-gray-400 font-semibold">
                                    <span>
                                        <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                                    </span>
                                    <p>{format!("{} Sales", (*total_sales))}</p>
                                </div>
                                <div></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

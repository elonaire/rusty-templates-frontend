use yew::prelude::*;
use crate::{app::{AppStateContext, Route, StateAction}, components::{button::BasicButton, loading_spinner::LoadingSpinner, card::Card, forms::{input::{InputField, InputFieldType}, select::{SelectInput, SelectOption}}, nav::top_nav::TopNav, wizards::stepper::{Step, Stepper}}, data::{context::{orders::{checkout, get_cart, get_product_external_ids}, products::{get_products, get_products_by_ids}, users::get_new_token}, models::{order::{CartTotals, CheckoutPayload}, user::AuthDetails}}, utils::auth_interceptor::retrieve_new_token, views::landing::{TemplateCardProps, TemplatesListProps}};
use web_sys::window;
use yew_router::prelude::*;

#[function_component]
pub fn CartPage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let current_state_for_changes = current_state.clone();
    let cart_totals = use_state_eq(|| CartTotals::default().calculate());
    let navigator = use_navigator().unwrap();
    let loading = use_state_eq(|| false);

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading_clone.set(true);
                if current_state_clone.auth_details.token.is_empty() {
                   let _new_token = get_new_token(&current_state_clone).await;
                }
                if current_state_clone.products.is_empty() {
                    let _products = get_products(&current_state_clone).await;
                }

                if current_state_clone.cart.id.is_none() {
                    let _cart = get_cart(&current_state_clone).await;
                }
                loading_clone.set(false);
            }); // Await the async block
            || ()
        },
        (),
    );

    let cart_totals_clone = cart_totals.clone();
    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(
        move |_| {
            loading_clone.set(true);
            let mut totals = CartTotals {
                subtotal: current_state_for_changes.cart.total_amount,
                ..*cart_totals_clone
            };

            cart_totals_clone.set(totals.calculate());
            // current_state_for_changes
            wasm_bindgen_futures::spawn_local(async move {
                if current_state_clone.cart.id.is_some() {
                    let _cart_product_ids = get_product_external_ids(&current_state_clone).await;
                }

                if current_state_clone.cart_products.len() == 0 {
                    let _cart_products = get_products_by_ids(&current_state_clone).await;
                }
                loading_clone.set(false);
            });
            || ()
        },
        (current_state.cart_products.clone(), current_state.cart.clone(), current_state.cart_products_ids.clone()),
    );

    let current_state_clone_checkout = current_state.clone();
    let navigator_clone = navigator.clone();
    let loading_clone = loading.clone();
    let on_checkout = {
        Callback::from(move |_| {
            log::info!("current_state_clone_checkout.auth_details.token: {}", current_state_clone_checkout.auth_details.token);
            if current_state_clone_checkout.auth_details.token.is_empty() {
                navigator_clone.push(&Route::SignIn);
            }
            loading_clone.set(true);
            let current_state_clone_checkout = current_state_clone_checkout.clone();
            let loading_clone = loading_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let payload = CheckoutPayload;
                let _add_to_cart = checkout(&current_state_clone_checkout, payload).await;
                loading_clone.set(false);
            });
        })
    };

    let current_state_clone_navigation = current_state.clone();
    use_effect_with_deps(
        move |_| {
            if !current_state_clone_navigation.checkout_url.is_empty() {
               navigate_to_url(&current_state_clone_navigation.checkout_url);
            }
            || ()
        },
        current_state.checkout_url.clone(),
    );

    fn navigate_to_url(url: &str) {
        if let Some(window) = window() {
            window
                .location()
                .set_href(url)
                .expect("Failed to navigate");
        }
    }

    html! {
        <>
            <div class="bg-gray-100 min-h-svh font-jost-sans">
                <TopNav />
                <div class="container mx-auto py-10">
                    <div class="grid sm:grid-cols-1 md:grid-cols-8 mt-4 items-center justify-center">
                        {
                            if *loading {
                                html!{ <LoadingSpinner /> }
                            } else { html!{} }
                        }
                        <div class="md:col-span-5 h-full">
                            <Card theme={"w-full h-full"}>
                                <Stepper indicator_no_theme_ext={"bg-primary"} indicator_text_theme_ext={"text-primary"} button_theme_ext={"bg-primary"} on_click_final_button={on_checkout} final_button_text={"Checkout"} steps_titles={vec!["Billing Details".to_string(), "Billing Address".to_string(), "Preview".to_string()]}>
                                    <Step>
                                        <div class="grid grid-cols-1 gap-4 w-full">
                                            <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
                                                <InputField ext_input_styles={""} required={true} label={"First Name"} name={"first_name"} field_type={InputFieldType::Text} />
                                                <InputField label={"Middle Name"} name={"middle_name"} field_type={InputFieldType::Text} />
                                                <InputField required={true} label={"Last Name"} name={"last_name"} field_type={InputFieldType::Text} />
                                            </div>
                                            <InputField required={true} label={"Email"} name={"email"} field_type={InputFieldType::Email} />
                                        </div>
                                    </Step>
                                    <Step>
                                        <div class="grid grid-cols-6 gap-2 w-full">
                                            <div class="col-span-6">
                                                <InputField required={true} label={"Country"} name={"card_number"} field_type={InputFieldType::Text} />
                                            </div>
                                        </div>
                                    </Step>
                                    <Step>
                                        <div>
                                            <table class="border-collapse w-full">
                                                <tr>
                                                    <td class="border border-transparent p-2 font-semibold">{"First Name"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.subtotal)}</td>
                                                    <td class="border border-transparent p-2 font-semibold">{"Middle Name"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.subtotal)}</td>
                                                </tr>
                                                <tr>
                                                    <td class="border border-transparent p-2 font-semibold">{"Last Name"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.vat)}</td>
                                                    <td class="border border-transparent p-2 font-semibold">{"Email"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.subtotal)}</td>
                                                </tr>
                                                <tr>
                                                    <td class="border border-transparent p-2 font-semibold">{"Country"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.total)}</td>
                                                    <td class="border border-transparent p-2 font-semibold">{"Address"}</td>
                                                    <td class="border border-transparent p-2">{format!("{}", cart_totals.subtotal)}</td>
                                                </tr>
                                            </table>
                                        </div>
                                    </Step>
                                </Stepper>
                            </Card>
                        </div>
                        <div class="md:col-span-3">
                            <div class="grid grid-cols-1 w-full p-2">
                                <table class="border-collapse w-full">
                                    <thead>
                                        <tr>
                                            <th class="border border-transparent p-2 text-left">{"Template"}</th>
                                            <th class="border border-transparent p-2 text-left">{"Price"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>

                                        {
                                            current_state.cart_products.iter().map(|product| {
                                                html! {
                                                    <tr>
                                                        <td class="border border-transparent p-2">{format!("{} ", product.name.clone().unwrap())}<span>{"\u{00D7} 1"}</span></td>
                                                        <td class="border border-transparent p-2">{format!("${}", product.price.clone().unwrap())}</td>
                                                    </tr>
                                                }
                                            }).collect::<Html>()
                                        }
                                        <tr class="border-t-2 border-secondary">
                                            <td class="border border-transparent p-2 pt-6 font-semibold">{"Subtotal"}</td>
                                            <td class="border border-transparent p-2 pt-6">{format!("${:.2}", cart_totals.subtotal)}</td>
                                        </tr>
                                        <tr>
                                            <td class="border border-transparent p-2 font-semibold">{"VAT"}</td>
                                            <td class="border border-transparent p-2">{format!("${:.2}", cart_totals.vat)}</td>
                                        </tr>
                                        <tr>
                                            <td class="border border-transparent p-2 font-semibold">{"Total"}</td>
                                            <td class="border border-transparent p-2">{format!("${:.2}", cart_totals.total)}</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

use yew::prelude::*;
use crate::{app::{AppStateContext, StateAction, Route}, components::{button::BasicButton, card::Card, forms::{input::{InputField, InputFieldType}, select::{SelectInput, SelectOption}}, nav::top_nav::TopNav, wizards::stepper::{Step, Stepper}}, data::{context::{orders::checkout, products::get_products}, models::order::{CartTotals, CheckoutPayload}}, views::landing::{TemplateCardProps, TemplatesListProps}};
use web_sys::window;
use yew_router::prelude::*;

#[function_component]
pub fn CartPage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let current_state_for_changes = current_state.clone();
    let cart_totals = use_state_eq(|| CartTotals::default().calculate());
    let navigator = use_navigator().unwrap();

    let current_state_clone = current_state.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {

                if current_state_clone.products.is_empty() {
                    let _products = get_products(&current_state_clone).await;
                }
            }); // Await the async block
            || ()
        },
        (),
    );

    let cart_totals_clone = cart_totals.clone();
    use_effect_with_deps(
        move |_| {
            let mut totals = CartTotals {
                subtotal: current_state_for_changes.cart.total_amount,
                ..*cart_totals_clone
            };

            cart_totals_clone.set(totals.calculate());
            // current_state_for_changes
            || ()
        },
        current_state.cart_products.clone(),
    );

    let current_state_clone_checkout = current_state.clone();
    let navigator_clone = navigator.clone();
    let on_checkout = {
        Callback::from(move |_| {
            if current_state_clone_checkout.auth_details.token.is_empty() {
                navigator_clone.push(&Route::SignIn);
            }
            let current_state_clone_checkout = current_state_clone_checkout.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let payload = CheckoutPayload;
                let _add_to_cart = checkout(&current_state_clone_checkout, payload).await;

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
        log::info!("{}", url);
        if let Some(window) = window() {
            window
                .location()
                .set_href(url)
                .expect("Failed to navigate");
        }
    }

    html! {
        <>
            <TopNav />
            <div class="flex mt-4 items-center justify-center flex-1">
                <Card theme={"w-[800px]"}>
                    <Stepper indicator_no_theme_ext={"bg-primary"} indicator_text_theme_ext={"text-primary"} button_theme_ext={"bg-primary"} on_click_final_button={on_checkout} final_button_text={"Checkout"} steps_titles={vec!["Billing Details".to_string(), "Billing Address".to_string(), "Preview".to_string()]}>
                        <Step>
                            <div class="grid grid-cols-1 gap-4 w-full">
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
                                    <InputField required={true} label={"First Name"} name={"first_name"} field_type={InputFieldType::Text} />
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
                            <div class="grid grid-cols-1 w-full">
                            <table class="border-collapse w-full">
                                <thead>
                                    <tr>
                                        <th class="border border-transparent p-2 text-left">{"Product"}</th>
                                        <th class="border border-transparent p-2 text-left">{"Subtotal"}</th>
                                    </tr>
                                </thead>
                                <tbody>

                                    {
                                        current_state.cart_products.iter().map(|product| {
                                            html! {
                                                <tr>
                                                    <td class="border border-transparent p-2">{product.name.clone().unwrap()}</td>
                                                    <td class="border border-transparent p-2">{format!("${}", product.price.clone().unwrap())}</td>
                                                </tr>
                                            }
                                        }).collect::<Html>()
                                    }
                                    <tr>
                                        <td class="border border-transparent p-2 font-semibold">{"Subtotal"}</td>
                                        <td class="border border-transparent p-2">{format!("${:.2}", cart_totals.subtotal)}</td>
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
                        </Step>
                    </Stepper>
                </Card>
            </div>
        </>
    }
}

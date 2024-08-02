use yew::prelude::*;
use yew_router::prelude::*;
use crate::{app::{AppRoute, AppStateContext, Route, StateAction, TemplateRoute}, components::{button::BasicButton, forms::select::{SelectInput, SelectOption}, loading_spinner::LoadingSpinner, nav::top_nav::TopNav}, data::{context::{orders::{add_to_cart, get_cart, get_product_external_ids}, products::{get_products, get_products_by_ids}, users::get_new_token}, models::order::{CartOperation, UpdateCartPayload}}, views::landing::{TemplateCardProps, TemplatesListProps}};

#[function_component]
pub fn StorePage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let loading = use_state_eq(|| false);

    let current_state_clone = current_state.clone();
    let loading_clone = loading.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading_clone.set(true);
                if current_state_clone.products.is_empty() {
                    let _products = get_products(&current_state_clone).await;
                }

                if current_state_clone.auth_details.token.is_empty() {
                   let _new_token = get_new_token(&current_state_clone).await;
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

    // let current_state_clone_update = current_state.clone();
    // let loading_clone = loading.clone();
    // use_effect_with_deps(move |_| {
    //     wasm_bindgen_futures::spawn_local(async move {
    //         loading_clone.set(true);
    //         if current_state_clone_update.cart.id.is_some() {
    //             let _cart_product_ids = get_product_external_ids(&current_state_clone_update).await;
    //         }

    //         if current_state_clone_update.cart_products.len() == 0 {
    //             let _cart_products = get_products_by_ids(&current_state_clone_update).await;
    //         }
    //         loading_clone.set(false);
    //     });

    // }, current_state.clone());


    html! {
        <>
            <div class="bg-gray-100 min-h-svh font-jost-sans">
                <TopNav />
                <main class="container mx-auto py-10">
                    <div class="flex">
                        {
                            if *loading {
                                html!{ <LoadingSpinner /> }
                            } else { html!{} }
                        }
                        // Left sidebar for filters
                        <div class="w-1/4">
                            <FilterMenu />
                        </div>
                        // Right main content for product listing
                        <div class="w-3/4 p-4">
                        <TemplatesList templates={current_state.products.to_vec()} />
                        </div>
                    </div>
                </main>
            </div>
        </>
    }
}

#[function_component]
pub fn FilterMenu() -> Html {
    let categories = use_state_eq(|| vec![] as Vec<SelectOption>);

    html! {
        <div class="p-6">
            <h2 class="text-xl font-semibold mb-4">{"Filters"}</h2>
            <div class="space-y-2">
                <div>
                    <SelectInput label={"Select a Category"} options={categories.to_vec()} name={"category"} />
                </div>
                <div>
                    <label class="block text-gray-700">{"Price Range"}</label>
                    <input type="range" class="block w-full mt-1" min="0" max="1000"/>
                </div>
                <div>
                    <SelectInput label={"Select Rating(min)"} options={categories.to_vec()} name={"rating"} />
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplateCard(props: &TemplateCardProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let navigator = use_navigator().unwrap();

    let view_file_uri = option_env!("FILES_SERVICE_VIEW_URL").expect("FILES_SERVICE_VIEW_URL env var not set");

    let navigator_clone = navigator.clone();
    let onclick_details = {
        let current_state_clone = current_state.clone();
        let product_clone = props.product.clone();
        Callback::from(move |_| {
            current_state_clone.dispatch(StateAction::UpdateCurrentProductDetails(product_clone.clone()));
            navigator_clone.push(&TemplateRoute::TemplateDetails { id: product_clone.slug.clone().unwrap() });
        })
    };

    html! {
        <div class="bg-white shadow-md rounded">
        <img src={format!("{}{}", view_file_uri, props.product.screenshot.clone().unwrap_or("".into()))} alt={props.product.name.clone()} class="w-full h-auto object-cover mb-4 rounded-t" />
            <div class="grid grid-cols-1 p-2">
                <div class="flex flex-row justify-between mb-2 min-h-16">
                    <p class="text-lg font-semibold line-clamp-2">{&props.product.name.clone().unwrap()}</p>
                    <p class="text-lg font-semibold">{format!("${}", props.product.price.unwrap())}</p>
                </div>
                <div class="flex flex-row items-center justify-between gap-2">
                    <BasicButton onclick={onclick_details} button_text={"Details"} style_ext={"px-4 py-2 text-sm border-2 border-primary text-primary hover:bg-primary hover:text-white transition w-full"} />
                    <BasicButton onclick={Callback::noop()} button_text={"Preview"} style_ext={"px-4 py-2 text-sm border-2 border-secondary text-secondary hover:bg-secondary hover:text-white transition w-full"} />
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {for templates.into_iter().map(|template| html! {
                    <TemplateCard product={template.clone()} />
                })}
            </div>
        </section>
    }
}

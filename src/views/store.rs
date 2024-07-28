use yew::prelude::*;
use yew_router::prelude::*;
use crate::{app::{AppRoute, AppStateContext, Route, StateAction}, components::{button::BasicButton, forms::select::{SelectInput, SelectOption}, nav::top_nav::TopNav}, data::{context::{orders::add_to_cart, products::get_products}, models::order::{CartOperation, UpdateCartPayload}}, views::landing::{TemplateCardProps, TemplatesListProps}};

#[function_component]
pub fn StorePage() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();

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

    html! {
        <>
            <TopNav />
            <div class="flex mt-4">
                // Left sidebar for filters
                <div class="w-1/4">
                    <FilterMenu />
                </div>
                // Right main content for product listing
                <div class="w-3/4 p-4">
                <TemplatesList templates={current_state.products.to_vec()} />
                </div>
            </div>
        </>
    }
}

#[function_component]
pub fn FilterMenu() -> Html {
    let categories = use_state_eq(|| vec![] as Vec<SelectOption>);

    html! {
        <div class="max-w-[1200px] p-6">
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

    let onclick_buy = {
        let current_state_clone = current_state.clone();
        let product_clone = props.product.clone();
        Callback::from(move |_| {
            // current_state_clone.dispatch(StateAction::UpdateCart(product_clone.clone()));
            let current_state_clone = current_state_clone.clone();
            let product_clone = product_clone.clone();
            let navigator_clone = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let payload = UpdateCartPayload {
                    external_product_id: product_clone.id.unwrap(),
                    cart_operation: CartOperation::AddProduct
                };
                let _add_to_cart = add_to_cart(&current_state_clone, payload).await;
                navigator_clone.push(&Route::Cart);
            });
        })
    };

    html! {
        <div class="bg-white shadow-md rounded">
        <img src={props.product.screenshot.clone()} alt={props.product.name.clone()} class="w-full h-44 object-cover mb-4 rounded-t" />
            <div class="p-2">
                <div class="flex flex-row items-center justify-between mb-2">
                    <h3 class="text-lg font-semibold">{&props.product.name.clone().unwrap()}</h3>
                    <p class="text-lg font-semibold">{format!("${}", props.product.price.unwrap())}</p>
                </div>
                <BasicButton onclick={onclick_buy} button_text={"Buy Now"} style_ext={"bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition"} />
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                {for templates.into_iter().map(|template| html! {
                    <TemplateCard product={template.clone()} />
                })}
            </div>
        </section>
    }
}

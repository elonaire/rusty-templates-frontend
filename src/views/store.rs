use yew::prelude::*;
use crate::{app::AppStateContext, components::nav::top_nav::TopNav, data::context::products::get_products, views::landing::{TemplateCardProps, TemplatesListProps}};

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
    html! {
        <div class="p-4 space-y-4">
            <h2 class="text-xl font-semibold mb-4">{"Filters"}</h2>
            <div class="space-y-2">
                <div>
                    <label class="block text-gray-700">{"Category"}</label>
                    <select class="block w-full mt-1 rounded-md border-gray-300 shadow-sm">
                        <option>{"All"}</option>
                        <option>{"Category 1"}</option>
                        <option>{"Category 2"}</option>
                    </select>
                </div>
                <div>
                    <label class="block text-gray-700">{"Price Range"}</label>
                    <input type="range" class="block w-full mt-1" min="0" max="1000"/>
                </div>
                <div>
                    <label class="block text-gray-700">{"Rating"}</label>
                    <select class="block w-full mt-1 rounded-md border-gray-300 shadow-sm">
                        <option>{"All"}</option>
                        <option>{"1 star"}</option>
                        <option>{"2 stars"}</option>
                        <option>{"3 stars"}</option>
                        <option>{"4 stars"}</option>
                        <option>{"5 stars"}</option>
                    </select>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplateCard(props: &TemplateCardProps) -> Html {
    html! {
        <div class="bg-white shadow-md rounded">
        <img src={props.product.screenshot.clone()} alt={props.product.name.clone()} class="w-full h-32 object-cover mb-4 rounded-t" />
            <div class="p-2">
                <h3 class="text-lg font-semibold mb-2">{props.product.name.clone()}</h3>
                // <p class="text-gray-700 mb-4 text-xs">{&props.description}</p>
                <button class="bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition">{"Buy Now"}</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="">
            // <h2 class="text-3xl font-bold text-center mb-6">{"Templates"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
                {for templates.into_iter().map(|template| html! {
                    <TemplateCard product={template.clone()} />
                })}
            </div>
        </section>
    }
}

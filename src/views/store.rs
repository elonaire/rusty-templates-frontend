use yew::prelude::*;
use crate::views::landing::{TemplateProps, TemplatesListProps};

#[function_component]
pub fn StorePage() -> Html {
    let templates = vec![
        TemplateProps {
            title: "Product 1".to_string(),
            description: "Description for product 1".to_string(),
            price: "$19.99".to_string(),
            image: "path_to_image_1.jpg".to_string(),
        },
        TemplateProps {
            title: "Product 2".to_string(),
            description: "Description for product 2".to_string(),
            price: "$29.99".to_string(),
            image: "path_to_image_2.jpg".to_string(),
        },
        // Add more products as needed
    ];

    html! {
        <div class="flex">
            // Left sidebar for filters
            <div class="w-1/4">
                <FilterMenu />
            </div>
            // Right main content for product listing
            <div class="w-3/4 p-4">
                <TemplatesList templates={templates} />
            </div>
        </div>
    }
}

#[function_component]
pub fn FilterMenu() -> Html {
    html! {
        <div class="bg-gray-100 p-4 space-y-4">
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
pub fn TemplateCard(props: &TemplateProps) -> Html {
    html! {
        <div class="bg-white shadow-md rounded">
            <img src={props.image.clone()} alt={props.title.clone()} class="w-full h-32 object-cover mb-4 rounded-t" />
            <div class="p-2">
                <h3 class="text-lg font-semibold mb-2">{&props.title}</h3>
                <p class="text-gray-700 mb-4 text-xs">{&props.description}</p>
                <button class="bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition">{"Buy Now"}</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="py-10">
            <h2 class="text-3xl font-bold text-center mb-6">{"Featured Templates"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
                {for templates.into_iter().map(|template| html! {
                    <TemplateCard title={template.title.clone()} description={template.description.clone()} image={template.image.clone()} price={template.price.clone()} />
                })}
            </div>
        </section>
    }
}

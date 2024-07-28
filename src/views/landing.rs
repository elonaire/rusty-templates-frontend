use chrono::Utc;
use yew::prelude::*;
use yew::function_component;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use crate::data::context::products::get_products;
use crate::data::models::template::Product;
use crate::{app::{Route, AppStateContext}, components::{nav::top_nav::TopNav, button::BasicButton}};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct TemplateCardProps {
   pub product: Product,
   // #[prop_or(Callback::noop())]
   // pub onclick_buy: Callback<MouseEvent>,
}

#[function_component]
pub fn Landing() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    // let products = use_state_eq(|| vec![] as Vec<Product>);

    let current_year = {
        let now = Utc::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.format("%Y").to_string()
    };

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

    // let products_state_clone = products.clone();
    let current_state_clone_update = current_state.clone();
    use_effect_with_deps(move |_| {
        log::info!("{:?}", current_state_clone_update.products);
        // products_state_clone.set(current_state_clone_update.products.clone());
    }, current_state.clone());

    html! {
        <>
            <div class="bg-gray-100 min-h-screen">
                <TopNav />
                <main class="container mx-auto py-10">
                    <Hero />
                    <TemplatesList templates={current_state.products.to_vec()} />
                    <ContactSection />
                </main>
                <footer class="bg-gray-800 text-white py-4">
                    <div class="container mx-auto text-center">
                        {format!("© {} Rusty Templates. All rights reserved.", current_year)}
                    </div>
                </footer>
            </div>
        </>
    }
}

#[function_component]
pub fn Hero() -> Html {
    html! {
        <section class="text-center py-20 bg-gray-300 rounded">
            <h1 class="text-5xl font-bold mb-4">{"Do You Really Want to Reinvent?"}</h1>
            <p class="text-lg mb-8">{"Browse our collection of awesome Rust templates. Web (Frontend and Backend), Mobile & Desktop."}</p>
            // <a href="#templates" class="bg-primary text-white px-6 py-2 rounded hover:bg-secondary transition">{"Shop Now"}</a>
            <Link<Route> classes={classes!("bg-primary", "text-white", "px-6", "py-2", "rounded", "hover:bg-secondary", "transition")} to={Route::Store}>{"Shop Now"}</Link<Route>>
        </section>
    }
}

#[function_component]
pub fn PopularTemplateCard(props: &TemplateCardProps) -> Html {
    let is_hovered = use_state(|| false);

    let on_mouse_over = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_mouse_out = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    let button_class = if *is_hovered {
        ""
    } else {
        "hidden"
    };

    html! {
        <div class="rounded">
            <div onmouseover={on_mouse_over} onmouseout={on_mouse_out} class="relative cursor-pointer">
                <img src={props.product.screenshot.clone().unwrap()} alt={props.product.name.clone().unwrap()} class="w-full h-72 object-cover mb-2 rounded" />
                <button class={format!("absolute bottom-2 right-2 bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition {}", button_class)}>{"Live Preview"}</button>
            </div>
            <div class="p-2">
                <div class="flex flex-row items-center justify-between mb-2">
                    <h3 class="text-lg font-semibold">{&props.product.name.clone().unwrap()}</h3>
                    <p class="text-lg font-semibold">{format!("${}", props.product.price.unwrap())}</p>
                </div>
                // <p class="text-gray-700 mb-4 text-sm">{&props.use_case.clone().unwrap()}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct TemplatesListProps {
    pub templates: Vec<Product>,
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="py-10">
            <h2 class="text-3xl font-bold text-center mb-6">{"Featured Templates"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                {for templates.into_iter().map(|template| html! {
                    <PopularTemplateCard product={template.clone()} />
                })}
            </div>
        </section>
    }
}

#[function_component]
pub fn ContactSection() -> Html {
    html! {
        <section id="contact" class="bg-white py-10 px-2">
            <h2 class="text-3xl font-bold text-center mb-6">{"Contact Us"}</h2>
            <form class="max-w-xl mx-auto">
                <div class="mb-4">
                    <label for="name" class="block text-gray-700">{"Name"}</label>
                    <input type="text" id="name" class="mt-1 p-2 block w-full border border-gray-300 rounded" />
                </div>
                <div class="mb-4">
                    <label for="email" class="block text-gray-700">{"Email"}</label>
                    <input type="email" id="email" class="mt-1 p-2 block w-full border border-gray-300 rounded" />
                </div>
                <div class="mb-4">
                    <label for="message" class="block text-gray-700">{"Message"}</label>
                    <textarea id="message" class="mt-1 p-2 block w-full border border-gray-300 rounded"></textarea>
                </div>
                // <button type="submit" class="bg-primary text-white px-4 py-2 rounded hover:bg-secondary transition">{"Send"}</button>
                <BasicButton button_text={"Send"} style_ext={"bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition"} />
            </form>
        </section>
    }
}

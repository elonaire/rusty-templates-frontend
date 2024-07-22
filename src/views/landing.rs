use chrono::Utc;
use yew::prelude::*;
use yew::function_component;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;
use crate::app::Route;

#[function_component]
pub fn Landing() -> Html {
    // Create state to manage the visibility of the sidemenu
    let is_menu_open = use_state(|| false);

    // Function to toggle the visibility of the menu
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let current_year = {
        let now = Utc::now();
        let datetime: chrono::DateTime<chrono::Utc> = now.into();
        datetime.format("%Y").to_string()
    };

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
        <>
            <div class="bg-gray-100 min-h-screen">
                <header class="bg-white shadow">
                    <nav class="container mx-auto p-4 flex justify-between items-center">
                        // <h1 class="text-2xl font-bold">{"Rusty Templates"}</h1>
                        <img class="w-24" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="logo" />
                <div class="hidden md:flex items-center">
                            <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>{"Home"}</Link<Route>>
                            <a href="#templates" class="text-gray-700 px-4">{"Templates"}</a>
                            <a href="#contact" class="text-gray-700 px-4">{"Contact"}</a>
                            <Link<Route> classes={classes!("text-gray-700", "px-4")} to={Route::Landing}>
                                <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                            </Link<Route>>
                        </div>
                        <button class="block md:hidden" onclick={toggle_menu.clone()}>
                            <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7"/>
                            </svg>
                        </button>
                    </nav>
                </header>
                <div class={format!("{} fixed inset-0 bg-gray-800 bg-opacity-75 z-50 p-6 flex flex-col space-y-4 {}",
                    if *is_menu_open { "flex" } else { "hidden" },
                    if *is_menu_open { "" } else { "hidden" })}>
                    <button class="self-end text-white" onclick={toggle_menu.clone()}>
                        <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                    <Link<Route> classes={classes!("text-white", "text-center", "text-xl")} to={Route::Landing}>{"Home"}</Link<Route>>
                    <a href="#templates" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Templates"}</a>
                    <a href="#contact" class="text-white text-center text-xl" onclick={toggle_menu.clone()}>{"Contact"}</a>
                    <Link<Route> classes={classes!("text-white", "text-center", "text-xl")} to={Route::Landing}>
                        <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCart3}/>
                    </Link<Route>>
                </div>
                <main class="container mx-auto py-10">
                    <Hero />
                    <TemplatesList templates={templates} />
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

#[derive(Properties, PartialEq, Clone)]
pub struct TemplateProps {
    pub title: String,
    pub description: String,
    pub image: String,
    pub price: String,
}

#[function_component]
pub fn PopularTemplateCard(props: &TemplateProps) -> Html {
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
                <img src={props.image.clone()} alt={props.title.clone()} class="w-full h-56 object-cover mb-2 rounded" />
                <button class={format!("absolute bottom-2 right-2 bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition {}", button_class)}>{"Live Preview"}</button>
            </div>
            <div class="p-2">
                <h3 class="text-lg font-semibold mb-2">{&props.title}</h3>
                <p class="text-gray-700 mb-4 text-sm">{&props.description}</p>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct TemplatesListProps {
    pub templates: Vec<TemplateProps>,
}

#[function_component]
pub fn TemplatesList(TemplatesListProps { templates }: &TemplatesListProps) -> Html {

    html! {
        <section id="templates" class="py-10">
            <h2 class="text-3xl font-bold text-center mb-6">{"Featured Templates"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                {for templates.into_iter().map(|template| html! {
                    <PopularTemplateCard title={template.title.clone()} description={template.description.clone()} image={template.image.clone()} price={template.price.clone()} />
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
                <button type="submit" class="bg-primary text-white px-4 py-2 rounded hover:bg-secondary transition">{"Send"}</button>
            </form>
        </section>
    }
}

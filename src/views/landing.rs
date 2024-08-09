use crate::app::StateAction;
use crate::app::TemplateRoute;
use crate::components::cookie_consent::CookieConsent;
use crate::data::context::orders::get_cart;
use crate::data::context::products::get_products;
use crate::data::context::users::get_new_token;
use crate::data::models::template::Product;
use crate::{
    app::{AppStateContext, Route},
    components::{button::BasicButton, nav::top_nav::TopNav},
};
use chrono::Utc;
use web_sys::window;
use yew::function_component;
use yew::prelude::*;
use yew_router::prelude::*;

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

                if current_state_clone.cart.id.is_none() {
                    let _cart = get_cart(&current_state_clone).await;
                }

                if current_state_clone.auth_details.token.is_empty() {
                    let _new_token = get_new_token(&current_state_clone).await;
                }
            }); // Await the async block
            || ()
        },
        (),
    );

    // let products_state_clone = products.clone();
    // let current_state_clone_update = current_state.clone();
    // use_effect_with_deps(move |_| {
    //     wasm_bindgen_futures::spawn_local(async move {
    //         if current_state_clone_update.cart.id.is_some() {
    //             let _cart_product_ids = get_product_external_ids(&current_state_clone_update).await;
    //         }

    //         if current_state_clone_update.cart_products.len() == 0 {
    //             let _cart_products = get_products_by_ids(&current_state_clone_update).await;
    //         }
    //     });

    // }, current_state.clone());

    html! {
        <>
            <div class="bg-gray-100 min-h-svh font-jost-sans">
                <TopNav />
                <main class="container mx-auto py-10 px-2">
                    <CookieConsent />
                    <Hero />
                    <TemplatesList templates={current_state.products.to_vec()} />
                    <WhyPurchaseTemplates />
                    <MissionVision />
                    // <ContactSection />
                </main>
                <footer class="bg-gray-800 text-white py-10">
                    <div class="container mx-auto text-center mb-6">
                        <h2 class="text-lg font-bold mb-4">{"Stay Connected"}</h2>
                        <div class="flex justify-center space-x-6 mb-6">
                            <a target="_blank" href="https://x.com/RustyTemplates" class="hover:text-gray-400">{"X"}</a>
                            <a target="_blank" href="https://www.instagram.com/rustytemplates/" class="hover:text-gray-400">{"Instagram"}</a>
                            <a target="_blank" href="https://www.linkedin.com/company/rusty-templates/?viewAsMember=true" class="hover:text-gray-400">{"LinkedIn"}</a>
                        </div>
                        <div class="flex justify-center space-x-6 mb-6">
                            <Link<Route> classes={"transition hover:text-gray-400"} to={Route::PrivacyPolicy}>{"Privacy Policy"}</Link<Route>>
                            <Link<Route> classes={"transition hover:text-gray-400"} to={Route::TermsOfService}>{"Terms of Service"}</Link<Route>>
                            <Link<Route> classes={"transition hover:text-gray-400"} to={Route::FAQs}>{"FAQs"}</Link<Route>>
                            <Link<Route> classes={"transition hover:text-gray-400"} to={Route::About}>{"About Us"}</Link<Route>>
                        </div>
                    </div>
                    <div class="container mx-auto text-center">
                        <p class="text-sm">
                            {format!("© {} Rusty Templates. All rights reserved.", current_year)}
                        </p>
                    </div>
                </footer>
            </div>
        </>
    }
}

#[function_component]
pub fn Hero() -> Html {
    let view_file_uri = option_env!("FILES_SERVICE_VIEW_PROD_URL")
        .expect("FILES_SERVICE_VIEW_PROD_URL env var not set");
    let file = use_state_eq(|| "e07c7891-f947-42d1-aa06-8e4e0d86d50f".to_string());
    let background_image = format!("background-image: url({}{})", view_file_uri, *file);

    html! {
        <section class="text-center py-20 bg-gray-300 rounded bg-no-repeat bg-cover bg-center" style={background_image}>
            <div class="bg-gray-300 py-6 opacity-90 h-full">
                <h1 class="text-5xl font-bold mb-4">{"Are you looking to start your next project in Rust?"}</h1>
                <p class="text-lg mb-8">{"Browse our collection of awesome Rust templates. Web (Frontend and Backend), dApps, Mobile & Desktop."}</p>
                // <a href="#templates" class="bg-primary text-white px-6 py-2 rounded hover:bg-secondary transition">{"Shop Now"}</a>
                <Link<Route> classes={"bg-primary text-white px-6 py-2 rounded hover:bg-secondary transition"} to={Route::Store}>{"Shop Now"}</Link<Route>>
            </div>
        </section>
    }
}

#[function_component]
pub fn PopularTemplateCard(props: &TemplateCardProps) -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let is_hovered = use_state(|| false);
    let view_file_uri =
        option_env!("FILES_SERVICE_VIEW_URL").expect("FILES_SERVICE_VIEW_URL env var not set");
    let navigator = use_navigator().unwrap();

    let on_mouse_over = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_mouse_out = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    let button_class = if *is_hovered { "" } else { "hidden" };

    let navigator_clone = navigator.clone();
    let onclick_details = {
        let current_state_clone = current_state.clone();
        let product_clone = props.product.clone();
        Callback::from(move |_| {
            current_state_clone.dispatch(StateAction::UpdateCurrentProductDetails(
                product_clone.clone(),
            ));
            navigator_clone.push(&TemplateRoute::TemplateDetails {
                id: product_clone.slug.clone().unwrap(),
            });
        })
    };

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
        <div class="rounded">
            <div onmouseover={on_mouse_over} onmouseout={on_mouse_out} class="relative cursor-pointer">
            <img onclick={onclick_details.clone()} src={format!("{}{}", view_file_uri, props.product.screenshot.clone().unwrap())} alt={props.product.name.clone().unwrap()} class="w-full h-auto object-cover mb-2 rounded" />
                // <button class={format!("absolute bottom-2 right-2 bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition {}", button_class)}>{"Live Preview"}</button>
                <BasicButton disabled={props.product.preview_link.clone().is_none()} onclick={on_click_preview.emit(props.product.preview_link.clone().unwrap_or("".to_string()))} button_text={"Live Preview"} style_ext={format!("absolute bottom-2 right-2 bg-primary text-white text-sm px-4 py-2 rounded hover:bg-secondary transition {}", button_class)} />
            </div>
            <div class="p-2">
                <div class="flex flex-row items-center justify-between mb-2">
                    <p onclick={onclick_details.clone()} class="text-lg font-semibold cursor-pointer line-clamp-2">{&props.product.name.clone().unwrap()}</p>
                    <p class="text-lg font-semibold">{format!("${}", props.product.price.unwrap())}</p>
                </div>
                <p class="text-gray-700 mb-4 text-sm">{format!("{}", &props.product.use_case.clone().unwrap())}</p>
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

#[function_component]
pub fn WhyPurchaseTemplates() -> Html {
    html! {
        <section class="py-16 bg-gray-100">
            <div class="container mx-auto text-center">
                <h2 class="text-3xl font-bold text-gray-800 mb-8">{"Why Purchase Our Templates?"}</h2>
                <div class="flex justify-around flex-wrap">
                    <div class="max-w-xs bg-white border rounded shadow-md p-6 m-4">
                        <h3 class="text-xl font-semibold text-gray-700 mb-4">{"Expertise in Rust"}</h3>
                        <p class="text-gray-600">{"We use Rust, therefore you are sure that we can help whenever you feel stuck."}</p>
                    </div>
                    <div class="max-w-xs bg-white border rounded shadow-md p-6 m-4">
                        <h3 class="text-xl font-semibold text-gray-700 mb-4">{"Data-Driven Design"}</h3>
                        <p class="text-gray-600">{"Our templates are designed with data in mind, therefore you spend less time refactoring, all you need to do is adjust the data models."}</p>
                    </div>
                    <div class="max-w-xs bg-white border rounded shadow-md p-6 m-4">
                        <h3 class="text-xl font-semibold text-gray-700 mb-4">{"Free Support & Updates"}</h3>
                        <p class="text-gray-600">{"We offer free support and updates for up to 1 year."}</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[function_component]
pub fn MissionVision() -> Html {
    let view_file_uri = option_env!("FILES_SERVICE_VIEW_PROD_URL")
        .expect("FILES_SERVICE_VIEW_PROD_URL env var not set");
    let mission_file = use_state_eq(|| "748d3d30-0aee-4336-9714-2a094ab65171".to_string());
    let vision_file = use_state_eq(|| "700573b3-c093-4414-ae2e-9a60d2b59bad".to_string());

    html! {
        <section class="py-20 bg-gray-200">
            <div class="container mx-auto text-center mb-10">
                <h2 class="text-3xl font-bold mb-6">{"Our Mission and Vision"}</h2>
                <p class="text-lg text-gray-600">{"Empowering developers to build the future."}</p>
            </div>
            // I want the divs in the div below to always have the same height
            <div class="flex flex-col md:flex-row justify-around container mx-auto">
                <div class="bg-white shadow-lg rounded m-4 flex-1 flex flex-row">
                    <div class="p-6 basis-2/3">
                        <h3 class="text-2xl font-semibold text-gray-800 mb-4">{"Our Mission"}</h3>
                        <p class="text-gray-600">{"Our mission is to promote the development of web applications that are secure, reliable, and capable of delivering exceptional performance."}</p>
                    </div>
                    <div class="basis-1/3">
                        <img src={format!("{}{}", view_file_uri, (*mission_file).clone())} alt="mission" class="w-full h-full object-cover mb-2 rounded-r" />
                    </div>
                </div>
                <div class="bg-white shadow-lg rounded m-4 flex-1 flex flex-row">
                    <div class="p-6 basis-2/3">
                        <h3 class="text-2xl font-semibold text-gray-800 mb-4">{"Our Vision"}</h3>
                        <p class="text-gray-600">{"Our vision is to explore and innovate in areas of the web that have been hindered by the limitations of current programming languages, particularly where performance is a concern."}</p>
                    </div>
                    <div class="basis-1/3">
                        <img src={format!("{}{}", view_file_uri, (*vision_file).clone())} alt="vision" class="w-full h-full object-cover mb-2 rounded-r" />
                    </div>
                </div>
            </div>
        </section>
    }
}

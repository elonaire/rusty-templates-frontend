use chrono::{Duration, Utc};
use gloo::utils::document;
use web_sys::{HtmlDocument, wasm_bindgen::JsCast};
use yew::prelude::*;

#[function_component(CookieConsent)]
pub fn cookie_consent() -> Html {
    let accepted = use_state(|| get_cookie("cookieConsent").unwrap_or_default() == "accepted");

    let accept = {
        let accepted = accepted.clone();
        Callback::from(move |_| {
            set_cookie("cookieConsent", "accepted", 365);
            accepted.set(true);
        })
    };

    let reject = {
        let accepted = accepted.clone();
        Callback::from(move |_| {
            set_cookie("cookieConsent", "rejected", 365);
            accepted.set(true);
        })
    };

    if *accepted {
        html! {}
    } else {
        html! {
            <div id="cookieConsentBanner" class="fixed bottom-0 left-0 right-0 bg-gray-800 text-white p-4 z-50">
                <div class="grid sm:grid-cols-1 md:grid-cols-12 gap-2 items-center justify-between">
                    <p class="md:col-span-8">{ "We use cookies to enhance your experience. By continuing to visit this site, you agree to our use of cookies. " }
                    <a href="/cookie-policy" class="underline text-blue-300 hover:text-blue-500">{ "Learn more" }</a>
                    { " or " }
                    <a href="/cookie-settings" class="underline text-blue-300 hover:text-blue-500">{ "manage your settings" }</a>
                    { "." }</p>
                    <div class="md:col-span-4 flex items-center justify-center space-x-2">
                        <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" onclick={accept}>{ "Accept" }</button>
                        <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" onclick={reject}>{ "Reject" }</button>
                    </div>
                </div>
            </div>
        }
    }
}

fn set_cookie(name: &str, value: &str, days: i32) {
    let document = document().unchecked_into::<HtmlDocument>();
    let expiration_date = Utc::now() + Duration::days(days.into());
    let expires = expiration_date.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let cookie = format!("{}={}; expires={}; path=/", name, value, expires);
    document.set_cookie(&cookie).unwrap();
}

fn get_cookie(name: &str) -> Option<String> {
    let document = document().unchecked_into::<HtmlDocument>();
    let cookies = document.cookie().unwrap();
    for cookie in cookies.split(';') {
        let parts: Vec<&str> = cookie.split('=').collect();
        if parts.len() == 2 && parts[0].trim() == name {
            return Some(parts[1].trim().to_string());
        }
    }
    None
}

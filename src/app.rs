use chrono::{TimeZone, Utc};
use gloo::timers::callback::Interval;
use std::time::Duration;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let target_date = Utc.with_ymd_and_hms(2024, 7, 14, 17, 0, 0).unwrap();
    let now = Utc::now();

    let remaining_duration = if target_date > now {
        (target_date - now).to_std().unwrap()
    } else {
        Duration::new(0, 0)
    };

    let time_left = use_state(|| remaining_duration);

    // Set interval to update countdown every second
    {
        let time_left = time_left.clone();
        use_effect_with_deps(
            move |_| {
                let interval = Interval::new(1000, move || {
                    let new_now = Utc::now();
                    let new_remaining_duration = if target_date > new_now {
                        (target_date - new_now).to_std().unwrap()
                    } else {
                        Duration::new(0, 0)
                    };
                    time_left.set(new_remaining_duration);
                });
                move || drop(interval)
            },
            (),
        );
    }

    let days = time_left.as_secs() / 86400;
    let hours = (time_left.as_secs() % 86400) / 3600;
    let minutes = (time_left.as_secs() % 3600) / 60;
    let seconds = time_left.as_secs() % 60;

    html! {
        <main class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-r from-blue-500 to-indigo-600 text-white">
            <img class="w-48 mb-8 animate-pulse text-gray-100" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/de1ec98f-ab67-41b2-75af-7b9cc7d35200/public" alt="RT logo" />
            <h1 class="text-5xl font-bold mb-4">{ "Almost there! It will be worth the wait!" }</h1>
            <div class="text-3xl font-medium mb-8 w-64 text-gray-100">
                { format!("{:02}d {:02}h {:02}m {:02}s", days, hours, minutes, seconds) }
            </div>
            <h2 class="text-3xl text-center max-w-lg text-gray-100 mt-2">{ "Your one-stop shop for easy-to-customize, intuitive and responsive web templates written in Rust!" }</h2>
            <h3 class="wave-text text-2xl text-center max-w-lg text-gray-150 mt-2">{ "There's no need to reinvent. Compile to Web Assembly and deploy!" }</h3>
            <p class="mt-4">{"If you are an interested creator, kindly reach out to us via info@rustytemplates.com."}</p>
        </main>
    }
}

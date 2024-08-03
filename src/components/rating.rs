use yew::prelude::*;

#[function_component]
pub fn Rating() -> Html {
    let rating = use_state(|| 0);
    let on_click = {
        let rating = rating.clone();
        Callback::from(move |value| {
            let rating = rating.clone();
            Callback::from(move |_| {
                rating.set(value);
            })
        })
    };

    let stars: Vec<_> = (0..5).map(|i| {
        let is_filled = *rating >= i + 1;
        let fill = if is_filled { "text-primary" } else { "text-gray-400" };

        html! {
            <svg
                class={format!("w-6 h-6 cursor-pointer {}", fill)}
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                // onmouseover={on_click.emit(i + 1)}
                onclick={on_click.emit(i + 1)}
            >
                <polygon points="12 2 15 8 22 9 17 14 18 21 12 17 6 21 7 14 2 9 9 8" />
            </svg>
        }
    }).collect();

    html! {
        <div class="flex space-x-1">
            { for stars }
        </div>
    }
}

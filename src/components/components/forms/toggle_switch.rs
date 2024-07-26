use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ToggleSwitchProps {
    pub active: bool,
    pub on_toggle: Callback<bool>,
    #[prop_or("On".to_string())]
    pub label_active: String,
    #[prop_or("Off".to_string())]
    pub label_inactive: String,
}

#[function_component]
pub fn ToggleSwitch(props: &ToggleSwitchProps) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_| {
            props.on_toggle.emit(!props.active);
        })
    };

    let switch_class = if props.active {
        "translate-x-full"
    } else {
        ""
    };

    let switch_bg = if props.active {
        "bg-blue-950"
    } else {
        "bg-gray-300"
    };

    html! {
        <div class="flex items-center cursor-pointer" onclick={onclick}>
            <div class="relative">
                <input type="checkbox" id="toggle-switch" class="sr-only"/>
                <div class={format!("block w-14 h-8 rounded-full {}", switch_bg)}></div>
                <div class={format!("dot absolute left-1 top-1 w-6 h-6 rounded-full transition transform {}", switch_class)}></div>
            </div>
            <div class="ml-3 text-gray-700 font-medium">
                { if props.active { props.label_active.clone() } else { props.label_inactive.clone() } }
            </div>
        </div>
    }
}

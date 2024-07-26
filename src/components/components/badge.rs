use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BadgeProps {
    pub text: String,
    #[prop_or("blue".to_string())]
    pub color: String,
    #[prop_or_default]
    pub parent_class: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub badge_position: String,
}

#[function_component]
pub fn Badge(props: &BadgeProps) -> Html {
    let color_classes = match props.color.as_str() {
        "red" => "bg-red-500",
        "green" => "bg-green-500",
        "yellow" => "bg-yellow-500",
        _ => "bg-blue-500", // default color
    };

    let width_classes = if props.text.is_empty() {
        "w-2 h-2"
    } else {
        "min-w-4 h-4 p-1"
    };

    html! {
        <div class={format!("relative {}", props.parent_class)}>
            { for props.children.iter() }
            <span class={format!("inline-flex items-center justify-center rounded-full text-xs font-medium text-white absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 {} {} {}", color_classes, width_classes, props.badge_position)}>
                { &props.text }
            </span>
        </div>
    }
}

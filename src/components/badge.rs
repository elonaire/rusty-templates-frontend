use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BadgeProps {
    pub text: String,
    #[prop_or("bg-blue-500".to_string())]
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
    let width_classes = if props.text.is_empty() {
        "w-2 h-2"
    } else {
        "min-w-4 h-4 p-1"
    };

    html! {
        <div class={format!("relative {}", props.parent_class)}>
            { for props.children.iter() }
            <span class={format!("inline-flex items-center justify-center rounded-full text-xs font-medium text-white absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 {} {} {}", &props.color, width_classes, props.badge_position)}>
                { &props.text }
            </span>
        </div>
    }
}

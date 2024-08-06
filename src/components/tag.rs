use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LabelTagProps {
    pub label: String,
    #[prop_or_default]
    pub color_class: String,
}

#[function_component()]
pub fn LabelTag(LabelTagProps { label, color_class }: &LabelTagProps) -> Html {
    // Function to return the corresponding tailwind classes
    let color_classes = match color_class.clone().as_str() {
        "success" => "text-green-600 border-2 border-green-600 bg-green-100".to_string(),
        "warning" => "text-yellow-600 border-2 border-yellow-600 bg-yellow-100".to_string(),
        "info" => "text-blue-600 border-2 border-blue-600 bg-blue-100".to_string(),
        "error" => "text-red-600 border-2 border-red-600 bg-red-100".to_string(),
        _ => "text-gray-600 border-2 border-gray-600 bg-gray-100".to_string(),
    };

    html! {
        <div class={format!("inline-block px-3 rounded {}", color_classes)}>
            { label }
        </div>
    }
}

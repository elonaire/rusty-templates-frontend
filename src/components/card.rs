use yew::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub theme: String,
    #[prop_or_default]
    pub title: String,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html! {
        <div class={format!("bg-slate-50 shadow-lg border-[1px] border-gray-200 rounded relative {}", props.theme)}>
            {
                if !props.title.is_empty() {
                    html! {
                        <div class="flex flex-row items-center gap-2 border-b-[1px] p-2">
                            <p class="text-lg font-bold text-gray-600">{ &props.title }</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            <div class="p-4 h-full">
                { props.children.clone() }
            </div>
        </div>
    }
}

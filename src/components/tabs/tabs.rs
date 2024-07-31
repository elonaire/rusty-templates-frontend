use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TabsProps {
    pub children: Children,
    #[prop_or_default]
    pub vertical: bool,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    html! {
        <div class={classes!(if props.vertical { "flex flex-row" } else { "flex flex-col" })}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabProps {
    pub label: String,
    pub children: Children,
}

#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    let onclick = Callback::from(|_| {
        // Implement your own logic for tab selection
    });

    html! {
        <div class="border rounded p-4 cursor-pointer" {onclick}>
            <div class="font-bold">{ &props.label }</div>
            <div>
                { for props.children.iter() }
            </div>
        </div>
    }
}

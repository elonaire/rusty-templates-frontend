use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct BasicButtonProps {
    #[prop_or_default]
    pub button_text: String,
    #[prop_or_default]
    pub style_ext: String,
    pub onclick: Option<Callback<MouseEvent>>,
    pub icon: Option<IconId>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or("button".to_string())]
    pub button_type: String,
    #[prop_or(true)]
    pub icon_before: bool,
}

#[function_component]
pub fn BasicButton(props: &BasicButtonProps) -> Html {
    let BasicButtonProps {
        style_ext,
        onclick,
        button_text,
        icon,
        disabled,
        button_type,
        icon_before,
    } = props;

    let button_content_styles = if button_text.is_empty() {
        ""
    } else {
        if *icon_before {
            "gap-2"
        } else {
            "gap-2 flex-row-reverse"
        }
    };
    html! {
        <button type={button_type.clone()} class={format!("font-bold py-2 px-4 rounded {}", style_ext)} onclick={onclick.clone().unwrap_or(Callback::noop())} disabled={disabled.clone()}>
            <span class={format!("flex flex-row items-center justify-center {}", button_content_styles)}>
                {
                    match icon {
                        Some(button_icon) => html!{ <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={button_icon.to_owned()}/> },
                        None => { html!{} }
                    }
                }
                <span>{ button_text.clone() }</span>
            </span>
        </button>
    }
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub style_ext: String,
    pub children: ChildrenWithProps<BasicButton>,
}

#[function_component]
pub fn ButtonGroup(props: &ButtonGroupProps) -> Html {
    let ButtonGroupProps {
        children,
        style_ext,
    } = props;
    html! {
        <div class="inline-flex rounded-md shadow-sm" role="group">
            {
                for children.iter().enumerate().map(|(index, child)| {
                    // let mut class_name = String::from("bg-blue-500 text-white border border-gray-200 hover:bg-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-500 focus:bg-blue-700 border-l-0");
                    let mut class_name = format!("bg-blue-500 text-white border border-gray-200 hover:bg-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-500 focus:bg-blue-700 border-l-0 {}", style_ext);

                    if index == 0 {
                        class_name.push_str(" rounded-l-md");
                    }

                    if index == children.len() - 1 {
                        class_name.push_str(" rounded-r-md");
                    }

                    html! {
                        <div class={class_name}>
                            { child.clone() }
                        </div>
                    }
                })
            }
        </div>
    }
}

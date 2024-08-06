use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    pub icon: IconId,
}

#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    let is_open = use_state(|| false);

    let toggle_content = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div>
            <span onclick={toggle_content} class="flex flex-row items-center justify-between gap-2 mb-2 p-2 rounded cursor-pointer hover:bg-blue-800"><span class="flex flex-row items-center gap-2"><Icon width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={props.icon}/><span>{ &props.title }</span></span><Icon width={"0.7em".to_owned()} height={"0.7em".to_owned()} icon_id={if *is_open { IconId::BootstrapDashLg } else { IconId::BootstrapPlusLg }}/></span>
            if *is_open {
                <div class="transition-max-height duration-700 ease-in-out overflow-hidden max-h-svh p-2 ml-2">
                    { props.children.clone() }
                </div>
            } else {
                <div class="overflow-hidden h-0 transition-max-height duration-700 ease-in-out"></div>
            }
        </div>
    }
}

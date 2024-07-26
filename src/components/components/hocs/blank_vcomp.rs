use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct BlankVCompProps {
    #[prop_or_default]
    pub children: Children
}

#[function_component]
pub fn BlankVComp(props: &BlankVCompProps) -> Html {
    html!{
        {
            props.children.iter().collect::<Html>()
        }
    }
}
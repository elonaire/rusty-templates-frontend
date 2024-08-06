use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct SelectInputProps {
    #[prop_or("".to_string())]
    pub initial_value: String,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    #[prop_or(false)]
    pub readonly: bool,
    pub options: Vec<SelectOption>,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(Callback::noop())]
    pub onchange: Callback<Event>,
    #[prop_or("".to_string())]
    pub ext_input_styles: String,
}

#[function_component]
pub fn SelectInput(props: &SelectInputProps) -> Html {

    let SelectInputProps {
        initial_value,
        label,
        name,
        input_node_ref,
        readonly,
        required,
        onchange,
        options,
        ext_input_styles
    } = props;

    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label for={props.name.clone()} class="block text-gray-700 text-sm font-bold mb-2">
                {props.label.clone()}
                { if *required { html!{ <span class="text-red-500">{ "*" }</span> }  } else { html!{} } }
            </label>
            <select
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                name={name.clone()}
                class={format!("form-input ring-0 shadow appearance-none border border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent flex-grow {}", ext_input_styles)}
                value={initial_value.clone()}
                readonly={*readonly}
                onchange={onchange.clone()}
                id={name.clone()}
            >
                { for options.iter().map(|option| {
                    html! {
                        <option value={option.value.clone()}>{option.clone().label.clone()}</option>
                    }
                })}
            </select>
            <p class="text-red-500 text-xs italic">{ if *display_error { "This field is required" } else { "" }  }</p>
        </div>
    }
}

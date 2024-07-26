use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct SelectInputProps {
    pub initial_value: Option<String>,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    pub readonly: Option<bool>,
    pub options: Vec<SelectOption>,
    pub required: Option<bool>,
    pub onchange: Option<Callback<Event>>,
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
        options
    } = props;

    let field_required = props.required.unwrap_or(false);
    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label for={props.name.clone()} class="block text-gray-700 text-sm font-bold mb-2">
                {props.label.clone()}
                { if field_required { html!{ <span class="text-red-500">{ "*" }</span> }  } else { html!{} } }
            </label>
            <select
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                name={name.clone()}
                class="form-input focus:ring-0 shadow border-2 border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500"
                value={initial_value.clone().unwrap_or("".to_string())}
                readonly={readonly.unwrap_or(false)}
                onchange={onchange.clone().unwrap_or(Callback::noop())}
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

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxInputFieldProps {
    pub initial_value: Option<String>,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub placeholder: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
    pub id_attr: String,
}

#[function_component]
pub fn CheckboxInputField(props: &CheckboxInputFieldProps) -> Html {
    let CheckboxInputFieldProps {
        initial_value,
        label,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
        id_attr
    } = props;

    let field_required = required.unwrap_or(false);
    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label class="inline-flex items-center gap-2 text-gray-700 text-sm font-bold" for={id_attr.clone()}>
                <input
                    class="leading-tight rounded border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50"
                    type="checkbox"
                    value={initial_value.clone().unwrap_or("".to_string())}
                    name={name.clone()}
                    ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                    readonly={readonly.unwrap_or(false)}
                    oninput={oninput.clone().unwrap_or(Callback::noop())}
                    placeholder={placeholder.clone().unwrap_or("".to_string())}
                    autocomplete="on"
                    id={id_attr.clone()}
                />
                <span>{ label }{
                    if field_required {
                        html!{ <span class="text-red-500">{ "*" }</span> }
                    } else {
                        html!{}
                    }
                }</span>
            </label>

            <p class="text-red-500 text-xs italic">{
                if *display_error {
                    "This field is required"
                } else {
                    ""
                }
            }</p>
        </div>
    }
}

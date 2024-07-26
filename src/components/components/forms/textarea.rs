use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    pub initial_value: Option<String>,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub placeholder: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
}

#[function_component]
pub fn Textarea(props: &TextAreaProps) -> Html {
    let TextAreaProps {
        initial_value,
        label,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
    } = props;

    let field_required = required.unwrap_or(false);
    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for={name.clone()}>
                { label }
                {
                    if field_required {
                        html!{ <span class="text-red-500">{ "*" }</span> }
                    } else {
                        html!{}
                    }
                }
            </label>
            <textarea
                class="form-input focus:ring-0 shadow appearance-none border-2 border-slate-400 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline focus:border-blue-500"
                value={initial_value.clone().unwrap_or("".to_string())}
                name={name.clone()}
                ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                readonly={readonly.unwrap_or(false)}
                oninput={oninput.clone().unwrap_or(Callback::noop())}
                placeholder={placeholder.clone().unwrap_or("".to_string())}
                id={name.clone()}
            />
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

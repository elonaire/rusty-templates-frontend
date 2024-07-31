use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioInputFieldProps {
    #[prop_or("".to_string())]
    pub initial_value: String,
    pub label: String,
    pub name: String,
    pub input_node_ref: Option<NodeRef>,
    #[prop_or(false)]
    pub readonly: bool,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(Callback::noop())]
    pub oninput: Callback<InputEvent>,
    pub id_attr: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn RadioInputField(props: &RadioInputFieldProps) -> Html {
    let RadioInputFieldProps {
        initial_value,
        label,
        name,
        input_node_ref,
        readonly,
        required,
        placeholder,
        oninput,
        id_attr,
        children
    } = props;

    let display_error = use_state(|| false);

    html! {
        <div class="mb-4">
            <label class="inline-flex items-center gap-2 text-gray-700 text-sm cursor-pointer" for={id_attr.clone()}>
                <input
                    class="leading-tight rounded-full border-gray-300 text-blue-950 shadow-sm focus:border-blue-950 focus:ring focus:ring-offset-0 focus:ring-indigo-200 focus:ring-opacity-50"
                    type="radio"
                    value={initial_value.clone()}
                    name={name.clone()}
                    ref={input_node_ref.clone().unwrap_or(NodeRef::default())}
                    readonly={*readonly}
                    oninput={oninput.clone()}
                    placeholder={placeholder.clone()}
                    autocomplete="on"
                    id={id_attr.clone()}
                />
                <div class="flex flex-col">
                    <span>
                        { label }
                    {
                        if *required {
                            html!{ <span class="text-red-500">{ "*" }</span> }
                        } else {
                            html!{}
                        }
                    }
                    </span>
                    { for props.children.iter() }
                </div>
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

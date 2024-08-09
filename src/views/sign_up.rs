use std::ops::Deref;

use crate::{
    app::Route,
    components::{
        button::BasicButton,
        forms::input::{InputField, InputFieldType},
        loading_spinner::LoadingSpinner,
        modal::basic_modal::{BasicModal, UseCase},
    },
    data::{
        context::users::sign_up,
        graphql::api_call::GraphQLResponse,
        models::{
            general::ModalConfigs,
            user::{SignUpForm, SignUpPayload},
        },
    },
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn SignUpPage() -> Html {
    let signup_form = use_state_eq(|| SignUpForm::default());
    let signup_form_is_valid = use_state_eq(|| false);
    let confirm_password = use_state_eq(|| String::new());
    let navigator = use_navigator().unwrap();
    // let current_state = use_context::<AppStateContext>().unwrap();
    let error = use_state_eq(|| String::new());
    let modal_configs = use_state_eq(|| ModalConfigs::default());
    let loading = use_state_eq(|| false);

    let onsubmit = {
        // let current_state_clone = current_state.clone();
        let signup_form_clone = signup_form.clone();
        // let navigator_clone = navigator.clone();
        let error_clone = error.clone();
        let modal_configs_clone = modal_configs.clone();
        let loading_clone = loading.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let logins = SignUpForm {
                email: signup_form_clone.email.clone(),
                password: signup_form_clone.password.clone(),
            };

            let payload = SignUpPayload { user: logins };

            // let current_state_clone = current_state_clone.clone();
            // let navigator_clone = navigator_clone.clone();
            let error_clone = error_clone.clone();
            let modal_configs_clone = modal_configs_clone.clone();
            let loading_clone = loading_clone.clone();
            let signup_form_clone = signup_form_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_clone.set(true);
                let sign_up_response = sign_up(payload).await;

                match &sign_up_response {
                    GraphQLResponse::Data(_data) => {
                        let modal_info = ModalConfigs {
                            use_case: UseCase::Success,
                            title: "Successfully Registered!".to_string(),
                            is_open: true,
                            message:
                                "You have been successfully registered, you can proceed to login"
                                    .to_string(),
                        };

                        modal_configs_clone.set(modal_info);

                        loading_clone.set(false);
                        signup_form_clone.set(SignUpForm::default());
                        error_clone.set(String::new());
                        // navigator_clone.push(&Route::Cart);
                        // navigator_clone.push(&Route::SignIn);
                    }
                    GraphQLResponse::Error(_e) => {
                        let modal_info = ModalConfigs {
                            use_case: UseCase::Error,
                            title: "Registration Failed!".to_string(),
                            is_open: true,
                            message:
                                "Registration Failed. Please check your details and try again."
                                    .to_string(),
                        };

                        modal_configs_clone.set(modal_info);
                        error_clone.set(sign_up_response.get_error().clone());
                        loading_clone.set(false);
                    }
                }
            });
        })
    };

    // reusing a handler is much cleaner
    let oninput = {
        let signup_form_clone = signup_form.clone();
        let confirm_password_clone = confirm_password.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target_dyn_into::<HtmlInputElement>();

            // match your name props for your text fields
            match target {
                Some(target) => match target.name().as_str() {
                    "password" => signup_form_clone.set(SignUpForm {
                        password: target.value(),
                        ..signup_form_clone.deref().clone()
                    }),
                    "email" => signup_form_clone.set(SignUpForm {
                        email: target.value(),
                        ..signup_form_clone.deref().clone()
                    }),
                    _ => confirm_password_clone.set(target.value()),
                },
                None => {
                    // If its a textarea, the exception will be handled here e.g
                    // let target = event.target_dyn_into::<HtmlTextAreaElement>();
                    // match target {
                    //     Some(target) => match target.name().as_str() {
                    //         "text_area_name" => {},
                    //         _ => {}
                    //     },
                    //     None => {}
                    // }
                }
            }
        })
    };

    {
        let signup_form_clone_deps = signup_form.clone();
        let signup_form_is_valid_clone = signup_form_is_valid.clone();
        let confirm_password_clone = confirm_password.clone();
        use_effect_with_deps(
            move |_| {
                // set valid to true if none of the fields are empty
                signup_form_is_valid_clone.set(
                    !signup_form_clone_deps.deref().email.is_empty()
                        && !signup_form_clone_deps.deref().password.is_empty()
                        && (signup_form_clone_deps.deref().password.as_str()
                            == confirm_password_clone.deref()),
                );
                || ()
            },
            (signup_form.clone(), confirm_password.clone()),
        );
    }

    let on_click_primary_modal = {
        let modal_configs_clone = modal_configs.clone();
        let navigator_clone = navigator.clone();
        let error_clone = error.clone();
        Callback::from(move |_| {
            let update_modal = ModalConfigs {
                is_open: false,
                ..(*modal_configs_clone).clone()
            };

            modal_configs_clone.set(update_modal);

            if (*error_clone).clone().is_empty() {
                navigator_clone.push(&Route::SignIn);
            }
        })
    };

    html! {
        <div class="min-h-screen font-jost-sans">
            {
                if *loading {
                    html!{ <LoadingSpinner /> }
                } else { html!{} }
            }
            <BasicModal use_case={modal_configs.use_case.clone()} title={modal_configs.title.clone()} is_open={modal_configs.is_open} on_click_primary={on_click_primary_modal} primary_button_text={"OK"}>
                <p>{modal_configs.message.clone()}</p>
                </BasicModal>
            <div class="flex flex-col items-center justify-center p-8 bg-white">
            <Link<Route> to={Route::Landing}>
            <img class="w-32 my-4" src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/01f762dc-20a6-4842-30fb-2b2401c66200/public" alt="logo" />
            </Link<Route>>
                <h1 class="text-4xl font-bold my-4">{"Sign Up"}</h1>
                // <div class="w-full max-w-md flex flex-col items-center gap-2 md:flex-row md:justify-between my-4">
                //     <BasicButton
                //         button_text={"Sign up with Google".to_string()}
                //         style_ext={"bg-red-500 hover:bg-red-400 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                //         onclick={Callback::from(|_| {
                //             // Handle Google Sign-Up
                //             gloo::console::log!("Google Sign-Up clicked");
                //         })}
                //         icon={Some(IconId::BootstrapGoogle)} // Assuming you have icons for Google
                //         disabled={false}
                //         button_type={"button".to_string()}
                //         icon_before={true}
                //     />
                //     <BasicButton
                //         button_text={"Sign up with GitHub".to_string()}
                //         style_ext={"bg-gray-700 hover:bg-gray-600 transition-all duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                //         onclick={Callback::from(|_| {
                //             // Handle GitHub Sign-Up
                //             gloo::console::log!("GitHub Sign-Up clicked");
                //         })}
                //         icon={Some(IconId::BootstrapGithub)} // Assuming you have icons for GitHub
                //         disabled={false}
                //         button_type={"button".to_string()}
                //         icon_before={true}
                //     />
                // </div>
                // <div class="w-full max-w-md flex items-center my-6">
                //     <hr class="flex-grow border-t border-gray-300"/>
                //     <span class="mx-4 text-gray-500">{"OR"}</span>
                //     <hr class="flex-grow border-t border-gray-300"/>
                // </div>
                <form {onsubmit} class="w-full max-w-md">
                    <div class="mb-6">
                        <InputField
                            initial_value={signup_form.email.clone()}
                            label={"Email".to_string()}
                            field_type={InputFieldType::Email}
                            name={"email".to_string()}
                            required={true}
                            placeholder={"Enter your email"}
                            oninput={oninput.clone()}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="mb-6">
                        <InputField
                            initial_value={signup_form.password.clone()}
                            label={"Password".to_string()}
                            field_type={InputFieldType::Password}
                            name={"password".to_string()}
                            required={true}
                            placeholder={"Enter your password"}
                            oninput={oninput.clone()}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <div class="mb-6">
                        <InputField
                            initial_value={(*confirm_password).clone()}
                            label={"Confirm Password".to_string()}
                            field_type={InputFieldType::Password}
                            name={"confirm_password".to_string()}
                            required={true}
                            placeholder={"Confirm your password"}
                            oninput={oninput.clone()}

                            ext_wrapper_styles={"mb-4".to_string()}
                            ext_label_styles={"block text-gray-700 text-sm font-bold mb-2".to_string()}
                            ext_input_styles={"focus:ring-secondary"}
                            autocomplete={"on".to_string()}
                        />
                    </div>
                    <BasicButton
                        button_text={"Sign Up".to_string()}
                        style_ext={"bg-primary text-white px-4 py-2 text-sm hover:bg-secondary transition duration-300 ease-in-out hover:shadow-md hover:-translate-y-1 hover:z-10 text-white w-full".to_string()}
                        icon={None}
                        disabled={!*signup_form_is_valid}
                        button_type={"submit".to_string()}
                        icon_before={true} // if you have an icon before the button text, set it to true
                    />
                    <div class="flex items-center justify-center mt-6 text-sm text-blue-500 hover:text-blue-400">
                        <Link<Route> to={Route::SignIn}>
                            {"Already have an account? Login."}
                        </Link<Route>>
                    </div>
                </form>

            </div>
        </div>
    }
}

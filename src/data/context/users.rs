use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{perform_mutation_or_query_with_vars, GraphQLResponse},
        models::user::{AuthDetails, LoginPayload, LoginResponse, SignUpPayload, SignUpResponse},
    },
    utils::auth_interceptor::retrieve_new_token,
};

pub async fn sign_up(payload: SignUpPayload) -> GraphQLResponse<SignUpResponse> {
    let endpoint = option_env!("ACL_SERVICE_URL").expect("ACL_SERVICE_URL env var not set");

    let query = r#"
            mutation Mutation($user: UserInput!) {
                signUp(user: $user) {
                    id
                }
            }
        "#;

    let sign_up_res = perform_mutation_or_query_with_vars::<SignUpResponse, SignUpPayload>(
        None, endpoint, query, payload,
    )
    .await;

    sign_up_res

    // state_clone.dispatch(StateAction::UpdateUserAuthInfo(
    //     match sign_up_res.get_data() {
    //         Some(data) => AuthDetails {
    //             user: data.sign_up.clone(),
    //             ..state_clone.auth_details.clone()
    //         },
    //         None => {
    //             state_clone.dispatch(StateAction::UpdateGlobalError(sign_up_res.get_error()));
    //             Default::default()
    //         },
    //     },
    // ));

    // Ok(())
}

pub async fn sign_in(payload: LoginPayload) -> GraphQLResponse<LoginResponse> {
    let endpoint = option_env!("ACL_SERVICE_URL").expect("ACL_SERVICE_URL env var not set");

    let query = r#"
            mutation Mutation($rawUserDetails: UserLoginsInput!) {
                signIn(rawUserDetails: $rawUserDetails) {
                    token
                    url
                }
            }
        "#;

    let sign_in_res = perform_mutation_or_query_with_vars::<LoginResponse, LoginPayload>(
        None, endpoint, query, payload,
    )
    .await;

    sign_in_res

    // state_clone.dispatch(StateAction::UpdateUserAuthInfo(
    //     match sign_in_res.get_data() {
    //         Some(data) => AuthDetails {
    //             user: Default::default(),
    //             token: data.sign_in.token.clone().unwrap_or("".to_string()),
    //             url: data.sign_in.url.clone(),
    //         },
    //         None => {
    //             state_clone.dispatch(StateAction::UpdateGlobalError(sign_in_res.get_error()));
    //             Default::default()
    //         },
    //     },
    // ));

    // Ok(())
}

pub async fn get_new_token(state_clone: &UseReducerHandle<AppState>) -> Result<(), Error> {
    match retrieve_new_token(&state_clone.auth_details.token).await {
        Ok(new_token) => {
            let details = AuthDetails {
                token: new_token.strip_prefix("Bearer ").unwrap_or("").to_string(),
                ..state_clone.auth_details.clone()
            };
            state_clone.dispatch(StateAction::UpdateUserAuthInfo(details));
        }
        Err(_e) => {}
    };

    Ok(())
}

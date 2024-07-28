use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{perform_mutation_or_query_with_vars, perform_query_without_vars},
        models::user::{AuthDetails, LoginPayload, LoginResponse},
    },
};

pub async fn sign_in(
    state_clone: &UseReducerHandle<AppState>,
    payload: LoginPayload
) -> Result<(), Error> {
    let endpoint = option_env!("ACL_SERVICE_URL").expect("ACL_SERVICE_URL env var not set");

    let query = r#"
            mutation Mutation($rawUserDetails: UserLoginsInput!) {
                signIn(rawUserDetails: $rawUserDetails) {
                    token
                    url
                }
            }
        "#;

    let sign_in_res = perform_mutation_or_query_with_vars::<
        LoginResponse,
        LoginPayload,
    >(None, endpoint, query, payload)
    .await;

    log::info!("Enters here: {:?}", sign_in_res);

    state_clone.dispatch(StateAction::UpdateUserAuthInfo(
        match sign_in_res.get_data() {
            Some(data) => AuthDetails {
                user: Default::default(),
                token: data.sign_in.token.clone().unwrap()
            },
            None => Default::default(),
        },
    ));

    Ok(())
}

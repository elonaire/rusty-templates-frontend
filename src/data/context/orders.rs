use std::{collections::HashMap, fmt::Error};

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{perform_mutation_or_query_with_vars, perform_query_without_vars},
        models::order::{CheckoutPayload, CheckoutResponse, UpdateCartPayload, UpdateCartResponse},
    },
};

pub async fn add_to_cart(
    state_clone: &UseReducerHandle<AppState>,
    payload: UpdateCartPayload
) -> Result<(), Error> {
    let mut auth_headers = HashMap::new();
    auth_headers.insert("Authorization".to_string(), format!("Bearer {}", state_clone.auth_details.token.clone()));

    log::info!("auth_headers: {:?}", auth_headers);
    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            mutation CartMutation($externalProductId: String!, $cartOperation: CartOperation!) {
            createOrUpdateCart(externalProductId: $externalProductId, cartOperation: $cartOperation) {
                    id
                    totalAmount
                }
            }
        "#;

    let cart = perform_mutation_or_query_with_vars::<
        UpdateCartResponse,
        UpdateCartPayload,
    >(Some(auth_headers), endpoint, query, payload)
    .await;

    log::info!("Enters here: {:?}", cart);

    state_clone.dispatch(StateAction::UpdateCart(
        match cart.get_data() {
            Some(data) => data.create_or_update_cart.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn checkout(
    state_clone: &UseReducerHandle<AppState>,
    payload: CheckoutPayload
) -> Result<(), Error> {
    let mut auth_headers = HashMap::new();
    auth_headers.insert("Authorization".to_string(), format!("Bearer {}", state_clone.auth_details.token.clone()));

    log::info!("auth_headers: {:?}", auth_headers);
    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            mutation OrderMutation {
                createOrder
            }
        "#;

    let payment_url = perform_mutation_or_query_with_vars::<
        CheckoutResponse,
        CheckoutPayload,
    >(Some(auth_headers), endpoint, query, payload)
    .await;

    log::info!("Enters here: {:?}", payment_url);

    state_clone.dispatch(StateAction::UpdateCheckoutUrl(
        match payment_url.get_data() {
            Some(data) => data.create_order.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

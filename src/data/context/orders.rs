use std::{collections::HashMap, fmt::Error};

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{perform_mutation_or_query_with_vars, perform_query_without_vars},
        models::order::{CheckoutPayload, CheckoutResponse, GetCartResponse, GetLicensesResponse, GetOrderCartProductsResponse, GetOrderCartProductsVar, GetProductsIdsResponse, GetProductsIdsVar, GetRawCartProductsResponse, GetRawCartProductsVar, OrderStatus, UpdateCartPayload, UpdateCartResponse},
    },
};

pub async fn add_to_cart(
    state_clone: &UseReducerHandle<AppState>,
    payload: UpdateCartPayload
) -> Result<(), Error> {
    let mut auth_headers = HashMap::new();
    auth_headers.insert("Authorization".to_string(), format!("Bearer {}", state_clone.auth_details.token.clone()));

    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            mutation CartMutation($externalProductId: String!, $cartOperation: CartOperation!, $externalLicenseId: String) {
            createOrUpdateCart(externalProductId: $externalProductId, cartOperation: $cartOperation, externalLicenseId: $externalLicenseId) {
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

    log::info!("{:?}", cart);

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

    state_clone.dispatch(StateAction::UpdateCheckoutUrl(
        match payment_url.get_data() {
            Some(data) => data.create_order.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_product_external_ids(
    state_clone: &UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            query CartQuery($cartId: String!) {
                getProductExternalIds(cartId: $cartId)
            }
        "#;
    println!("state_clone.cart.id.clone(){:?}", state_clone.cart.id.clone());

    let vars = GetProductsIdsVar {
        cart_id: state_clone.cart.id.clone().unwrap_or("".into())
    };

    let products_ids = perform_mutation_or_query_with_vars::<
        GetProductsIdsResponse,
        GetProductsIdsVar
    >(None, endpoint, query, vars)
    .await;

    state_clone.dispatch(StateAction::UpdateCartProductsIds(
        match products_ids.get_data() {
            Some(data) => data.get_product_external_ids.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_cart(
    state_clone: &UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let mut auth_headers = HashMap::new();
    auth_headers.insert("Authorization".to_string(), format!("Bearer {}", state_clone.auth_details.token.clone()));

    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            query CartQuery {
                getCart {
                    id
                    totalAmount
                    archived
                }
            }
        "#;

    let cart = perform_query_without_vars::<
        GetCartResponse,
    >(Some(auth_headers), endpoint, query)
    .await;

    state_clone.dispatch(StateAction::UpdateCart(
        match cart.get_data() {
            Some(data) => data.get_cart.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_licenses(
    state_clone: &UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            query OrderQuery {
                getLicenses{
                    id
                    name
                    priceFactor
                    shortDescription
                }
            }
        "#;

    let licenses = perform_query_without_vars::<
        GetLicensesResponse,
    >(None, endpoint, query)
    .await;

    state_clone.dispatch(StateAction::UpdateLicenses(
        match licenses.get_data() {
            Some(data) => data.get_licenses.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_raw_cart_products(
    state_clone: &UseReducerHandle<AppState>,
    cart_id: String
) -> Result<(), Error> {
    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            query OrderQuery($cartId: String!) {
                getRawCartProducts(cartId: $cartId){
                    id
                    license
                    quantity
                    extProductId
                    artifact
                }
            }
        "#;

    let vars = GetRawCartProductsVar {
        cart_id,
    };

    let licenses = perform_mutation_or_query_with_vars::<
        GetRawCartProductsResponse,
        GetRawCartProductsVar
    >(None, endpoint, query, vars)
    .await;

    state_clone.dispatch(StateAction::UpdateRawCartProducts(
        match licenses.get_data() {
            Some(data) => data.get_raw_cart_products.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_order_cart_products_by_status(
    state_clone: &UseReducerHandle<AppState>,
    status: OrderStatus
) -> Result<(), Error> {
    let mut auth_headers = HashMap::new();
    auth_headers.insert("Authorization".to_string(), format!("Bearer {}", state_clone.auth_details.token.clone()));

    let endpoint = option_env!("ORDERS_SERVICE_URL").expect("ORDERS_SERVICE_URL env var not set");

    let query = r#"
            query OrderQuery($status: OrderStatus!) {
                getCustomerOrdersByStatus(status: $status){
                    id
                    license
                    quantity
                    extProductId
                    artifact
                }
            }
        "#;

    let vars = GetOrderCartProductsVar {
        status,
    };

    let order_products = perform_mutation_or_query_with_vars::<
        GetOrderCartProductsResponse,
        GetOrderCartProductsVar
    >(Some(auth_headers), endpoint, query, vars)
    .await;

    log::info!("{:?}", order_products);

    state_clone.dispatch(StateAction::UpdateOrderCartProducts(
        match order_products.get_data() {
            Some(data) => data.get_customer_orders_by_status.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

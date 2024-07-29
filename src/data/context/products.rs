use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{perform_mutation_or_query_with_vars, perform_query_without_vars},
        models::template::*,
    },
};

pub async fn get_products(
    state_clone: &UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let endpoint = option_env!("PRODUCTS_SERVICE_URL").expect("PRODUCTS_SERVICE_URL env var not set");

    let query = r#"
            query ProductQuery {
                getProducts {
                    id
                    name
                    price
                    screenshot
                    useCase
                }
            }
        "#;

    let products = perform_query_without_vars::<
        GetProductsResponse
    >(None, endpoint, query)
    .await;

    state_clone.dispatch(StateAction::UpdateProducts(
        match products.get_data() {
            Some(data) => data.get_products.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

pub async fn get_products_by_ids(
    state_clone: &UseReducerHandle<AppState>,
) -> Result<(), Error> {
    let endpoint = option_env!("PRODUCTS_SERVICE_URL").expect("PRODUCTS_SERVICE_URL env var not set");

    let query = r#"
            query ProductQuery($productIds: [String]!) {
                getProductsByIds(productIds: $productIds) {
                    id
                    name
                    price
                    screenshot
                    useCase
                }
            }
        "#;

    let vars = GetProductsByIdsVar {
        product_ids: state_clone.cart_products_ids.clone()
    };

    let products = perform_mutation_or_query_with_vars::<
        GetProductsByIdsResponse,
        GetProductsByIdsVar
    >(None, endpoint, query, vars)
    .await;

    state_clone.dispatch(StateAction::UpdateCartProducts(
        match products.get_data() {
            Some(data) => data.get_products_by_ids.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

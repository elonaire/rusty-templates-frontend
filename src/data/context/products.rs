use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::perform_query_without_vars,
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

    log::info!("Enters here: {:?}", products);

    state_clone.dispatch(StateAction::UpdateProducts(
        match products.get_data() {
            Some(data) => data.get_products.clone(),
            None => Default::default(),
        },
    ));

    Ok(())
}

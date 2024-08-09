use std::fmt::Error;

use yew::UseReducerHandle;

use crate::{
    app::{AppState, StateAction},
    data::{
        graphql::api_call::{
            perform_mutation_or_query_with_vars, perform_query_without_vars, GraphQLResponse,
        },
        models::{
            general::{ServeMdPayload, ServeMdResponse},
            template::*,
        },
    },
};

pub async fn get_products(state_clone: &UseReducerHandle<AppState>) -> Result<(), Error> {
    let endpoint =
        option_env!("PRODUCTS_SERVICE_URL").expect("PRODUCTS_SERVICE_URL env var not set");

    let query = r#"
            query ProductQuery {
                getProducts {
                    id
                    name
                    price
                    screenshot
                    useCase
                    slug
                    previewLink
                    productDetails
                }
            }
        "#;

    let products = perform_query_without_vars::<GetProductsResponse>(None, endpoint, query).await;

    state_clone.dispatch(StateAction::UpdateProducts(match products.get_data() {
        Some(data) => data.get_products.clone(),
        None => {
            // state_clone.dispatch(StateAction::UpdateGlobalError(products.get_error()));
            Default::default()
        }
    }));

    Ok(())
}

pub async fn get_products_by_ids(state_clone: &UseReducerHandle<AppState>) -> Result<(), Error> {
    let endpoint =
        option_env!("PRODUCTS_SERVICE_URL").expect("PRODUCTS_SERVICE_URL env var not set");

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
        product_ids: state_clone.cart_products_ids.clone(),
    };

    let products = perform_mutation_or_query_with_vars::<
        GetProductsByIdsResponse,
        GetProductsByIdsVar,
    >(None, endpoint, query, vars)
    .await;

    state_clone.dispatch(StateAction::UpdateCartProducts(match products.get_data() {
        Some(data) => data.get_products_by_ids.clone(),
        None => {
            // state_clone.dispatch(StateAction::UpdateGlobalError(products.get_error()));
            Default::default()
        }
    }));

    Ok(())
}

pub async fn get_product_by_slug(slug: &String) -> GraphQLResponse<GetProductBySlugResponse> {
    let endpoint =
        option_env!("PRODUCTS_SERVICE_URL").expect("PRODUCTS_SERVICE_URL env var not set");

    let query = r#"
            query ProductQuery($slug: String) {
                getProductBySlug(slug: $slug) {
                    id
                    name
                    slug
                    price
                    previewLink
                    screenshot
                    framework
                    applicationLayer
                    uiFramework
                    productDetails
                }
            }
        "#;

    let vars = GetProductBySlugVar { slug: slug.clone() };

    let product = perform_mutation_or_query_with_vars::<
        GetProductBySlugResponse,
        GetProductBySlugVar,
    >(None, endpoint, query, vars)
    .await;

    product

    // state_clone.dispatch(StateAction::UpdateCurrentProductDetails(
    //     match product.get_data() {
    //         Some(data) => data.get_product_by_slug.clone(),
    //         None => {
    //             // state_clone.dispatch(StateAction::UpdateGlobalError(product.get_error()));
    //             Default::default()
    //         }
    //     },
    // ));

    // Ok(())
}

pub async fn serve_md_file(file_name: String) -> GraphQLResponse<ServeMdResponse> {
    let endpoint = option_env!("FILES_SERVICE").expect("FILES_SERVICE env var not set");

    let query = r#"
            query FilesQuery($fileName: String!) {
                serveMdFiles(fileName: $fileName)
            }
        "#;

    let payload = ServeMdPayload { file_name };

    let serve_md_res = perform_mutation_or_query_with_vars::<ServeMdResponse, ServeMdPayload>(
        None, endpoint, query, payload,
    )
    .await;

    serve_md_res
}

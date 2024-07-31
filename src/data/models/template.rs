use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Properties, Default)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "previewLink")]
    pub preview_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<Framework>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "applicationLayer")]
    pub application_layer: Option<ApplicationLayer>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "uiFramework")]
    pub ui_framework: Option<UiFramework>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "useCase")]
    pub use_case: Option<UseCase>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum Framework {
    Yew,
    Dioxus,
    Axum,
    Rocket,
    Iced,
    Tauri,
    Actix,
    Warp,
    Rouille,
    Thruster,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum ApplicationLayer {
    Frontend,
    Backend,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UiFramework {
    #[serde(rename = "RustyUI")]
    RustyUI,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UseCase {
    Dashboard,
    Ecommerce,
    Admin,
    #[serde(rename = "EcommerceAdmin")]
    EcommerceAdmin,
    #[serde(rename = "FinanceAdmin")]
    FinanceAdmin,
    #[serde(rename = "IoTAdmin")]
    IoTAdmin,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProductsResponse {
    #[serde(rename = "getProducts")]
    pub get_products: Vec<Product>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProductsByIdsResponse {
    #[serde(rename = "getProductsByIds")]
    pub get_products_by_ids: Vec<Product>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProductsByIdsVar {
    #[serde(rename = "productIds")]
    pub product_ids: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProductBySlugResponse {
    #[serde(rename = "getProductBySlug")]
    pub get_product_by_slug: Product,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetProductBySlugVar {
    pub slug: String
}

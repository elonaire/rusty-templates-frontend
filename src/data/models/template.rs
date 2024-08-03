use std::fmt;

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

// Implement the Display trait
impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Framework::Yew => write!(f, "Yew"),
            Framework::Dioxus => write!(f, "Dioxus"),
            Framework::Axum => write!(f, "Axum"),
            Framework::Rocket => write!(f, "Rocket"),
            Framework::Iced => write!(f, "Iced"),
            Framework::Tauri => write!(f, "Tauri"),
            Framework::Actix => write!(f, "Actix"),
            Framework::Warp => write!(f, "Warp"),
            Framework::Rouille => write!(f, "Rouille"),
            Framework::Thruster => write!(f, "Thruster"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum ApplicationLayer {
    Frontend,
    Backend,
}

// Implement the Display trait
impl fmt::Display for ApplicationLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationLayer::Frontend => write!(f, "Frontend"),
            ApplicationLayer::Backend => write!(f, "Backend"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum UiFramework {
    #[serde(rename = "RustyUI")]
    RustyUI,
}

// Implement the Display trait
impl fmt::Display for UiFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiFramework::RustyUI => write!(f, "RustyUI"),
        }
    }
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

// Implementing the Display trait
impl fmt::Display for UseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UseCase::Dashboard => write!(f, "Dashboard"),
            UseCase::Ecommerce => write!(f, "E-commerce"),
            UseCase::Admin => write!(f, "Admin"),
            UseCase::EcommerceAdmin => write!(f, "E-commerce Admin"),
            UseCase::FinanceAdmin => write!(f, "Finance Admin"),
            UseCase::IoTAdmin => write!(f, "IoT Admin"),
        }
    }
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

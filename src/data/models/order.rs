use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum CartOperation {
    RemoveProduct,
    AddProduct,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateCartPayload {
    #[serde(rename = "externalProductId")]
    pub external_product_id: String,
    #[serde(rename = "cartOperation")]
    pub cart_operation: CartOperation,
    #[serde(rename = "externalLicenseId")]
    pub external_license_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateCartResponse {
    #[serde(rename = "createOrUpdateCart")]
    pub create_or_update_cart: Cart,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Cart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "totalAmount")]
    pub total_amount: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CartTotals {
    pub subtotal: u64,
    pub vat_rate: f64,
    pub vat: f64,
    pub total: u64,
}

impl CartTotals {
    pub fn calculate(&mut self) -> Self {
        self.vat = self.subtotal as f64 * self.vat_rate;
        self.total = (self.subtotal as f64 + self.vat) as u64;

        self.to_owned()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CheckoutPayload;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct CheckoutResponse {
    #[serde(rename = "createOrder")]
    pub create_order: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductsIdsResponse {
    #[serde(rename = "getProductExternalIds")]
    pub get_product_external_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductsIdsVar {
    #[serde(rename = "cartId")]
    pub cart_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetCartVar {
    #[serde(rename = "cartId")]
    pub cart_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetCartResponse {
    #[serde(rename = "getCart")]
    pub get_cart: Cart,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductSalesPayload {
    #[serde(rename = "externalProductId")]
    pub external_product_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductSalesResponse {
    #[serde(rename = "getProductTotalSales")]
    pub get_product_total_sales: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetLicensesResponse {
    #[serde(rename = "getLicenses")]
    pub get_licenses: Vec<License>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct License {
    pub id: String,
    pub name: String,
    #[serde(rename = "priceFactor")]
    pub price_factor: u64,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default, Eq)]
pub struct CartProduct {
    pub id: String,
    pub license: String,
    pub quantity: u32,
    #[serde(rename = "extProductId")]
    pub ext_product_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetRawCartProductsVar {
    #[serde(rename = "cartId")]
    pub cart_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GetOrderCartProductsVar {
    pub status: OrderStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetRawCartProductsResponse {
    #[serde(rename = "getRawCartProducts")]
    pub get_raw_cart_products: Vec<CartProduct>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetOrderCartProductsResponse {
    #[serde(rename = "getCustomerOrdersByStatus")]
    pub get_customer_orders_by_status: Vec<CartProduct>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Ready,
    Completed,
    Failed,
    Refunded,
    OnHold,
}

// Implement the Display trait
impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Confirmed => write!(f, "Confirmed"),
            _ => write!(f, "On Hold"),
        }
    }
}

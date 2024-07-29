use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Eq, PartialEq)]
pub enum CartOperation {
    RemoveProduct,
    AddProduct
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateCartPayload {
    #[serde(rename = "externalProductId")]
    pub external_product_id: String,
    #[serde(rename = "cartOperation")]
    pub cart_operation: CartOperation,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct UpdateCartResponse {
    #[serde(rename = "createOrUpdateCart")]
    pub create_or_update_cart: Cart
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
    pub total: u64
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
    pub create_order: String
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductsIdsResponse {
    #[serde(rename = "getProductExternalIds")]
    pub get_product_external_ids: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetProductsIdsVar {
    #[serde(rename = "cartId")]
    pub cart_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetCartVar {
    #[serde(rename = "cartId")]
    pub cart_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct GetCartResponse {
    #[serde(rename = "getCart")]
    pub get_cart: Cart
}

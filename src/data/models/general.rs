use crate::components::modal::basic_modal::UseCase;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Clone)]
pub struct ModalConfigs {
    pub is_open: bool,
    pub message: String,
    pub use_case: UseCase,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServeMdResponse {
    #[serde(rename = "serveMdFiles")]
    pub serve_md_files: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ServeMdPayload {
    #[serde(rename = "fileName")]
    pub file_name: String,
}

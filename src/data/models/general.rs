use crate::components::modal::basic_modal::UseCase;

#[derive(Default, PartialEq, Clone)]
pub struct ModalConfigs {
    pub is_open: bool,
    pub message: String,
    pub use_case: UseCase,
    pub title: String,
}

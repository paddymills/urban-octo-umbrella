
//! CO02: edit order

use crate::api::Predicate;

use super::TransAct;

/// CO02 predicates
#[derive(Debug)]
pub enum Predicates {
    /// Main screen header text
    MainHeader,
    /// operations listing header text
    OperationsHeader,
    /// components listing header text
    ComponentsHeader,
    /// delete component error header text
    DeleteComponentErrorHeader,
    /// header text of error message on save
    SaveErrorHeader
}

impl Predicates {
    /// loads an image predicate for variant
    pub fn img(&self) -> Result<Predicate, String> {
        match self {
            Self::MainHeader                 => Predicate::img(TransAct::CO02, "MainHeader"),
            Self::OperationsHeader           => Predicate::img(TransAct::CO02, "OperationsHeader"),
            Self::ComponentsHeader           => Predicate::img(TransAct::CO02, "ComponentsHeader"),
            Self::DeleteComponentErrorHeader => Predicate::img(TransAct::CO02, "DeleteComponentErrorHeader"),
            Self::SaveErrorHeader            => Predicate::img(TransAct::CO02, "SaveErrorHeader"),
        }
    }
}

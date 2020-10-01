use thiserror::Error;




#[derive(Error, Debug)]
pub enum GenError {

    #[error("The corresponding character is missing!")]
    MissChar,

    #[error("Delete non-exist value!")]
    DelNonExistValue,

    #[error("Require unit to be non-negative")]
    InvalidUnit,

    #[error("Require ASCII characters excluded control ones")]
    InvalidChar,

    #[error("Require a existed kind: `L`, `S` or `N`")]
    InvalidKind,

    #[error("Require Non-negative integer in `&str`")]
    InvalidNumber,

    #[error("Require a defined operation: `update` or `check`")]
    InvalidOperation(String),

}

use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub(crate) error: bool,
    pub(crate) message: String,
}

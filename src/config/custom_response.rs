use serde::Serialize;

#[derive(Serialize)]
pub struct CustomResponse<T> {
    pub status: &'static str,
    pub data: T
}

impl <T> CustomResponse<T> {
    pub fn new(data: T) -> Self{
        return CustomResponse {
            status: "ok",
            data
        }
    }
}

#[derive(Serialize)]
pub struct CustomResponseEmptyData{}
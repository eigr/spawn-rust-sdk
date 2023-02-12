pub mod v1 {
    use crate::actor::Actor;

    use actix_web::{post, web::Data, HttpRequest, Responder};

    #[post("/actions")]
    pub async fn handle(_data: Data<Vec<Box<dyn Actor>>>, _req: HttpRequest) -> impl Responder {
        "Hello World from v1 API!"
    }
}

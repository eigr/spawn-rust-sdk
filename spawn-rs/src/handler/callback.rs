pub mod v1 {
    use std::cell::Cell;

    use crate::actor::Actor;
    use crate::eigr::spawn::ActorInvocation;

    use actix_protobuf::ProtoBuf;
    use actix_web::{http::header::ContentType, post, web::Data, HttpResponse, Responder};

    #[post("/actions")]
    pub async fn handle(
        ctx: Data<Cell<Vec<Box<dyn Actor>>>>,
        req: ProtoBuf<ActorInvocation>,
    ) -> impl Responder {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(format!("Hello !"))
    }
}

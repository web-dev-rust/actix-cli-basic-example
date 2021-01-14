
use actix_web::{web, HttpResponse};
use crate::maluco_cli_web::controllers::{ pong, readiness };
use crate::maluco_cli_web::controllers::crud::{create_object, get_object, update_object, delete_object, show_object};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            
.service(
    web::scope("api/")
    
            .route("object/new", web::post().to(create_object))
            .route("object/delete/{id}", web::delete().to(delete_object))
            .route("object/get", web::get().to(show_object))
            .route("object/get/{id}", web::get().to(get_object))
            .route("object/update/{id}", web::put().to(update_object))
)
            .route("ping", web::get().to(pong))
            .route("~/ready", web::get().to(readiness))
            .route("", web::get().to(|| HttpResponse::NotFound())),
    );
}


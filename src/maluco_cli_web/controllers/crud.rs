
    use crate::maluco_cli::model::maluco_cli::*;
    use crate::maluco_cli::database::Context;

    use actix_web::{web, HttpResponse, Responder};
    
    pub async fn create_object(state: web::Data<Context>, info: web::Json<Object>) -> impl Responder {
        let id = uuid::Uuid::new_v4();
    
        unimplemented!()
    }
    
    pub async fn show_object(state: web::Data<Context>) -> impl Responder {
        unimplemented!()
    }

    pub async fn delete_object(id: web::Path<String>, state: web::Data<Context>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }
    
        unimplemented!()
    }
    
    pub async fn get_object(id: web::Path<String>, state: web::Data<Context>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }
    
        unimplemented!()
    }
    
    pub async fn update_object(
        id: web::Path<String>,
        info: web::Json<ObjectUpdate>, 
        state: web::Data<Context>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }
    
        unimplemented!()
    }


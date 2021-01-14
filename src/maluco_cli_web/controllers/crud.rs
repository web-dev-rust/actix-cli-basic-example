
    use crate::maluco_cli::model::maluco_cli::*;
    use crate::maluco_cli::database::Context;
    use std::sync::{Arc, Mutex};

    use actix_web::{web, HttpResponse, Responder};
    
    pub async fn create_object(state: web::Data<Arc<Mutex<Context>>>, info: web::Bytes) -> impl Responder {
        let id = uuid::Uuid::new_v4();
    
        let bytes_str = match String::from_utf8(info.to_vec()) {
            Ok(text) => Ok(text),
            Err(_) => Err("bytes parse error")
        };
    
        let info = serde_json::from_str(&bytes_str.unwrap()).unwrap();
        let mut state = state.lock().unwrap();
        match state.create(id.to_string(), info) {
            true => HttpResponse::Created().body(id.to_string()),
            false => HttpResponse::Conflict().finish(),
        }
    }
    
    pub async fn show_object(state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
        let mut state = state.lock().unwrap();
        let response = state.all();
        HttpResponse::Ok().json(response)
    }

    pub async fn delete_object(id: web::Path<String>, state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }
    
        let mut state = state.lock().unwrap();
        match state.delete(id.to_string()) {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::BadRequest().finish(),
        }
    }
    
    pub async fn get_object(id: web::Path<String>, state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }
    
        let mut state = state.lock().unwrap();
        match state.get(uuid) {
            Some(body) => HttpResponse::Ok().json(body),
            None => HttpResponse::BadRequest().finish(),
        }
    }
    
    pub async fn update_object(
        id: web::Path<String>,
        info: web::Bytes, 
        state: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
        let uuid = id.to_string();
    
        if uuid::Uuid::parse_str(&uuid).is_err() {
            return HttpResponse::BadRequest().body("Id must be a Uuid::V4");
        }

        let bytes_str = match String::from_utf8(info.to_vec()) {
            Ok(text) => Ok(text),
            Err(_) => Err("bytes parse error")
        };
    
        let info: ObjectUpdate = serde_json::from_str(&bytes_str.unwrap()).unwrap();
        let mut state = state.lock().unwrap();
        match state.update(id.to_string(), info) {
            true => HttpResponse::Ok().finish(),
            false => HttpResponse::BadRequest().finish(),
        }
    }


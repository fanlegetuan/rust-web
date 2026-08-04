use super::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn heath_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let heath_check_response = &app_state.heath_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", heath_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

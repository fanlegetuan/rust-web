use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use actix_web::{HttpResponse, web};
use chrono::Utc;

pub async fn heath_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let heath_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", heath_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_all_courses(app_state: web::Data<AppState>) -> HttpResponse {
    match get_all_courses_db(&app_state.db).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(err) => {
            eprintln!("get_all_courses error: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let now = Utc::now().naive_utc();
    match new_course_db(&app_state.db, new_course.teacher_id, &new_course.name, now).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => {
            eprintln!("new_course error: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let teacher_id = params.into_inner();
    match get_courses_for_teacher_db(&app_state.db, teacher_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(err) => {
            eprintln!("get_courses_for_teacher error: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.into_inner();
    match get_course_details_db(&app_state.db, teacher_id, course_id).await {
        Ok(Some(course)) => HttpResponse::Ok().json(course),
        Ok(None) => HttpResponse::NotFound().json("Course not found"),
        Err(err) => {
            eprintln!("get_course_detail error: {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Mutex;

    async fn make_state() -> web::Data<AppState> {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = PgPoolOptions::new()
            .connect(&url)
            .await
            .expect("failed to connect to database");
        web::Data::new(AppState {
            health_check_response: String::new(),
            visit_count: Mutex::new(0),
            db,
        })
    }

    #[actix_rt::test]
    async fn post_course_test() {
        let app_state = make_state().await;
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            time: None,
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_courses_for_teacher_test() {
        let app_state = make_state().await;
        let teacher_id = web::Path::<i32>::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        let app_state = make_state().await;
        let params = web::Path::<(i32, i32)>::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

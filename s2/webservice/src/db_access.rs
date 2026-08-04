use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_all_courses_db(pool: &PgPool) -> Result<Vec<Course>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course"#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name,
            time: r.time,
        })
        .collect())
}

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name,
            time: r.time,
        })
        .collect())
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Option<Course>, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Course {
        id: Some(r.id),
        teacher_id: r.teacher_id,
        name: r.name,
        time: r.time,
    }))
}

pub async fn new_course_db(
    pool: &PgPool,
    teacher_id: i32,
    name: &str,
    time: NaiveDateTime,
) -> Result<Course, sqlx::Error> {
    let row = sqlx::query!(
        r#"INSERT INTO course (teacher_id, name, time)
        VALUES ($1, $2, $3)
        RETURNING id, teacher_id, name, time"#,
        teacher_id,
        name,
        time
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name,
        time: row.time,
    })
}

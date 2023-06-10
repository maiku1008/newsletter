use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing;
use uuid::Uuid;

// The form data, a serializable object
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// This is a macro and acts like a decorator in Python
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form, pool), 
    fields(
        subscriber_email = %form.email, 
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // The posted form is automatically deserialized in the FormData struct for immediate use
    // provided the header is "application/x-www-form-urlencoded"
    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(name = "Saving new subscriber in the database", skip(form, pool))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

// TODO:
// Add other CRUD 
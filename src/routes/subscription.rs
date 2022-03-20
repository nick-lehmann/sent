use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubscriptionData {
    pub name: String,
    pub email: String,
}

#[post("/subscriptions")]
#[tracing::instrument(
    name = "Adding a new subscriber", skip(form, pool),
    fields(subscriber_email = %form.email, subscriber_name= %form.name)
)]
pub async fn subscribe(
    form: web::Form<SubscriptionData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new subscriber into database", skip(pool, form))]
async fn insert_subscriber(pool: &PgPool, form: &SubscriptionData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
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
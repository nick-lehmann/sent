use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(Debug, Deserialize)]
pub struct SubscriptionData {
    pub name: String,
    pub email: String,
}

impl TryFrom<SubscriptionData> for NewSubscriber {
    type Error = String;

    fn try_from(form: SubscriptionData) -> Result<Self, Self::Error> {
        Ok(NewSubscriber {
            name: SubscriberName::parse(form.name)?,
            email: SubscriberEmail::parse(form.email)?,
        })
    }
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
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Saving new subscriber into database", skip(pool, subscriber))]
async fn insert_subscriber(pool: &PgPool, subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
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

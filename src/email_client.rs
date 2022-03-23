use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct MockEmailClient {
    pub sender: SubscriberEmail,
}

impl MockEmailClient {
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        tracing::info!(
            "New subscriber email sent to {} with token {}",
            recipient.as_ref(),
            ""
        );
        Ok(())
    }
}

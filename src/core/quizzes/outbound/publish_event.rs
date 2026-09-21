use crate::core::quizzes::models::outbound::Observation::Observation;
use reqwest::{Client, StatusCode};

pub struct EventPublisher {}

impl EventPublisher {
    pub async fn publish_event(observation: &Observation) -> reqwest::Result<()> {
        let client = Client::new();

        match client
            .post("http://intelligence-service:8080/api/v1/observations")
            .json(observation)
            .send()
            .await
        {
            Ok(response) => {
                log::info!(
                    "event.publish | event_publisher | publish_event | response_received | \"Received response from event publisher.\" |"
                );
                let status = response.status();

                match status {
                    StatusCode::OK => {
                        log::info!(
                            "event.publish | event_publisher | publish_event | success | \"Successfully published event.\" |"
                        );
                    }
                    StatusCode::BAD_REQUEST => {
                        log::error!(
                            "event.publish | event_publisher | publish_event | failed | {:?} | \"Failed to publish event.\" | message = {:?}",
                            StatusCode::BAD_REQUEST,
                            response.text().await?
                        );
                    }
                    StatusCode::INTERNAL_SERVER_ERROR => {
                        log::error!(
                            "event.publish | event_publisher | publish_event | failed | {:?} | \"Failed to publish event.\" | message = {:?}",
                            StatusCode::INTERNAL_SERVER_ERROR,
                            response.text().await?
                        );
                    }
                    _ => {
                        log::error!(
                            "event.publish | event_publisher | publish_event | failed | \"Failed to publish event.\" | response = {:?}",
                            response
                        );
                    }
                }
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to publish event: {:?}", e);
                Err(e)
            }
        }
    }
}

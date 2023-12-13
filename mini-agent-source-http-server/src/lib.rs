use axum::http::StatusCode;
use axum::{Extension, Json};
use mini_agent_core::event::{Event, EventMetric};
use mini_agent_core::prelude::Component;
use mini_agent_source_prelude::prelude::{Source, SourceConfig};
use tokio::sync::mpsc;

#[derive(Debug, serde::Deserialize)]
pub struct HttpServerConfig {
    pub address: String,
}

impl SourceConfig for HttpServerConfig {
    type Output = HttpServer;

    fn build(self, output: mpsc::Sender<Event>) -> Self::Output {
        HttpServer {
            address: self.address,
            output,
        }
    }
}

async fn handle_metric(
    Extension(sender): Extension<mpsc::Sender<Event>>,
    Json(payload): Json<EventMetric>,
) -> StatusCode {
    match sender.send(Event::Metric(payload)).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_err) => StatusCode::TOO_MANY_REQUESTS,
    }
}

pub struct HttpServer {
    address: String,
    output: mpsc::Sender<Event>,
}

impl Component for HttpServer {
    async fn run(self) {
        let app = axum::Router::new()
            .layer(Extension(self.output))
            .route("/metrics", axum::routing::post(handle_metric));
        let listener = tokio::net::TcpListener::bind(self.address.as_str())
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

impl Source for HttpServer {}

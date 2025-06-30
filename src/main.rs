mod client;
mod handlers;
mod routes;
mod types;

use routes::create_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = create_routes();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Server starting on {}", addr);
    println!("Available endpoints:");
    println!("  GET  /                    - Docs");
    println!("  GET  /health              - Health check");
    println!("  GET  /balance/:address    - Get account balance");
    println!("  GET  /account/:address    - Get account information");
    println!("  GET  /transaction/:sig    - Get transaction details");
    println!("  GET  /validators          - Get network validators info");
    println!("  GET  /slot                - Get latest slot");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

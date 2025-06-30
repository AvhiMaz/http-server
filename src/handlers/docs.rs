use axum::http::StatusCode;
use axum::response::Html;

pub async fn docs() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::OK,
        Html(
            r#"
            <html>
            <head>
                <title>Solana HTTP API</title>
                <style>
                    body {
                        background-color:#18181a;
                        color: #fff;
                        font-family: monospace;
                        padding: 2rem;
                    }
                    h2 {
                        color: #00ff88;
                    }
                    ul {
                        line-height: 1.8;
                        font-size: 1rem;
                    }
                    code {
                        background: #111;
                        padding: 2px 6px;
                        border-radius: 4px;
                        color: #0ff;
                    }
                </style>
            </head>
            <body>
                <h2>Solana HTTP API</h2>
                <p>available endpoints:</p>
                <ul>
                    <li><code>GET /</code> – docs</li>
                    <li><code>GET /health</code> – health check</li>
                    <li><code>GET /balance/:address</code> – Get SOL balance of wallet</li>
                    <li><code>GET /account/:address</code> – Get account info</li>
                    <li><code>GET /transaction/:signature</code> – Get transaction details</li>
                    <li><code>GET /validators</code> – Get current validators</li>
                    <li><code>GET /slot</code> – Get latest slot</li>
                </ul>
            </body>
            </html>
        "#,
        ),
    )
}

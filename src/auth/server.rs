use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use axum::{extract::Query, response::Html, routing::get, serve, Router};
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    runtime::Runtime,
    sync::oneshot::{self, Sender},
};

use crate::{core::config::Config, AppResult};

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

pub struct AuthServer {
    pub thread: Option<JoinHandle<()>>,
    pub sender: Option<Sender<()>>,
}

impl Default for AuthServer {
    fn default() -> Self {
        Self {
            thread: None,
            sender: None,
        }
    }
}

impl AuthServer {
    pub fn start(&mut self, config: &Config) -> AppResult<()> {
        let (tx, rx) = oneshot::channel::<()>();

        self.sender = Some(tx);

        let rt = Runtime::new()?;
        let host = Arc::new(Self::get_host_from_redirect_url(
            &config.redirect_uri.clone().unwrap_or("".to_string()),
        ));

        let thread = thread::spawn(move || {
            rt.block_on(async {
                let handle_callback = |Query(query): Query<CallbackQuery>| async move {
                    let code = &query.code;
                    let html = format!(
                        "<!DOCTYPE html>
                        <html>
                            <head>
                                <title>Spotify Client TUI</title>
                                <style>
                                    html {{
                                        box-sizing: border-box;
                                        font-size: 16px;
                                    }}

                                    *, *:before, *:after {{
                                        box-sizing: inherit;
                                    }}

                                    body, h1, h2, h3, h4, h5, h6, p, ol, ul {{
                                        margin: 0;
                                        padding: 0;
                                        font-weight: normal;
                                    }}

                                    ol, ul {{
                                        list-style: none;
                                    }}

                                    img {{
                                        max-width: 100%;
                                        height: auto;
                                    }}

                                    body {{
                                        background: #343633;
                                    }}

                                    h1 {{
                                        color: #45B69C;
                                        font-size: 60px;
                                        margin-bottom: 20px;
                                    }}

                                    h2 {{
                                        color: white;
                                    }}

                                    #copied-title {{
                                        color: #45B69C; 
                                        margin-top: 40px;
                                        display: none;
                                    }}

                                    button {{
                                        width: max-content;
                                        font-size: 20px;
                                        padding: 10px;
                                        color: #343633;
                                        font-weight: bold;
                                        border-radius: 10px;
                                        background: #45B69C;
                                        border: none;
                                    }}

                                    .container {{
                                        width: 100%;
                                        height: 100vh; 
                                        display: flex;
                                        flex-direction: column;
                                        justify-content: center;
                                        align-items: center;
                                    }}

                                    .code {{
                                        width: 50%;
                                        overflow: hidden;
                                        word-break: break-all;
                                        background: #7293A0;
                                        color: white;
                                        padding: 30px;
                                        margin-top: 40px;
                                        margin-bottom: 20px;
                                        border-radius: 10px;
                                    }}
                                </style>
                            </head>
                            <body>
                                <div class=\"container\">
                                    <h1>Spotify Client TUI</h1>
                                    <h2>Copy this authentication code to the terminal application in order to sign in.</h2>
                                    <h3 id=\"copied-title\">Copied to clipboard!</h3>

                                    <div class=\"code\">
                                        <h2>{}</h2>
                                    </div>

                                   <button id=\"copy-button\">Copy to Clipboard</button>

                                    <script>
                                        document.getElementById('copy-button').addEventListener('click', function() {{
                                            navigator.clipboard.writeText('{}').then(() => {{
                                               document.getElementById('copied-title').style.display = 'block'; 
                                            }});
                                        }});
                                    </script>
                                </div>
                            </body>
                        </html>
                    ",
                        code,
                        code
                    );

                    Html(html)
                };

                let router: Router = Router::new().route("/callback", get(handle_callback));

                let listener = TcpListener::bind(&*host)
                    .await
                    .expect("failed to create listener for auth server");

                serve(listener, router)
                    .with_graceful_shutdown(async {
                        rx.await.ok();
                    })
                    .await
                    .expect("failed to run auth server")
            })
        });

        self.thread = Some(thread);
        Ok(())
    }

    pub fn stop(&mut self) -> AppResult<()> {
        if let Some(sender) = self.sender.take() {
            sender.send(()).ok();
        }

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        Ok(())
    }

    fn get_host_from_redirect_url(redirect_url: &str) -> String {
        let mut host = String::new();

        if let Some(start) = redirect_url.find("://") {
            let start_index = start + 3;
            if let Some(end) = redirect_url[start_index..].find("/") {
                host = redirect_url[start_index..start_index + end].to_string();
            }
        }

        host
    }
}

impl Drop for AuthServer {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

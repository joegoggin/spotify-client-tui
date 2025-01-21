use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use log::info;
use tiny_http::{Header, Response, Server};

use crate::{core::config::Config, AppResult};

#[derive(Clone)]
pub struct AuthServer {
    pub running: Arc<AtomicBool>,
    pub thread: Arc<Option<JoinHandle<()>>>,
}

impl Default for AuthServer {
    fn default() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            thread: Arc::new(None),
        }
    }
}

impl AuthServer {
    pub fn start(&mut self, config: &Config) -> AppResult<()> {
        info!("#### Starting Auth Server ####");

        self.running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&self.running);

        if let Some(redirect_uri) = config.redirect_uri.clone() {
            let thread = thread::spawn(move || {
                let host = Self::get_host_from_redirect_url(&redirect_uri);
                let server = Server::http(host.clone());

                info!("#### Server Running On {} ####", host);

                if let Ok(server) = server {
                    for request in server.incoming_requests() {
                        if !running_clone.load(Ordering::SeqCst) {
                            break;
                        }

                        let url = request.url();

                        if url.starts_with("/callback") {
                            let code = url
                                .split('?')
                                .nth(1)
                                .unwrap_or("")
                                .split('=')
                                .nth(1)
                                .unwrap_or("")
                                .to_string();

                            let html = format!(
                                "
                                <html>
                                    <head>
                                        <title>Spotify Client TUI</title>
                                    </head>
                                    <body>
                                        <h1>Code: {}</h1>
                                    </body>
                                </html>
                                ",
                                code
                            );

                            let response = Response::from_string(html)
                                .with_header("Content-Type: text/html".parse::<Header>().unwrap());

                            let _ = request.respond(response);
                        }
                    }
                }
            });

            self.thread = Arc::new(Some(thread));
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

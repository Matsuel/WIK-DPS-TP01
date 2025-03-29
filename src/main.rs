use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect("Impossible de démarrer le serveur");

    println!("Serveur démarré sur http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Erreur lors de l'acceptation de la connexion");

        let mut buffer = [0; 1024];
        if let Ok(_) = stream.read(&mut buffer) {
            let request = String::from_utf8_lossy(&buffer);

            if request.starts_with("GET /ping ") {
                let headers = get_headers(&request);
                let json_headers = format_as_json(&headers);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    json_headers.len(),
                    json_headers
                );

                stream
                    .write_all(response.as_bytes())
                    .expect("Erreur d'écriture dans le stream");
            } else {
                let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
                stream
                    .write_all(response.as_bytes())
                    .expect("Erreur d'écriture dans le stream");
            }
        }
    }
}

fn get_headers(request: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();

    for line in request.lines() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.splitn(2, ':');
        if let Some(key) = parts.next() {
            if let Some(value) = parts.next() {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    headers
}

fn format_as_json(headers: &HashMap<String, String>) -> String {
    let mut json = String::from("{");
    let mut first = true;

    for (key, value) in headers {
        if !first {
            json.push(',');
        }
        json.push_str(&format!("\"{}\":\"{}\"", key, value));
        first = false;
    }

    json.push('}');
    json
}

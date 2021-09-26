use std::process::Command;
#[macro_use]
extern crate json;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
#[macro_use]
extern crate rust_embed;
#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();
    println!("Server start at: {}", address);
    if env::consts::OS == "linux" {
        Command::new("xdg-open")
            .arg(address.to_owned() + "?max_time_frames=100")
            .spawn()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
    } else {
        println!("To see chart open ./index.html?max_time_frames=100");
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(b"GET / HTTP/1.1") || buffer.starts_with(b"GET /?") {
        let response_headers = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\n\r\n";
        let index_html = Asset::get("index.html").unwrap();

        stream.write(response_headers.as_bytes()).unwrap();
        stream.write(&index_html).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET /sensors_chart_data") {
        let response_headers = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\n\r\n";

        let json = get_cpu_data();
        stream.write(response_headers.as_bytes()).unwrap();
        stream.write(json.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET /index.js") {
        let response_headers = "HTTP/1.1 200 OK\r\n\r\n";

        let index_js = Asset::get("index.js").unwrap();
        stream.write(response_headers.as_bytes()).unwrap();
        stream.write(&index_js).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";

        stream.write(status_line.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn get_cpu_data() -> String {
    let mut cpu_temp_data = object! {
            "Core 0": 0,
            "Core 1": 0,
            "Core 2": 0,
            "Core 3": 0,
        };

    let sensors_data_json: String = get_sensors_data();
    let parsed = json::parse(&sensors_data_json).unwrap();
    for (_key, value) in parsed["coretemp-isa-0000"].entries() {
        if _key.contains("Core ") == false {
            continue;
        }
        if value.has_key("temp2_input") {
            cpu_temp_data[_key] = value["temp2_input"].clone();
        } else if value.has_key("temp3_input") {
            cpu_temp_data[_key] = value["temp3_input"].clone();
        } else if value.has_key("temp4_input") {
            cpu_temp_data[_key] = value["temp4_input"].clone();
        } else if value.has_key("temp5_input") {
            cpu_temp_data[_key] = value["temp5_input"].clone();
        }
    }
    return  cpu_temp_data.dump();
}

fn get_sensors_data() -> String {
    let output = Command::new("sensors")
        .arg("-j")
        .arg("--no-adapter")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        return String::from_utf8_lossy(&output.stdout).to_string();
    }
    panic!("command sensors failed and stderr was:\n{}", String::from_utf8_lossy(&output.stderr))
}
use std::process::Command;
#[macro_use]
extern crate json;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut cpu_temp_data = object!{
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
        handle_connection(stream, cpu_temp_data.dump());
    }
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

fn handle_connection(mut stream: TcpStream, json: String) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response_headers = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\n\r\n";

    stream.write(response_headers.as_bytes()).unwrap();
    stream.write(json.as_bytes()).unwrap();
    stream.flush().unwrap();
}
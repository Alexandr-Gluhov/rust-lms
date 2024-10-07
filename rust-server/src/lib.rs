use std::{
    error::Error, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

pub struct Server {
    listener: TcpListener
}

pub struct HttpRequestParser {
    request: String,
}

impl HttpRequestParser {
    pub fn new(request: String) -> Self {
        Self{request}
    }

    pub fn get_uri(&self) -> String {
        self.request.split_whitespace().skip(1).next().unwrap().into()
    }
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            listener: TcpListener::bind(addr).expect(&format!("Can't bind address {addr}")),
        }
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {

            let stream = stream.unwrap();

            if let Err(_) = self.handle_connection(stream) {
                println!("Incorrect query");
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {

        let buf_reader = BufReader::new(&stream);

        let request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        let request_reader = HttpRequestParser::new(request);

        println!("{}", request_reader.get_uri());

        let uri = request_reader.get_uri();

        let content;

        let mut headers = Vec::new();
        
        let mut code = "200 OK";
        
        if uri == "/" {
            content = fs::read("/files/index.html")?;
        } else {
            if let Ok(cont) = fs::read(format!("/files{uri}")) {
                content = cont;
                let mime_type = mime_guess::from_path(&uri).first_or_octet_stream().to_string();
                headers.push(format!("Content-Type: {mime_type}"));
            } else {
                mime_guess::from_path("/files/blocks/404.html").first_or_octet_stream().to_string();
                content = fs::read(format!("/files/blocks/404.html"))?;
                code = "404 NOT FOUND";
            }
        }

        

        headers.push(format!("Content-length: {}", content.len()));

        let headers = headers.join(";\r\n");

        let response =
            format!("HTTP/1.1 {code}\r\n{headers}\r\n\r\n");

        stream.write(response.as_bytes()).unwrap();
        stream.write(&content).unwrap();
        Ok(())
    }
}
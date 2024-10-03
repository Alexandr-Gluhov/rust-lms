use std::{
    io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time,
};

mod plugin_manager;
use plugin_manager::PluginManager;

pub struct Server {

    plugin_manager: PluginManager,

}

impl Server {
    pub fn new() -> Self {
        Self {
            plugin_manager: PluginManager::new(),
        }
    }

    pub fn start(&mut self, addr: &str) {
        
        let listener = TcpListener::bind(addr).expect(&format!("Can't bind address {addr}"));

        self.plugin_manager.load_plugin("not_found").unwrap();

        for stream in listener.incoming() {

            let stream = stream.unwrap();

            if let None = self.handle_connection(stream) {
                println!("Incorrect query structure");
            }
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> Option<()> {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let mut code = "200 OK";

        let mut uri = request_line
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .split("/")
            .skip(2);
        let mut plug = uri.next()?;
        let mut query = uri.next()?;
        println!("/{plug}/{query}");

        if plug == "sleeppy" && query == "open" {
            thread::sleep(time::Duration::from_secs(10));
        }

        if !self.plugin_manager.has_plugin(plug) {
            if let Err(_) = self.plugin_manager.load_plugin(plug) {
                query = plug;
                plug = "not_found";
                code = "404 NOT FOUND";
            }
        }

        let component = self.plugin_manager.get_plugin(plug);
        let new_world = self.plugin_manager.instantiate_world(&component).unwrap();

        let content = new_world.call_query(self.plugin_manager.get_store(), query).unwrap();

        let content_length = content.len();

        let response =
            format!("HTTP/1.1 {code}\r\nContent-length: {content_length}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap();
        Some(())
    }
}
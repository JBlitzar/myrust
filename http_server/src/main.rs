use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

struct Header {
    name: String,
    value: String,
}
impl Header {
    fn parse(s: String) -> Self {
        if let Some((name, value)) = s.split_once(": ") { 
            return Self {
                name: String::from(name),
                value: String::from(value)
            }
        }
        Self {
            name: String::from(""),
            value: String::from("")
        }
       
    }
}
#[derive(Debug)]
enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl RequestMethod {
    fn parse(s: &str) -> Self {
        match s {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            "PUT" => RequestMethod::PUT,
            "DELETE" => RequestMethod::DELETE,
            _ => RequestMethod::GET, 
        }
    }
}


struct HTTPRequest {
    method: RequestMethod,
    path: String,
    headers: Vec<Header>
}

impl HTTPRequest {
    fn parse(first_line: &str) -> Self {
        let parts: Vec<&str> = first_line.split(" ").collect();
        let method_str = parts[0];
        let path_str = parts[1];
        Self {
            method: RequestMethod::parse(method_str),
            path: String::from(path_str),
            headers: Vec::new()
        }
    }

    fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }
}



struct RequestHandler {
    request: HTTPRequest,
    pathHandlers: Vec<(String, String)>,
}
impl RequestHandler {
    fn new(request: HTTPRequest) -> Self {
        Self {
            request,
            pathHandlers: Vec::new(),
        }
    }

    fn add_path_handler(&mut self, path: String, handler: String) {
        self.pathHandlers.push((path, handler));
    }

    fn body(&self) -> String {
        for (path, handler) in &self.pathHandlers {
            if &self.request.path == path {
                return handler.clone();
            }
        }
        // 404 Not Found
        String::from("<h1>404 Not Found</h1>")
    }

    fn handle(&self) -> String {
        let body = self.body();
        format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )   
    }
}


fn handle_client(mut stream: TcpStream) {
    
   let buf_reader = BufReader::new(&stream);
    // "trust me bro" approach to get lines that I don't actually understand
    let mut http_request: Vec<String> = buf_reader
        .lines() // get lines
        .map(|result| result.unwrap()) // panic if error, haha
        .take_while(|line| !line.is_empty()) // repeat until empty line (http ends headers with empty line)
        .collect();

    let first_line = &http_request[0];
    println!("Request Line: {}", first_line);

    let mut my_request = HTTPRequest::parse(first_line);
    println!("Method: {:?}", my_request.method);
    println!("Path: {}", my_request.path);

    http_request.remove(0);

    for line in &http_request {
        let h = Header::parse(line.to_string());
        // println!("Header: {} => {}", h.name, h.value);
        my_request.add_header(h);
    }
    
    // println!("Request: {:#?}", http_request);

    let mut handler = RequestHandler::new(my_request);
    handler.add_path_handler(String::from("/"), String::from("<h1>Hello, /!</h1> <br> <a href=\"/page2\">Go to page 2</a>"));
    handler.add_path_handler(String::from("/page2"), String::from("<h1>Hello, /page2!</h1> <br> <a href=\"/\">Go to home</a>"));
    let response = handler.handle();

    stream
        .write_all(response.as_bytes())
        .unwrap();

}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

/*
HTTP server concept. Single threaded.
Use std::net to listen for TCP connections on a specified port.
Read lines blockingly. Parse HTTP requests manually.
Respond with basic HTTP responses.
*/
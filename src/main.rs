pub mod templates;
pub mod file;

use hyper::body::{HttpBody, Bytes};
use hyper::http::Extensions;
use hyper::http::response::Parts;
use hyper::{Body, Request, Response, Server, StatusCode, Version, HeaderMap};
use hyper::service::{make_service_fn, service_fn};

use std::fs::{File, read_dir, self};
use std::{convert::Infallible, net::SocketAddr};

#[tokio::main]
async fn amain() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8087));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_service);
    
    println!("Swissdex is running on {}",addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}",err);
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = format!(".{}",req.uri().to_string());
    let f;
    match File::open(&path) {
        Ok(a) => {f = a;},
        Err(err) => {
            return Ok(Response::new(Body::from(err.to_string())));
        },
    }; 
    
    if f.metadata().unwrap().is_file() {
        let file;
        match fs::read_to_string(&path) {
            Ok(a) => {file = a;}
            Err(err) => {
                return Ok(Response::new(Body::from(format!("{}", err))));
            }
        };
        let b = Bytes::from(file);
        return Ok(Response::new(Body::from(b)));
    } else {
        let dir;
        match read_dir(&path) {
            Ok(a) => {dir = a;}
            Err(err) => {
                return Ok(Response::new(Body::from(format!("{}", err))));
            }
        };
    
        return Ok(Response::new(Body::from(templates::dir(dir, &path))))
    }
}
/*
async fn error_handler(err: routerify::RouteError, req_info: RequestInfo) -> Response<Body> {
    // You can also access the same state from error handler.
    let state = req_info.data::<State>().unwrap();
    println!("State value: {}", state.0);

    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}*/
fn main() {
    amain(); // :troll:
}
#[macro_use]
extern crate lazy_static;

pub mod templates;
pub mod resources;

use hyper::body::{Bytes};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use std::fs::{File, read_dir, self};
use std::{convert::Infallible, net::SocketAddr};

#[tokio::main]
async fn amain() {
    // make sure any static files are loaded.
    //resources::load();

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
    let path_raw = req.uri().to_string();
    let path = format!(".{}",&path_raw);
    let parts = path_raw.split("/").collect::<Vec<&str>>();

    // resources folder 
    //println!("{:?}",parts);
    if parts[1] == "resources" {
        if parts.len() <= 1 {
            return serve_dir(&String::from("./resources"));
        } else {
            match parts[2] {
                "style.css" => {
                    return Ok(
                        Response::new(
                            Body::from(
                                Bytes::from(
                                    resources::STYLE_FILE.to_string()
                                )
                            )
                        )
                    );
                }
                _ => {
                    return Ok(Response::new(Body::from("Not found.")));
                }
            }
        }
    }
    let f;
    match File::open(&path) {
        Ok(a) => {f = a;},
        Err(err) => {
            return Ok(Response::new(Body::from(err.to_string())));
        },
    }; 
    
    
    if f.metadata().unwrap().is_file() {
        return serve_file(&path);
    } else {
        return serve_dir(&path);
    }
}

fn serve_dir(path: &String) -> Result<Response<Body>, Infallible> {
    let dir;
    match read_dir(&path) {
        Ok(a) => {dir = a;}
        Err(err) => {
            return Ok(Response::new(Body::from(format!("{}", err))));
        }
    };

    return Ok(Response::new(Body::from(templates::sanitize_html(templates::dir(dir, &path)))))
}

fn serve_file(path: &String) -> Result<Response<Body>, Infallible> {
    let file;
    match fs::read_to_string(&path) {
        Ok(a) => {
            file = a
            .replace("  ", "")
            .replace("\n","");
        }
        Err(err) => {
            return Ok(Response::new(Body::from(format!("{}", err))));
        }
    };
    let b = Bytes::from(file);
    return Ok(Response::new(Body::from(b)));
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
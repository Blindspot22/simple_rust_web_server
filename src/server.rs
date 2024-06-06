use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::fs;
use std::path::Path;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = match req.uri().path() {
        "/" => "index.html",
        other => &other[1..], 
    };

    if Path::new(path).exists() {
        let contents = fs::read(path).unwrap();
        Ok(Response::new(Body::from(contents)))
    } else {
        let mut not_found = Response::default();
        *not_found.status_mut() = StatusCode::NOT_FOUND;
        not_found.body_mut().extend_from_slice(b"404 Not Found");
        Ok(not_found)
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

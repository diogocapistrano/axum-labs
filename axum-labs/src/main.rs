use axum::{Router, routing::get};
use std::net::SocketAddr;
use axum::response::Html;
use actix_web::{get, Responder, HttpResponse, HttpServer, App, HttpRequest};
use anyhow::{Result, Context, Error};
use std::pin::Pin;
use futures::{Future, FutureExt, TryFutureExt};

#[tokio::main]
async fn main() -> Result<()> {

    let axum_server: Pin<Box<dyn Future<Output = Result<()>>>> = Box::pin(axum_main());
    let actix_server: Pin<Box<dyn Future<Output = Result<()>>>> = Box::pin(actix_main());

    let vec = vec![axum_server, actix_server];

    let (results, failed, _) = futures::future::select_all(vec).await;
    match failed {
        0 => println!("axum: {:?}", results),
        1 => println!("actix {:?}", results),
        _ => println!("generics {:?}", results),
    }
    Ok(())
}

async fn axum_main() -> Result<()>{
        let app = Router::new().route("/", get(handler));

        let socket_address = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("listening on {}", socket_address);
        axum::Server::bind(&socket_address)
            .serve(app.into_make_service())
            .await;
    Ok(())
}

async fn handler() -> Html<&'static str> {
    println!("executing handler....");
    Html("<h1>Hello, World!</h1>")
}

async fn actix_main() -> Result<()>{

    println!("listening on {}", "127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 8080))
    .context("ERROR")?
    .run()
    .await
        .context("ERORO2");
    Ok(())
}

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    println!("executing actix server....");
    "Hello world!\r\n"
}
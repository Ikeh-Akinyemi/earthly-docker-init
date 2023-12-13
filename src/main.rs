use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn create_post() -> impl Responder {
    HttpResponse::Created().body("Post created")
}

async fn read_posts() -> impl Responder {
    HttpResponse::Ok().body("Here are all the posts")
}

async fn update_post() -> impl Responder {
    HttpResponse::Ok().body("Post updated")
}

async fn delete_post() -> impl Responder {
    HttpResponse::Ok().body("Post deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/posts", web::post().to(create_post))
            .route("/posts", web::get().to(read_posts))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

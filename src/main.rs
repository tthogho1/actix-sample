use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::Files;
use tera::{Tera,Context};

mod render;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//async fn manual_hello() -> impl Responder {
//    HttpResponse::Ok().body("Hey there!")
//}

struct AppData {
    tmpl: Tera
}

async fn render_tmpl(data: web::Data<AppData>, req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let mut ctx = Context::new();
    ctx.insert("name", name);
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera =
        Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
        ).unwrap();
        
        App::new()
            .data(AppData {tmpl: tera})
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(render::manual_hello))
            .service(Files::new("/contents", "asset/").show_files_listing())
            .service(
                web::resource("/tmpl/{name}")
                    .route(web::get().to(render_tmpl))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
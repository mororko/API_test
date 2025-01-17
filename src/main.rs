use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::ToSchema;  // Importa el trait ToSchema

// Estructura para representar un mensaje
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]  // Deriva ToSchema
struct Mensaje {
    contenido: String,
}

// Handler para la ruta principal
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Saludo inicial", body = String)
    )
)]
async fn hola() -> impl Responder {
    HttpResponse::Ok().body("¡Hola, mundo!")
}

// Handler para la ruta /mensaje
#[utoipa::path(
    get,
    path = "/mensaje",
    responses(
        (status = 200, description = "Obtener un mensaje", body = Mensaje)
    )
)]
async fn obtener_mensaje() -> impl Responder {
    let mensaje = Mensaje {
        contenido: String::from("Este es un mensaje desde la API"),
    };
    HttpResponse::Ok().json(mensaje)
}

// Estructura para la documentación OpenAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        hola,
        obtener_mensaje
    ),
    components(
        schemas(Mensaje)
    ),
    tags(
        (name = "mi_api", description = "API de ejemplo con Swagger")
    )
)]
struct ApiDoc;

// Función principal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Generar la documentación OpenAPI
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .route("/", web::get().to(hola))
            .route("/mensaje", web::get().to(obtener_mensaje))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
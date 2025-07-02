use warp::{Filter, Reply, Rejection};
use crate::{endpoints, init_tracing, schemas};
use crate::utils::environment;
use crate::utils::validator::{json_body, with_query};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // init tracing
    init_tracing();
    environment::print_stacktrace(false);

    // add endpoint
    let add_get = warp::path!("add")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_query::<endpoints::add::GetQueryParams>())
        .and_then(endpoints::add::get)
        .boxed();
    let add = add_get;

    // subtract endpoint
    let subtract_get = warp::path!("subtract")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_query::<endpoints::subtract::GetQueryParams>())
        .and_then(endpoints::subtract::get)
        .boxed();
    let subtract = subtract_get;

    // multiply endpoint
    let multiply_post = warp::path!("multiply")
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body::<schemas::component_types::Operands>())
        .and_then(endpoints::multiply::post)
        .boxed();
    let multiply = multiply_post;

    // divide endpoint
    let divide_post = warp::path!("divide")
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body::<schemas::component_types::Operands>())
        .and_then(endpoints::divide::post)
        .boxed();
    let divide = divide_post;

    // square endpoint
    let square_get = warp::path!("square" / i32)
        .and(warp::path::end())
        .and(warp::get())
        .and_then(endpoints::square::get)
        .boxed();
    let square = square_get;

    // kubernetes health check endpoints
    let livez = warp::path!("healthcheck" / "livez")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(endpoints::health::livez)
        .boxed();
    let readyz = warp::path!("healthcheck" / "readyz")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(endpoints::health::readyz)
        .boxed();
    let infoz = warp::path!("infoz")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(endpoints::health::infoz)
        .boxed();
    let healthcheck = livez.or(readyz).or(infoz);

    let swagger_doc = warp::path("docs")
        .and(warp::fs::dir("web/swagger-ui-5.25.2/dist"))
        .boxed();

    let none = warp::path!("this exists only because this allows us to be lazy with the gtmpl templates").and(warp::any()).map(|| "This should not happen at any time");
    // Define routes
    let routes = none
        .or(add)
        .or(subtract)
        .or(multiply)
        .or(divide)
        .or(square)
        .or(healthcheck)
        .or(swagger_doc)
        .boxed()
        .recover(endpoints::recover)
        .with(warp::log("api"));
    
    routes
}
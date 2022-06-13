
use axum_todo::configuration::get_configuration;



#[tokio::main]
async fn main(){
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_todo=debug");
    }
    tracing_subscriber::fmt::init();

    let settings = get_configuration().expect("failed to read configuration");
    axum_todo::run(settings).await
}

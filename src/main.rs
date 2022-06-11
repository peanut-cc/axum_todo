
use axum_todo::configuration::get_configuration;


#[tokio::main]
async fn main(){
    let settings = get_configuration().expect("failed to read configuration");
    let addr =  format!(
        "{}:{}",
        settings.application.host,
        settings.application.port
    ).parse().unwrap();
    axum_todo::run(&addr).await
}

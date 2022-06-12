
use axum_todo::configuration::get_configuration;



#[tokio::main]
async fn main(){
    let settings = get_configuration().expect("failed to read configuration");
    axum_todo::run(settings).await
}

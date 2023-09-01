use chrust::{frontend::server::build_and_run_frontend, state::programstate::get_args};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_state = get_args()?;

    if build_and_run_frontend(program_state).await.is_err() {
        panic!("Error while trying to start frontend!");
    } else {
        Ok(())
    }
}

use std::{env, io::Result};

use blog_actix::Blog;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app = Blog::new(8000);
    app.run(database_url).await
}

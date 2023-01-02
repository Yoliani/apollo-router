use anyhow::Result;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().ok();
    apollo_router::main()
}

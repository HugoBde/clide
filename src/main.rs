use anyhow::Result;
use clide::app::App;
use dotenv::dotenv;

fn main() -> Result<()> {
    dotenv().unwrap();
    let api_key = dotenv::var("ANTHROPIC_API_KEY").expect("No ANTHROPIC_API_KEY set");
    let mut app = App::new(api_key);

    app.init()?;

    app.run()?;
    app.clean_up()?;


    Ok(())
}

// fn print_usage() {
//     println!("Usage: ./clide <prompt>");
// }

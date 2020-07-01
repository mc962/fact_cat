pub mod model;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await;
    Ok(())
}

async fn run() {
    let random_facts = client::get_random_facts(1).await.unwrap();
    let random_fact = random_facts.first();

    match random_fact {
        Some(fact) => println!("{}", fact.text),
        None => println!("A cat has swiped your facts")
    }
}
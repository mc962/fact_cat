use crate::fact::notify_fact;

pub mod model;
mod fact;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await;
    Ok(())
}

async fn run() {
    let random_facts = fact::get_random_facts(1).await.unwrap();
    let random_fact = random_facts.first();

    match random_fact {
        Some(fact) => {
            println!("{}", fact.text);
            notify_fact(fact);
        },
        None => println!("A cat has swiped your facts")
    }
}
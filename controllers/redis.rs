#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
use redis::{Commands, Connection, RedisResult};

// Define the Pokémon structure
#[derive(Serialize, Deserialize)]
struct Pokemon {
    name: String,
    pokemon_type: String,
    level: u8,
    abilities: Vec<String>,
}

// Function to connect to Redis
fn connect_to_redis() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?; // Replace with your Redis URL if needed
    client.get_connection()
}

// Handle the POST request
#[post("/pokemon", format = "json", data = "<pokemon>")]
fn handle_post(pokemon: Json<Pokemon>) -> String {
    // Serialize Pokémon data to JSON string
    let pokemon_data = serde_json::to_string(&*pokemon).unwrap();

    // Connect to Redis and save data
    match connect_to_redis() {
        Ok(mut conn) => {
            let key = format!("pokemon:{}", pokemon.name); // Use Pokémon name as key
            let _: () = conn.set(&key, &pokemon_data).unwrap();
            format!("Pokemon {} saved to Redis!", pokemon.name)
        }
        Err(err) => format!("Failed to connect to Redis: {}", err),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handle_post])
}

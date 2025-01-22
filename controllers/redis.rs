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
use redis::{Commands, Connection, RedisResult, Client};

fn connect_to_redis() -> Result<Connection, String> {
    // Try to open a connection to the Redis server
    let client = Client::open("redis://127.0.0.1/").map_err(|e| format!("Failed to create Redis client: {}", e))?;

    // Try to get a connection
    client.get_connection().map_err(|e| format!("Failed to connect to Redis: {}", e))
}


fn get_all_entries() -> RedisResult<HashMap<String, String>> {
    let mut conn = connect_to_redis()?;

    let mut cursor = 0;
    let mut all_entries: HashMap<String, String> = HashMap::new();

    // SCAN through all keys
    loop {
        // Fetch a batch of keys from Redis
        let (new_cursor, keys): (u64, Vec<String>) = conn.scan(cursor).unwrap();
        cursor = new_cursor;

        // Fetch the values for each key
        for key in keys {
            let value: String = conn.get(&key).unwrap();
            all_entries.insert(key, value);
        }

        // If the cursor is zero, we have scanned all keys
        if cursor == 0 {
            break;
        }
    }

    Ok(all_entries)
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
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![handle_post])
}

fn main() {
    match get_all_entries() {
        Ok(entries) => {
            for (key, value) in entries {
                println!("Key: {}, Value: {}", key, value);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

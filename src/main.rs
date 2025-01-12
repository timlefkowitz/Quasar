#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};

// Define the Pokémon structure
#[derive(Serialize, Deserialize)]
struct Pokemon {
    name: String,
    pokemon_type: String, // e.g., Fire, Water, Grass
    level: u8,
    abilities: Vec<String>, // List of abilities
}

// Handle the POST request
#[post("/pokemon", format = "json", data = "<pokemon>")]
fn handle_post(pokemon: Json<Pokemon>) -> Json<Pokemon> {
    // Return the same Pokémon data as a JSON response
    Json(Pokemon {
        name: pokemon.name.clone(),
        pokemon_type: pokemon.pokemon_type.clone(),
        level: pokemon.level,
        abilities: pokemon.abilities.clone(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handle_post])
}

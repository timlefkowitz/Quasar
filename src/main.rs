#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{fs::NamedFile, get, routes, Rocket, Build};
use std::path::{Path, PathBuf};

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

// Serve the front-end HTML file at "/post"
#[get("/post")]
async fn post_page() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// Serve static assets like CSS or JavaScript
#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![post_page, static_files,handle_post])
}

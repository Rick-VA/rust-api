#[macro_use]extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
struct Song {
    artist: String,
    title: String,
    album: String,
}

static  mut SONGS: Vec<Song> = Vec::new();

#[get("/")]
fn index() -> Json<Vec<Song>> {
    Json(unsafe { SONGS.clone() })
}

#[post("/song", format = "json", data = "<song>")]
fn post_song(song: Json<Song>) -> Json<Song> {
    let song = song.0;
    unsafe {
        SONGS.push(song.clone());
    }
    Json(song)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_song])
}

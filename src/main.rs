#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
struct Song {
    id: usize,
    artist: String,
    title: String,
    album: String,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
static mut SONGS: Vec<Song> = Vec::new();

#[get("/")]
fn index() -> Json<Vec<Song>> {
    Json(unsafe { SONGS.clone() })
}

#[post("/song", format = "json", data = "<song>")]
fn post_song(song: Json<Song>) -> Json<Song> {
    let mut song = song.0;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    song.id = id;
    unsafe {
        SONGS.push(song.clone());
    }
    Json(song)
}

#[put("/song/<id>", format = "json", data = "<song>")]
fn put_song(id: usize, song: Json<Song>) -> Result<Json<Song>, ()> {
    let mut updated_song = song.0;
    updated_song.id = id;

    unsafe {
        if let Some(existing_song) = SONGS.iter_mut().find(|s| s.id == id) {
            *existing_song = updated_song.clone();
            Ok(Json(updated_song))
        } else {
            Err(())
        }
    }
}

#[delete("/song/<id>")]
fn delete_song(id: usize) -> Result<&'static str, ()> {
    unsafe {
        if let Some(index) = SONGS.iter().position(|s| s.id == id) {
            SONGS.remove(index);
            Ok("Song deleted successfully")
        } else {
            Err(())
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, post_song, put_song, delete_song])
}

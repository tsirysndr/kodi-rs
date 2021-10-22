use surf::Client;

pub struct AudioLibraryService {
  client: Client,
}

impl AudioLibraryService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn clean(&self) {}

  pub fn export(&self) {}

  pub fn get_album_details(&self) {}

  pub fn get_albums(&self) {}

  pub fn get_artist_details(&self) {}

  pub fn get_artists(&self) {}

  pub fn get_available_art(&self) {}

  pub fn get_available_art_types(&self) {}

  pub fn get_genres(&self) {}

  pub fn get_properties(&self) {}

  pub fn get_recently_added_albums(&self) {}

  pub fn get_recently_added_songs(&self) {}

  pub fn get_recently_played_songs(&self) {}

  pub fn get_roles(&self) {}

  pub fn get_song_details(&self) {}

  pub fn get_songs(&self) {}

  pub fn get_sources(&self) {}

  pub fn scan(&self) {}

  pub fn set_album_details(&self) {}

  pub fn set_artist_details(&self) {}

  pub fn set_song_details(&self) {}
}

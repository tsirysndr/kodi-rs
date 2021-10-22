use surf::Client;

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn add(&self) {}

  pub fn clear(&self) {}

  pub fn get_items(&self) {}

  pub fn get_playlists(&self) {}

  pub fn get_properties(&self) {}

  pub fn insert(&self) {}

  pub fn remove(&self) {}

  pub fn swap(&self) {}
}

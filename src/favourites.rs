use surf::Client;

pub struct FavouritesService {
  client: Client,
}

impl FavouritesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn add_favourite(&self) {}

  pub fn get_favourites(&self) {}
}

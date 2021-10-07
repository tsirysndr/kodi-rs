use surf::Client;

pub struct FavouritesService {
  client: Client,
}

impl FavouritesService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}

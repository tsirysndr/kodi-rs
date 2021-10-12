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
}

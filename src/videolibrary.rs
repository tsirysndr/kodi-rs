use surf::Client;

pub struct VideoLibraryService {
  client: Client,
}

impl VideoLibraryService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

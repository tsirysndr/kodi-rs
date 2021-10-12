use surf::Client;

pub struct FilesService {
  client: Client,
}

impl FilesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

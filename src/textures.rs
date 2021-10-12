use surf::Client;

pub struct TexturesService {
  client: Client,
}

impl TexturesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

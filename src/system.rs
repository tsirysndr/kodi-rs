use surf::Client;

pub struct SystemService {
  client: Client,
}

impl SystemService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

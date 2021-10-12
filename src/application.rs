use surf::Client;

pub struct ApplicationService {
  client: Client,
}

impl ApplicationService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

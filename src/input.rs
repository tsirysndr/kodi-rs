use surf::Client;

pub struct InputService {
  client: Client,
}

impl InputService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

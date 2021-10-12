use surf::Client;

pub struct PvrService {
  client: Client,
}

impl PvrService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

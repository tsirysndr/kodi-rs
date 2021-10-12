use surf::Client;

pub struct AddonsService {
  client: Client,
}

impl AddonsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

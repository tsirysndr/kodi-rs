use surf::Client;

pub struct ApplicationService {
  client: Client,
}

impl ApplicationService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { client.as_ref().unwrap().clone() },
    }
  }
}

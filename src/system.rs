use surf::Client;

pub struct SystemService {
  client: Client,
}

impl SystemService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}

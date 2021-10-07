use surf::Client;

pub struct PvrService {
  client: Client,
}

impl PvrService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}

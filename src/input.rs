use surf::Client;

pub struct InputService {
  client: Client,
}

impl InputService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}

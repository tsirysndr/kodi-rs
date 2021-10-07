use surf::Client;

pub struct GuiService {
  client: Client,
}

impl GuiService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}

use surf::Client;

pub struct GuiService {
  client: Client,
}

impl GuiService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}

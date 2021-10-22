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

  pub fn back(&self) {}

  pub fn button_event(&self) {}

  pub fn context_menu(&self) {}

  pub fn down(&self) {}

  pub fn execute_action(&self) {}

  pub fn home(&self) {}

  pub fn info(&self) {}

  pub fn left(&self) {}

  pub fn right(&self) {}

  pub fn select(&self) {}

  pub fn send_text(&self) {}

  pub fn show_codec(&self) {}

  pub fn show_osd(&self) {}

  pub fn show_player_process_info(&self) {}

  pub fn up(&self) {}
}

use surf::Client;

pub struct PlayerService {
  client: Client,
}

impl PlayerService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn add_subtitle(&self) {}

  pub fn get_active_players(&self) {}

  pub fn get_properties(&self) {}

  pub fn get_view_mode(&self) {}

  pub fn go_to(&self) {}

  pub fn r#move(&self) {}

  pub fn open(&self) {}

  pub fn play_pause(&self) {}

  pub fn rotate(&self) {}

  pub fn seek(&self) {}

  pub fn set_audio_stream(&self) {}

  pub fn set_partymode(&self) {}

  pub fn set_repeat(&self) {}

  pub fn set_shuffle(&self) {}

  pub fn set_speed(&self) {}

  pub fn set_subtitle(&self) {}

  pub fn set_video_stream(&self) {}

  pub fn set_view_mode(&self) {}

  pub fn stop(&self) {}

  pub fn zoom(&self) {}
}

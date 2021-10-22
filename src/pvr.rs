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

  pub fn add_timer(&self) {}

  pub fn delete_timer(&self) {}

  pub fn get_broadcast_details(&self) {}

  pub fn get_broadcast_is_playable(&self) {}

  pub fn get_broadcasts(&self) {}

  pub fn get_channel_details(&self) {}

  pub fn get_channel_group_details(&self) {}

  pub fn get_channel_groups(&self) {}

  pub fn get_channels(&self) {}

  pub fn get_clients(&self) {}

  pub fn get_properties(&self) {}

  pub fn get_recording_details(&self) {}

  pub fn get_recordings(&self) {}

  pub fn get_timer_details(&self) {}

  pub fn get_timers(&self) {}

  pub fn get_record(&self) {}

  pub fn scan(&self) {}

  pub fn toggle_timer(&self) {}
}

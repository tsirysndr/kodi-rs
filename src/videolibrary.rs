use surf::Client;

pub struct VideoLibraryService {
  client: Client,
}

impl VideoLibraryService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn clean(&self) {}

  pub fn export(&self) {}

  pub fn get_available_art(&self) {}

  pub fn get_available_art_types(&self) {}

  pub fn get_episode_details(&self) {}

  pub fn get_episodes(&self) {}

  pub fn get_genres(&self) {}

  pub fn get_in_progress_tv_shows(&self) {}

  pub fn get_movie_details(&self) {}

  pub fn get_movie_set_details(&self) {}

  pub fn get_movie_sets(&self) {}

  pub fn get_movies(&self) {}

  pub fn get_music_video_details(&self) {}

  pub fn get_music_videos(&self) {}

  pub fn get_recently_added_episodes(&self) {}

  pub fn get_recently_added_movies(&self) {}

  pub fn get_recently_added_music_videos(&self) {}

  pub fn get_season_details(&self) {}

  pub fn get_seasons(&self) {}

  pub fn get_tv_show_details(&self) {}

  pub fn get_tv_shows(&self) {}

  pub fn get_tags(&self) {}

  pub fn get_refresh_episode(&self) {}

  pub fn get_refresh_movie(&self) {}

  pub fn get_refresh_music_video(&self) {}

  pub fn get_refresh_tv_show(&self) {}

  pub fn remove_episode(&self) {}

  pub fn remove_movie(&self) {}

  pub fn remove_music_video(&self) {}

  pub fn remove_tv_show(&self) {}

  pub fn scan(&self) {}

  pub fn set_episode_details(&self) {}

  pub fn set_movie_details(&self) {}

  pub fn set_movie_set_details(&self) {}

  pub fn set_music_video_details(&self) {}

  pub fn set_season_details(&self) {}

  pub fn set_tv_show_details(&self) {}
}

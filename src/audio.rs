use bevy::{
    asset::Handle,
    audio::{AudioPlayer, AudioSource, PlaybackSettings},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
};
use rand::RngExt;
use smart_default::SmartDefault;

#[derive(Resource, SmartDefault)]
pub struct AudioFactory {
    requests: Vec<AudioBundle>,
    #[default(1.0)]
    master_volume: f32,
    #[default(1.0)]
    sound_effects_volume: f32,
    #[default(1.0)]
    music_volume: f32,
    #[default(1.0)]
    ui_volume: f32,
}

#[derive(Clone)]
pub struct AudioBundle {
    source: Handle<AudioSource>,
    category: AudioCategory,
}

#[derive(Clone)]
pub enum AudioCategory {
    SOUND_EFFECT,
    MUSIC,
    UI,
}

impl AudioBundle {
    pub fn new(source: Handle<AudioSource>, category: AudioCategory) -> Self {
        Self {
            source, 
            category,
        }
    }
}

impl AudioFactory {
    pub fn play_sound(&mut self, bundle: AudioBundle) {
        self.requests.push(bundle);
    }
}

pub fn process_audio_requests(mut commands: Commands, mut audio: ResMut<AudioFactory>) {
    let master_volume = audio.master_volume;
    let sound_effects_volume = audio.sound_effects_volume;
    let music_volume = audio.music_volume;
    let ui_volume = audio.ui_volume;

    for request in audio.requests.drain(..) {
        let speed = rand::rng().random_range(0.9..=1.1);
        let volume = match request.category {
            AudioCategory::SOUND_EFFECT => (master_volume * sound_effects_volume).clamp(0.0, 1.0),
            AudioCategory::MUSIC => (master_volume * music_volume).clamp(0.0, 1.0),
            AudioCategory::UI => (master_volume * ui_volume).clamp(0.0, 1.0),
        };

        commands.spawn((
            AudioPlayer::new(request.source),
            PlaybackSettings::DESPAWN
                .with_speed(speed)
                .with_volume(bevy::audio::Volume::Linear(volume)),
        ));
    }
}

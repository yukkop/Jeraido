use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

use crate::{
    core::{AudioAssets, CoreGameState},
    settings::ApplySettings,
};

const MINIMAL_DELAY: f32 = 15.;
const MAXIMAL_DELAY: f32 = 90.;

#[derive(Default, Resource, Deref, DerefMut)]
struct MusicTimer(Timer);

#[derive(Default, Resource)]
pub struct MenuMusic {
    source_handle: Handle<AudioSource>,
    pub instance_handle: Handle<AudioInstance>,
    duration: Option<Duration>,
}

pub struct MusicPlugins;

impl Plugin for MusicPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMusic>()
            .init_resource::<MusicTimer>()
            .add_systems(OnExit(CoreGameState::PrimaryLoad), setup)
            .add_systems(Update, play_music.run_if(in_state(CoreGameState::Hub)));
    }
}

fn setup(
    mut commands: Commands,
    mut menu_music: ResMut<MenuMusic>,
    audio_assets: Res<AudioAssets>,
) {
    commands.insert_resource(MusicTimer(Timer::from_seconds(0.0, TimerMode::Repeating)));
    menu_music.source_handle = audio_assets.background.clone();
}

fn play_music(
    time: Res<Time>,
    audio: Res<Audio>,
    mut music_timer: ResMut<MusicTimer>,
    mut menu_music: ResMut<MenuMusic>,
    audio_sources: Res<Assets<AudioSource>>,
    mut event: EventWriter<ApplySettings>,
) {
    if music_timer.tick(time.delta()).just_finished() {
        if menu_music.duration.is_none() {
            if let Some(audio_source) = audio_sources.get(&menu_music.source_handle) {
                let duration = audio_source.sound.duration();
                menu_music.duration = Some(duration);
            } else {
                return;
            }
        }

        menu_music.instance_handle = audio.play(menu_music.source_handle.clone()).handle();

        let delay = thread_rng().gen_range(MINIMAL_DELAY..MAXIMAL_DELAY)
            + menu_music.duration.unwrap().as_secs_f32();
        music_timer.set_duration(Duration::from_secs_f32(delay));
        music_timer.reset();

        event.send(ApplySettings);
    }
}

use std::env;

use alsa::{mixer::SelemId, Mixer};

use self::volume_mapper::VolumeMapper;

mod error;
mod volume_mapper;

fn main() {
    let mixer_name = env::var("MV_CARD_NAME").unwrap_or_else(|_| "hw:0".to_string());
    let source_selem_name = env::var("MV_SOURCE_SELEM").unwrap_or_else(|_| "Speaker".to_string());
    let target_selem_name = env::var("MV_TARGET_SELEM").unwrap_or_else(|_| "Headphone".to_string());
    let mixer = Mixer::new(&mixer_name, false).unwrap();
    let source = mixer
        .find_selem(&SelemId::new(&source_selem_name, 0))
        .unwrap();
    let target = mixer
        .find_selem(&SelemId::new(&target_selem_name, 0))
        .unwrap();
    // let volume_range_master = selem_master.get_playback_volume_range();
    let source_range_db = source.get_playback_db_range();
    let target_range_db = target.get_playback_db_range();
    let volume_mapper = VolumeMapper::new(source_range_db, target_range_db);
    loop {
        volume_mapper.map_volume(&source, &target);
        mixer.wait(None).unwrap();
        mixer.handle_events().unwrap();
    }
}

use alsa::{mixer::SelemId, Mixer};

use self::volume_mapper::VolumeMapper;

mod error;
mod volume_mapper;

fn main() {
    let mixer = Mixer::new("hw:0", false).unwrap();
    let source = mixer.find_selem(&SelemId::new("Speaker", 0)).unwrap();
    let target = mixer.find_selem(&SelemId::new("Headphone", 0)).unwrap();
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

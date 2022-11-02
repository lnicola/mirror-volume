/*
 * Copyright (c) 2010 Clemens Ladisch <clemens@ladisch.de>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

// https://github.com/alsa-project/alsa-utils/blob/master/alsamixer/volume_mapping.c

use alsa::{
    mixer::{MilliBel, Selem, SelemChannelId},
    Round,
};

use crate::error::Error;

pub struct VolumeMapper {
    source_range_db: (MilliBel, MilliBel),
    target_range_db: (MilliBel, MilliBel),
}

impl VolumeMapper {
    pub fn new(
        source_range_db: (MilliBel, MilliBel),
        target_range_db: (MilliBel, MilliBel),
    ) -> Self {
        Self {
            source_range_db,
            target_range_db,
        }
    }

    pub fn map_volume(&self, source: &Selem, target: &Selem) {
        let _ = Self::map_playback_switch(source, target).map_err(|e| log::error!("{}", e));

        // let volume = selem_master
        //     .get_playback_volume(SelemChannelId::Unknown)
        //     .unwrap();
        let db_master = source.get_playback_vol_db(SelemChannelId::mono()).unwrap();

        let mut normalized_master =
            10_f32.powf((db_master.0 as f32 - self.source_range_db.1 .0 as f32) / 6000.0);
        let min_norm_master = 10_f32
            .powf((self.source_range_db.0 .0 as f32 - self.source_range_db.1 .0 as f32) / 6000.0);
        let min_norm_headphones = 10_f32
            .powf((self.target_range_db.0 .0 as f32 - self.target_range_db.1 .0 as f32) / 6000.0);
        normalized_master = (normalized_master - min_norm_master) / (1f32 - min_norm_master);
        let headphones = normalized_master * (1.0 - min_norm_headphones) + min_norm_headphones;
        let headphones =
            (6000.0 * headphones.log10() + self.target_range_db.1 .0 as f32).round() as i64;
        target
            .set_playback_db_all(MilliBel(headphones), Round::Floor)
            .unwrap();

        // 	normalized = pow(10, (value - max) / 6000.0);
        // 	if (min != SND_CTL_TLV_DB_GAIN_MUTE) {
        // 		min_norm = pow(10, (min - max) / 6000.0);
        // 		normalized = (normalized - min_norm) / (1 - min_norm);
        // 	}
        //
        // ---
        //
        // 	if (min != SND_CTL_TLV_DB_GAIN_MUTE) {
        // 		min_norm = pow(10, (min - max) / 6000.0);
        // 		volume = volume * (1 - min_norm) + min_norm;
        // 	}
        // 	value = lrint_dir(6000.0 * log10(volume)) + max;
        // 	return snd_mixer_selem_set_playback_dB(elem, channel, value);
    }

    fn map_playback_switch(source: &Selem, target: &Selem) -> Result<(), Error> {
        let switch = source
            .get_playback_switch(SelemChannelId::mono())
            .map_err(Error::GetPlaybackSwitch)?;
        target
            .set_playback_switch_all(switch)
            .map_err(Error::SetPlaybackSwitch)?;
        Ok(())
    }
}

// static double get_normalized_volume(snd_mixer_elem_t *elem,
// 				    snd_mixer_selem_channel_id_t channel)
// {
// 	long min, max, value;
// 	double normalized, min_norm;
// 	int err;

// 	err = snd_mixer_selem_get_playback_dB_range(elem, &min, &max);
// 	if (err < 0 || min >= max) {
// 		err = snd_mixer_selem_get_playback_volume_range(elem, &min, &max);
// 		if (err < 0 || min == max)
// 			return 0;

// 		err = snd_mixer_selem_get_playback_volume(elem, channel, &value);
// 		if (err < 0)
// 			return 0;

// 		return (value - min) / (double)(max - min);
// 	}

// 	err = snd_mixer_selem_get_playback_dB(elem, channel, &value);
// 	if (err < 0)
// 		return 0;

// 	if (use_linear_dB_scale(min, max))
// 		return (value - min) / (double)(max - min);

// 	normalized = pow(10, (value - max) / 6000.0);
// 	if (min != SND_CTL_TLV_DB_GAIN_MUTE) {
// 		min_norm = pow(10, (min - max) / 6000.0);
// 		normalized = (normalized - min_norm) / (1 - min_norm);
// 	}

// 	return normalized;
// }

// static int set_normalized_volume(snd_mixer_elem_t *elem,
// 				 snd_mixer_selem_channel_id_t channel,
// 				 double volume)
// {
// 	long min, max, value;
// 	double min_norm;
// 	int err;

// 	err = snd_mixer_selem_get_playback_dB_range(elem, &min, &max);
// 	if (err < 0 || min >= max) {
// 		err = snd_mixer_selem_get_playback_volume_range(elem, &min, &max);
// 		if (err < 0)
// 			return err;

// 		value = lrint_dir(volume * (max - min)) + min;
// 		return snd_mixer_selem_set_playback_volume(elem, channel, value);
// 	}

// 	if (use_linear_dB_scale(min, max)) {
// 		value = lrint_dir(volume * (max - min)) + min;
// 		return snd_mixer_selem_set_playback_dB(elem, channel, value);
// 	}

// 	if (min != SND_CTL_TLV_DB_GAIN_MUTE) {
// 		min_norm = pow(10, (min - max) / 6000.0);
// 		volume = volume * (1 - min_norm) + min_norm;
// 	}
// 	value = lrint_dir(6000.0 * log10(volume)) + max;
// 	return snd_mixer_selem_set_playback_dB(elem, channel, value);
// }

// #define MAX_LINEAR_DB_SCALE	24

// static inline bool use_linear_dB_scale(long dBmin, long dBmax)
// {
// 	return dBmax - dBmin <= MAX_LINEAR_DB_SCALE * 100;
// }

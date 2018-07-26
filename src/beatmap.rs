use std::cell::RefCell;
use std::rc::Rc;

use failure::Error;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use HitObject;
use HitObjectKind;
use Hitsound;
use Mode;
use SampleSet;
use TimeLocation;
use TimingPoint;

#[derive(Debug)]
pub struct BeatmapSet {}

#[derive(Debug)]
pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
}

/// Represents a single beatmap.
#[derive(Debug)]
pub struct Beatmap {
    pub version: u32,

    pub audio_filename: String,
    pub audio_leadin: u32,
    pub preview_time: u32,
    pub countdown: bool,
    pub sample_set: SampleSet,
    pub stack_leniency: f64,
    pub mode: Mode,
    pub letterbox_in_breaks: bool,
    pub widescreen_storyboard: bool,

    pub bookmarks: Vec<i32>,
    pub distance_spacing: f64,
    pub beat_divisor: u8,
    pub grid_size: u8,
    pub timeline_zoom: f64,

    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub difficulty_name: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,

    pub hit_objects: Vec<Rc<RefCell<HitObject>>>,
    pub timing_points: Vec<Rc<RefCell<TimingPoint>>>,
}

impl Beatmap {
    pub(crate) fn associate_hitobjects(&mut self) {
        /*
        let mut curr = 1;
        for obj_ref in self.hit_objects.iter() {
            if curr >= self.timing_points.len() {
                break;
            }
            let obj = obj_ref.borrow();
            let obj_time = obj.start_time.into_milliseconds();
            // should we advance?
            let next_time = self.timing_points[curr].borrow().time.into_milliseconds();
            if obj_time >= next_time {
                curr += 1;
            }
            // assign timing point
            let tp = &self.timing_points[curr - 1].borrow();

            let bpm = tp.get_bpm();
            let meter = tp.get_meter();
            let (measures, frac) = obj.start_time.approximate(&tp.time, bpm, meter);
            let mut obj_mut = (**obj_ref).borrow_mut();
            obj_mut.start_time = TimeLocation::Relative {
                time: Box::new(tp.time.clone()),
                bpm: tp.get_bpm(),
                meter: tp.get_meter(),
                measures,
                frac,
            };
        }
        */
    }

    /// Returns a list of this beatmap's hitsounds.
    ///
    /// This will also return hitsounds that occur on parts of objects, for example on slider
    /// bodies or slider ends. If a hitsound occurs on a spinner, the only "sound" that's counted
    /// is the moment that the spinner ends.
    pub fn get_hitsounds(&self) -> Result<Vec<Hitsound>, Error> {
        let mut hitsounds = Vec::new();
        for obj_ref in self.hit_objects.iter() {
            let obj = obj_ref.borrow();
            match obj.kind {
                HitObjectKind::Slider { .. } => {
                    // TODO: calculate middle hitsounds
                    hitsounds.push(obj.hitsound.clone());
                }
                _ => hitsounds.push(obj.hitsound.clone()),
            }
        }
        Ok(hitsounds)
    }
}

impl Serialize for Beatmap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Beatmap", 1)?;
        state.serialize_field("version", &self.version)?;

        // state.serialize_field("timing_points", &self.timing_points)?;
        state.end()
    }
}
use std::collections::BTreeSet;

use anyhow::Result;

use crate::{AbsoluteTime, SampleSet, TimeLocation, TimingPoint, TimingPointKind};

impl TimingPoint {
    /// Creates a TimingPoint from the *.osz format
    pub fn from_osz(input: impl AsRef<str>, parent: &Option<TimingPoint>) -> Result<TimingPoint> {
        let parts = input.as_ref().split(',').collect::<Vec<_>>();

        let timestamp = parts[0].parse::<i32>()?;
        let mpb = parts[1].parse::<f64>()?;
        let meter = parts[2].parse::<u32>()?;
        let sample_set = parts[3].parse::<i32>()?;
        let sample_index = parts[4].parse::<u32>()?;
        let volume = parts[5].parse::<u16>()?;
        let inherited = parts[6].parse::<i32>()? == 0;
        let kiai = parts[7].parse::<i32>()? > 0;

        // calculate bpm from mpb
        let bpm = 60_000.0 / mpb;
        let time = TimeLocation::Absolute(AbsoluteTime::new(timestamp));

        let timing_point = TimingPoint {
            kind: if inherited {
                assert!(parent.is_some());
                TimingPointKind::Inherited {
                    parent: parent.clone().map(Box::new),
                    slider_velocity: 0.0, // TODO: calculate this from mpb
                }
            } else {
                TimingPointKind::Uninherited {
                    bpm,
                    meter,
                    children: BTreeSet::new(),
                }
            },
            kiai,
            sample_set: match sample_set {
                0 => SampleSet::None,
                1 => SampleSet::Normal,
                2 => SampleSet::Soft,
                3 => SampleSet::Drum,
                _ => panic!("Invalid sample set '{}'.", sample_set),
            },
            mpb,
            sample_index,
            volume,
            time: match parent {
                Some(parent) => time.into_relative(parent),
                None => time,
            },
        };

        Ok(timing_point)
    }

    /// Serializes this TimingPoint into the *.osz format.
    pub fn as_osz(&self) -> Result<String> {
        let inherited = match self.kind {
            TimingPointKind::Inherited { .. } => 0,
            TimingPointKind::Uninherited { .. } => 1,
        };
        let line = format!(
            "{},{},{},{},{},{},{},{}",
            self.time.as_milliseconds(),
            self.mpb,
            self.get_meter(),
            self.sample_set.clone() as i32,
            self.sample_index,
            self.volume,
            inherited,
            if self.kiai { 1 } else { 0 },
        );
        Ok(line)
    }
}
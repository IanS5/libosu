use Point;
use TimeLocation;
use TimingPoint;

/// Distinguishes between different types of slider splines.
#[derive(Debug)]
pub enum SliderSplineKind {
    /// Linear is the most straightforward, and literally consists of two endpoints.
    Linear,
    /// Bezier is more complex, using control points to create smooth curves.
    Bezier,
    /// Catmull is a deprecated slider spline used mainly in older maps (looks ugly btw).
    Catmull,
    /// Perfect (circle) splines are circles circumscribed around three control points.
    Perfect,
}

/// Distinguishes between different types of hit objects.
#[derive(Debug)]
pub enum HitObjectKind<'map> {
    /// Regular hit circle.
    Circle,
    /// Slider.
    Slider {
        kind: SliderSplineKind,
        control: Vec<Point<i32>>,
    },
    /// Spinner.
    Spinner { end_time: TimeLocation<'map> },
}

/// Represents a single hit object.
#[derive(Debug)]
pub struct HitObject<'map> {
    pub pos: Point<i32>,
    pub start_time: TimeLocation<'map>,
    pub kind: HitObjectKind<'map>,
    pub new_combo: bool,
    /// Reference to the timing point under which this HitObject belongs.
    pub timing_point: Option<&'map TimingPoint<'map>>,
    /// The number of combo colors to skip
    pub skip_color: i32,
    /// WIP
    pub hitsound: u32,
}

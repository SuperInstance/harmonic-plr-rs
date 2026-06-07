//! # harmonic-plr-rs
//!
//! PLR group operations for neo-Riemannian chord transformations.
//!
//! The Parallel-Lead-Relative (PLR) group acts on major and minor triads,
//! generating all 24 triads through combinations of three fundamental
//! transformations: P (Parallel), L (Leittonwechsel), and R (Relative).
//!
//! ## Transformations
//! - **P** (Parallel): Toggles between parallel major/minor (same root)
//!   P(C major) = C minor
//! - **L** (Leittonwechsel): Moves by leading-tone exchange
//!   L(C major) = E minor
//! - **R** (Relative): Moves to relative major/minor
//!   R(C major) = A minor
//!
//! The PLR group is isomorphic to D₁₂ (dihedral group of order 24) and
//! all 24 major/minor triads are reachable from any single triad.

/// Pitch class represented as an integer 0-11 (C=0, C#=1, ..., B=11).
pub type PitchClass = u8;

/// Chord quality: Major or Minor triad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    Major,
    Minor,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Major => write!(f, "major"),
            Quality::Minor => write!(f, "minor"),
        }
    }
}

/// Named pitch classes for convenience.
pub mod note {
    use super::PitchClass;
    pub const C: PitchClass = 0;
    pub const CS: PitchClass = 1;
    pub const D: PitchClass = 2;
    pub const DS: PitchClass = 3;
    pub const E: PitchClass = 4;
    pub const F: PitchClass = 5;
    pub const FS: PitchClass = 6;
    pub const G: PitchClass = 7;
    pub const GS: PitchClass = 8;
    pub const A: PitchClass = 9;
    pub const AS: PitchClass = 10;
    pub const B: PitchClass = 11;
}

/// Convert a pitch class to a note name string.
pub fn pitch_class_name(pc: PitchClass) -> &'static str {
    match pc {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => "?",
    }
}

/// A triad consisting of a root pitch class and a quality (major or minor).
///
/// A major triad has intervals {0, 4, 7} from the root.
/// A minor triad has intervals {0, 3, 7} from the root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    /// Root pitch class (0-11).
    pub root: PitchClass,
    /// Major or Minor quality.
    pub quality: Quality,
}

impl Chord {
    /// Create a new chord from a root and quality.
    pub fn new(root: PitchClass, quality: Quality) -> Self {
        Chord {
            root: root % 12,
            quality,
        }
    }

    /// Create a major triad.
    pub fn major(root: PitchClass) -> Self {
        Chord::new(root, Quality::Major)
    }

    /// Create a minor triad.
    pub fn minor(root: PitchClass) -> Self {
        Chord::new(root, Quality::Minor)
    }

    /// Return the three pitch classes of this triad.
    pub fn pitch_classes(&self) -> [PitchClass; 3] {
        match self.quality {
            Quality::Major => [(self.root) % 12, (self.root + 4) % 12, (self.root + 7) % 12],
            Quality::Minor => [(self.root) % 12, (self.root + 3) % 12, (self.root + 7) % 12],
        }
    }

    /// The third of the triad.
    pub fn third(&self) -> PitchClass {
        match self.quality {
            Quality::Major => (self.root + 4) % 12,
            Quality::Minor => (self.root + 3) % 12,
        }
    }

    /// The fifth of the triad.
    pub fn fifth(&self) -> PitchClass {
        (self.root + 7) % 12
    }

    /// Check if this chord contains a given pitch class.
    pub fn contains(&self, pc: PitchClass) -> bool {
        self.pitch_classes().contains(&(pc % 12))
    }

    /// Number of common tones with another chord.
    pub fn common_tones(&self, other: &Chord) -> usize {
        let pcs = self.pitch_classes();
        let other_pcs = other.pitch_classes();
        pcs.iter().filter(|p| other_pcs.contains(p)).count()
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            pitch_class_name(self.root),
            self.quality
        )
    }
}

/// PLR operations: the three generators of the neo-Riemannian PLR group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlrOp {
    /// Parallel: toggle major/minor, keep root.
    /// P(C major) = C minor, P(C minor) = C major
    P,
    /// Leittonwechsel (Leading-tone exchange): swap root and third.
    /// L(C major) = E minor, L(A minor) = C major
    L,
    /// Relative: swap root and fifth.
    /// R(C major) = A minor, R(A minor) = C major
    R,
}

/// Apply a single PLR operation to a chord.
pub fn apply_plr(chord: Chord, op: PlrOp) -> Chord {
    match op {
        PlrOp::P => Chord::new(chord.root, match chord.quality {
            Quality::Major => Quality::Minor,
            Quality::Minor => Quality::Major,
        }),
        PlrOp::L => match chord.quality {
            Quality::Major => Chord::minor((chord.root + 4) % 12), // third becomes new root, minor
            Quality::Minor => Chord::major((chord.root + 8) % 12), // preserves third & fifth, root moves
        },
        PlrOp::R => match chord.quality {
            Quality::Major => Chord::minor((chord.root + 9) % 12), // relative minor: down a minor third
            Quality::Minor => Chord::major((chord.root + 3) % 12), // relative major: up a minor third
        },
    }
}

/// A sequence of PLR operations forming a group element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlrSequence {
    ops: Vec<PlrOp>,
}

impl PlrSequence {
    /// The identity element (empty sequence).
    pub fn identity() -> Self {
        PlrSequence { ops: vec![] }
    }

    /// Create from a slice of PLR operations.
    pub fn from_ops(ops: &[PlrOp]) -> Self {
        PlrSequence { ops: ops.to_vec() }
    }

    /// Compose two sequences: self followed by other.
    pub fn compose(&self, other: &PlrSequence) -> PlrSequence {
        PlrSequence {
            ops: self.ops.iter().chain(other.ops.iter()).copied().collect(),
        }
    }

    /// Apply this sequence of PLR operations to a chord.
    pub fn apply(&self, chord: Chord) -> Chord {
        self.ops.iter().fold(chord, |c, &op| apply_plr(c, op))
    }

    /// Compute the inverse sequence.
    ///
    /// For PLR group elements, each generator is its own inverse (P² = L² = R² = id),
    /// so the inverse of a sequence is the reverse sequence.
    pub fn inverse(&self) -> PlrSequence {
        PlrSequence {
            ops: self.ops.iter().rev().copied().collect(),
        }
    }

    /// Length of the operation sequence.
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Whether this is the identity (empty sequence).
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Alias for is_empty, since we have len.
    pub fn is_identity(&self) -> bool {
        self.is_empty()
    }
}

/// Voice-leading distance: the minimal total semitone movement between two triads.
///
/// Computes the minimal total pitch-class distance when mapping each note
/// of the source chord to a note of the target chord (optimal assignment).
pub fn voice_leading_distance(a: Chord, b: Chord) -> u8 {
    let pcs_a = a.pitch_classes();
    let pcs_b = b.pitch_classes();

    // Try all 6 permutations of assignment (3! = 6)
    let perms: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let mut min_dist = u8::MAX;
    for perm in &perms {
        let dist: u8 = (0..3)
            .map(|i| {
                let d = (pcs_a[i] as i16 - pcs_b[perm[i]] as i16).unsigned_abs() as u8;
                d.min(12 - d) // wrap-around distance
            })
            .sum();
        min_dist = min_dist.min(dist);
    }
    min_dist
}

/// Generate all 24 major and minor triads reachable via PLR operations from a starting chord.
pub fn all_triads_via_plr(start: Chord) -> Vec<Chord> {
    let generators = [PlrOp::P, PlrOp::L, PlrOp::R];
    let mut visited = [false; 24];
    let mut result = Vec::new();

    let chord_index = |c: Chord| -> usize {
        c.root as usize + if c.quality == Quality::Minor { 12 } else { 0 }
    };

    let mut queue = vec![start];
    visited[chord_index(start)] = true;

    while let Some(current) = queue.pop() {
        result.push(current);
        for &op in &generators {
            let next = apply_plr(current, op);
            let idx = chord_index(next);
            if !visited[idx] {
                visited[idx] = true;
                queue.push(next);
            }
        }
    }

    result
}

/// Navigate the neo-Riemannian torus.
///
/// The torus is the graph of triads connected by P, L, R edges.
/// This returns the shortest PLR path from source to target (BFS).
pub fn torus_path(source: Chord, target: Chord) -> Option<PlrSequence> {
    if source == target {
        return Some(PlrSequence::identity());
    }

    let generators = [PlrOp::P, PlrOp::L, PlrOp::R];
    let chord_index = |c: Chord| -> usize {
        c.root as usize + if c.quality == Quality::Minor { 12 } else { 0 }
    };

    let mut visited = [false; 24];
    let mut parent_op: Vec<Option<(usize, PlrOp)>> = vec![None; 24];
    let mut chords: Vec<Chord> = vec![Chord::major(0); 24]; // placeholder

    let start_idx = chord_index(source);
    let target_idx = chord_index(target);
    visited[start_idx] = true;
    chords[start_idx] = source;

    let mut queue = vec![start_idx];
    let mut found = false;

    while let Some(idx) = queue.first().copied() {
        queue.remove(0);
        let current = chords[idx];

        if idx == target_idx {
            found = true;
            break;
        }

        for &op in &generators {
            let next = apply_plr(current, op);
            let next_idx = chord_index(next);
            if !visited[next_idx] {
                visited[next_idx] = true;
                chords[next_idx] = next;
                parent_op[next_idx] = Some((idx, op));
                queue.push(next_idx);
            }
        }
    }

    if !found {
        return None;
    }

    // Reconstruct path
    let mut ops = Vec::new();
    let mut idx = target_idx;
    while let Some((parent, op)) = parent_op[idx] {
        ops.push(op);
        idx = parent;
    }
    ops.reverse();
    Some(PlrSequence::from_ops(&ops))
}

/// Check if applying PLR operations can reach all 24 triads from any starting chord.
pub fn plr_group_order() -> usize {
    all_triads_via_plr(Chord::major(0)).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_parallel_major_to_minor() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(c_major, PlrOp::P);
        assert_eq!(result, Chord::minor(note::C));
    }

    #[test]
    fn test_p_parallel_minor_to_major() {
        let c_minor = Chord::minor(note::C);
        let result = apply_plr(c_minor, PlrOp::P);
        assert_eq!(result, Chord::major(note::C));
    }

    #[test]
    fn test_l_leittonwechsel_major() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(c_major, PlrOp::L);
        assert_eq!(result, Chord::minor(note::E));
    }

    #[test]
    fn test_l_leittonwechsel_minor() {
        let e_minor = Chord::minor(note::E);
        let result = apply_plr(e_minor, PlrOp::L);
        assert_eq!(result, Chord::major(note::C));
    }

    #[test]
    fn test_r_relative_major() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(c_major, PlrOp::R);
        assert_eq!(result, Chord::minor(note::A));
    }

    #[test]
    fn test_r_relative_minor() {
        let a_minor = Chord::minor(note::A);
        let result = apply_plr(a_minor, PlrOp::R);
        assert_eq!(result, Chord::major(note::C));
    }

    #[test]
    fn test_p_squared_is_identity() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(apply_plr(c_major, PlrOp::P), PlrOp::P);
        assert_eq!(result, c_major);
    }

    #[test]
    fn test_l_squared_is_identity() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(apply_plr(c_major, PlrOp::L), PlrOp::L);
        assert_eq!(result, c_major);
    }

    #[test]
    fn test_r_squared_is_identity() {
        let c_major = Chord::major(note::C);
        let result = apply_plr(apply_plr(c_major, PlrOp::R), PlrOp::R);
        assert_eq!(result, c_major);
    }

    #[test]
    fn test_plr_generates_all_24_triads() {
        let triads = all_triads_via_plr(Chord::major(note::C));
        assert_eq!(triads.len(), 24);
    }

    #[test]
    fn test_plr_group_order_is_24() {
        assert_eq!(plr_group_order(), 24);
    }

    #[test]
    fn test_chord_pitch_classes_major() {
        let c_major = Chord::major(note::C);
        assert_eq!(c_major.pitch_classes(), [0, 4, 7]);
    }

    #[test]
    fn test_chord_pitch_classes_minor() {
        let a_minor = Chord::minor(note::A);
        assert_eq!(a_minor.pitch_classes(), [9, 0, 4]);
    }

    #[test]
    fn test_common_tones_p() {
        let c_major = Chord::major(note::C);
        let c_minor = Chord::minor(note::C);
        assert_eq!(c_major.common_tones(&c_minor), 2); // root and fifth
    }

    #[test]
    fn test_common_tones_l() {
        let c_major = Chord::major(note::C);
        let e_minor = Chord::minor(note::E);
        assert_eq!(c_major.common_tones(&e_minor), 2); // E and G
    }

    #[test]
    fn test_common_tones_r() {
        let c_major = Chord::major(note::C);
        let a_minor = Chord::minor(note::A);
        assert_eq!(c_major.common_tones(&a_minor), 2); // C and E
    }

    #[test]
    fn test_voice_leading_identity() {
        let c = Chord::major(note::C);
        assert_eq!(voice_leading_distance(c, c), 0);
    }

    #[test]
    fn test_voice_leading_p() {
        let c_major = Chord::major(note::C);
        let c_minor = Chord::minor(note::C);
        assert_eq!(voice_leading_distance(c_major, c_minor), 1);
    }

    #[test]
    fn test_voice_leading_l() {
        let c_major = Chord::major(note::C);
        let e_minor = Chord::minor(note::E);
        assert_eq!(voice_leading_distance(c_major, e_minor), 1);
    }

    #[test]
    fn test_voice_leading_r() {
        // R moves one note by whole tone, not semitone
        let c_major = Chord::major(note::C);
        let a_minor = Chord::minor(note::A);
        assert_eq!(voice_leading_distance(c_major, a_minor), 2);
    }

    #[test]
    fn test_sequence_identity() {
        let seq = PlrSequence::identity();
        assert!(seq.is_identity());
        let c = Chord::major(note::C);
        assert_eq!(seq.apply(c), c);
    }

    #[test]
    fn test_sequence_compose() {
        let p = PlrSequence::from_ops(&[PlrOp::P]);
        let l = PlrSequence::from_ops(&[PlrOp::L]);
        let pl = p.compose(&l);
        let c_major = Chord::major(note::C);
        // P then L on C major: P -> C minor, L -> Ab major
        let result = pl.apply(c_major);
        assert_eq!(result, apply_plr(apply_plr(c_major, PlrOp::P), PlrOp::L));
    }

    #[test]
    fn test_sequence_inverse() {
        let seq = PlrSequence::from_ops(&[PlrOp::P, PlrOp::L, PlrOp::R]);
        let inv = seq.inverse();
        assert_eq!(inv.ops, vec![PlrOp::R, PlrOp::L, PlrOp::P]);
    }

    #[test]
    fn test_torus_path_same_chord() {
        let c = Chord::major(note::C);
        let path = torus_path(c, c).unwrap();
        assert!(path.is_identity());
    }

    #[test]
    fn test_torus_path_p() {
        let c_major = Chord::major(note::C);
        let c_minor = Chord::minor(note::C);
        let path = torus_path(c_major, c_minor).unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path.ops[0], PlrOp::P);
    }

    #[test]
    fn test_chord_display() {
        assert_eq!(format!("{}", Chord::major(note::C)), "C major");
        assert_eq!(format!("{}", Chord::minor(note::A)), "A minor");
    }

    #[test]
    fn test_chord_contains() {
        let c_major = Chord::major(note::C);
        assert!(c_major.contains(note::C));
        assert!(c_major.contains(note::E));
        assert!(c_major.contains(note::G));
        assert!(!c_major.contains(note::CS));
    }
}

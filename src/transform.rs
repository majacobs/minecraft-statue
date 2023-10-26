use crate::drawing::Brush;

#[derive(Clone)]
pub struct Transform {
    steps: Vec<TransformStep>,
    pub scaling: i32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            scaling: 1,
        }
    }

    pub fn with_scale(scaling: u32) -> Self {
        Self {
            steps: vec![],
            scaling: scaling as i32,
        }
    }

    pub fn rotate(mut self, rotation: Rotation) -> Self {
        self.steps.push(TransformStep::Rotate(rotation));
        self
    }

    pub fn translate(mut self, dx: i32, dy: i32, dz: i32) -> Self {
        self.steps.push(TransformStep::Translate { dx, dy, dz });
        self
    }

    pub fn mirror(mut self, plane: Plane) -> Self {
        self.steps.push(TransformStep::Mirror(plane));
        self
    }

    pub fn then(mut self, other: &Transform) -> Self {
        self.steps.extend(other.steps.iter().cloned());
        self
    }

    pub fn apply(&self, mut x: i32, mut y: i32, mut z: i32) -> (i32, i32, i32) {
        x *= self.scaling;
        y *= self.scaling;
        z *= self.scaling;

        for step in self.steps.iter() {
            (x, y, z) = step.apply(x, y, z, self.scaling);
        }

        (x, y, z)
    }

    pub fn rotate_only(&self, mut x: i32, mut y: i32, mut z: i32) -> (i32, i32, i32) {
        for step in self.steps.iter() {
            if matches!(step, TransformStep::Rotate(_)) {
                (x, y, z) = step.apply(x, y, z, 0);
            }
        }

        (x, y, z)
    }

    pub fn brush(&self, mut brush: Brush) -> Brush {
        for step in self.steps.iter() {
            brush = step.brush(brush);
        }

        brush
    }
}

#[derive(Copy, Clone)]
enum TransformStep {
    Rotate(Rotation),
    Translate { dx: i32, dy: i32, dz: i32 },
    Mirror(Plane),
}

impl TransformStep {
    fn apply(&self, x: i32, y: i32, z: i32, scaling: i32) -> (i32, i32, i32) {
        match self {
            TransformStep::Rotate(r) => match r {
                Rotation::XPos => (x, -z, y),
                Rotation::XNeg => (x, z, -y),
                Rotation::YPos => (z, y, -x),
                Rotation::YNeg => (-z, y, x),
                Rotation::ZPos => (-y, x, z),
                Rotation::ZNeg => (y, -x, z),
            },
            TransformStep::Translate { dx, dy, dz } => {
                (x + dx * scaling, y + dy * scaling, z + dz * scaling)
            }
            TransformStep::Mirror(p) => match p {
                Plane::XY => (x, y, -z),
                Plane::XZ => (x, -y, z),
                Plane::YZ => (-x, y, z),
            },
        }
    }

    fn brush(&self, brush: Brush) -> Brush {
        match self {
            TransformStep::Rotate(r) => match (r, brush) {
                // Positive X: +Y => +Z => -Y => -Z
                (Rotation::XPos, Brush::YPos) => Brush::ZPos,
                (Rotation::XPos, Brush::YNeg) => Brush::ZNeg,
                (Rotation::XPos, Brush::ZPos) => Brush::YNeg,
                (Rotation::XPos, Brush::ZNeg) => Brush::YPos,
                // Negative X: +Y => -Z => -Y => +Z
                (Rotation::XNeg, Brush::YPos) => Brush::ZNeg,
                (Rotation::XNeg, Brush::YNeg) => Brush::ZPos,
                (Rotation::XNeg, Brush::ZPos) => Brush::YPos,
                (Rotation::XNeg, Brush::ZNeg) => Brush::YNeg,
                // Positive Y: +X => -Z => -X => +Z
                (Rotation::YPos, Brush::XPos) => Brush::ZNeg,
                (Rotation::YPos, Brush::XNeg) => Brush::ZPos,
                (Rotation::YPos, Brush::ZPos) => Brush::XPos,
                (Rotation::YPos, Brush::ZNeg) => Brush::XNeg,
                // Negative Y: +X => +Z => -X => -Z
                (Rotation::YNeg, Brush::XPos) => Brush::ZPos,
                (Rotation::YNeg, Brush::XNeg) => Brush::ZNeg,
                (Rotation::YNeg, Brush::ZPos) => Brush::XNeg,
                (Rotation::YNeg, Brush::ZNeg) => Brush::XPos,
                // Positive Z: +X => +Y => -X => -Y
                (Rotation::ZPos, Brush::XPos) => Brush::YPos,
                (Rotation::ZPos, Brush::XNeg) => Brush::YNeg,
                (Rotation::ZPos, Brush::YPos) => Brush::XNeg,
                (Rotation::ZPos, Brush::YNeg) => Brush::XPos,
                // Negative Z: +X => -Y => -X => +Y
                (Rotation::ZNeg, Brush::XPos) => Brush::YNeg,
                (Rotation::ZNeg, Brush::XNeg) => Brush::YPos,
                (Rotation::ZNeg, Brush::YPos) => Brush::XPos,
                (Rotation::ZNeg, Brush::YNeg) => Brush::XNeg,

                (_, b) => b,
            },
            TransformStep::Translate { .. } => brush,
            TransformStep::Mirror(p) => match (p, brush) {
                (Plane::XY, Brush::ZPos) => Brush::ZNeg,
                (Plane::XY, Brush::ZNeg) => Brush::ZPos,
                (Plane::XZ, Brush::YPos) => Brush::YNeg,
                (Plane::XZ, Brush::YNeg) => Brush::YPos,
                (Plane::YZ, Brush::XPos) => Brush::XNeg,
                (Plane::YZ, Brush::XNeg) => Brush::XPos,
                (_, b) => b,
            },
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Rotation {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Plane {
    XZ,
    XY,
    YZ,
}

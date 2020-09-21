use stretch::{
    geometry::Rect,
    style::Dimension,
};

#[derive(Debug, Copy, Clone)]
pub struct Thickness {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32
}

impl From<i32> for Thickness {
    fn from(i: i32) -> Self {
        Thickness {
            left: i,
            top: i,
            right: i,
            bottom: i
        }
    }
}

impl From<(i32, i32)> for Thickness {
    fn from(i: (i32, i32)) -> Self {
        Thickness {
            left: i.0,
            top: i.1,
            right: i.0,
            bottom: i.1
        }
    }
}

impl From<(i32, i32, i32, i32)> for Thickness {
    fn from(i: (i32, i32, i32, i32)) -> Self {
        Thickness {
            left: i.0,
            top: i.1,
            right: i.2,
            bottom: i.3
        }
    }
}

impl Into<Rect<Dimension>> for Thickness {
    fn into(self) -> Rect<Dimension> {
        Rect {
            start: Dimension::Points(self.left as f32),
            top: Dimension::Points(self.top as f32),
            end: Dimension::Points(self.right as f32),
            bottom: Dimension::Points(self.bottom as f32),
        }
    }
}
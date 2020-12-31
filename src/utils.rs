#[derive(Debug, PartialEq)]
pub(crate) struct Rect {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    pub fn intersect(&self, other: &Rect) -> Option<Rect> {
        let inter_left = self.left.max(other.left);
        let inter_right = (self.left + self.width).min(other.left + other.width);
        let inter_top = self.top.max(other.top);
        let inter_bottom = (self.top + self.height).min(other.top + other.height);

        if inter_left < inter_right && inter_top < inter_bottom {
            return Some(Rect::new(
                inter_left,
                inter_top,
                inter_right - inter_left,
                inter_bottom - inter_top,
            ));
        }
        None
    }

    pub fn scale_x(mut self, scale: f32) -> Self {
        self.left *= scale;
        self.width *= scale;

        self
    }

    pub fn scale_y(mut self, scale: f32) -> Self {
        self.top *= scale;
        self.height *= scale;

        self
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.left + self.width
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn bottom(&self) -> f32 {
        self.top + self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r1 = Rect::new(0.0, 0.0, 1600.0, 1200.0);
        assert_eq!(
            Rect::new(0.0, 0.0, 856.0, 161.0),
            r1.intersect(&Rect::new(-168.0, -863.0, 1024.0, 1024.0))
                .unwrap()
        );
        assert_eq!(
            Rect::new(0.0, 161.0, 856.0, 1024.0),
            r1.intersect(&Rect::new(-168.0, 161.0, 1024.0, 1024.0))
                .unwrap()
        );
        assert_eq!(
            Rect::new(0.0, 1185.0, 856.0, 15.0),
            r1.intersect(&Rect::new(-168.0, 1185.0, 1024.0, 1024.0))
                .unwrap()
        );
        assert_eq!(
            Rect::new(856.0, 0.0, 744.0, 161.0),
            r1.intersect(&Rect::new(856.0, -863.0, 1024.0, 1024.0))
                .unwrap()
        );
        assert_eq!(
            Rect::new(856.0, 161.0, 744.0, 1024.0),
            r1.intersect(&Rect::new(856.0, 161.0, 1024.0, 1024.0))
                .unwrap()
        );
        assert_eq!(
            Rect::new(856.0, 1185.0, 744.0, 15.0),
            r1.intersect(&Rect::new(856.0, 1185.0, 1024.0, 1024.0))
                .unwrap()
        );
    }
}

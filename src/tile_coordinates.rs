use crate::utils::Rect;

fn to_cartesian_x(x: f32) -> f32 {
    (x - 1.0 / 2.0) * 2.0
}

fn to_cartesian_y(y: f32) -> f32 {
    (1.0 / 2.0 - y) * 2.0
}

#[derive(Debug, Clone)]
pub(crate) struct TileCoordinates {
    pub shader_coords: (f32, f32, f32, f32),
    pub texture_coords: (f32, f32, f32, f32),
}

impl TileCoordinates {
    pub fn new(left: f32, top: f32, width: f32, height: f32, tile_size: f32) -> Self {
        let tile_rect = Rect::new(left, top, tile_size, tile_size)
            .intersect(&Rect::new(0.0, 0.0, width, height))
            .unwrap()
            .scale_x(1.0 / width)
            .scale_y(1.0 / height);

        let shader_coords = (
            to_cartesian_x(tile_rect.left()),
            to_cartesian_y(tile_rect.top()),
            to_cartesian_x(tile_rect.right()),
            to_cartesian_y(tile_rect.bottom()),
        );

        let intersect_with_screen = Rect::new(left, top, tile_size, tile_size)
            .intersect(&Rect::new(0.0, 0.0, width, height))
            .unwrap();

        let texture = Rect::new(
            intersect_with_screen.left() - left,
            intersect_with_screen.top() - top,
            intersect_with_screen.width(),
            intersect_with_screen.height(),
        )
        .scale_x(1.0 / tile_size)
        .scale_y(1.0 / tile_size);

        let texture_coords = (
            texture.left(),
            texture.top(),
            texture.right(),
            texture.bottom(),
        );
        Self {
            shader_coords,
            texture_coords,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tc = TileCoordinates::new(-863.0, -168.0, 1600.0, 1200.0, 1024.0);
        assert_eq!(tc.shader_coords, (-1.0, 1.0, -0.79875, -0.42666674));
        assert_eq!(tc.texture_coords, (0.84277344, 0.1640625, 1.0, 1.0));

        let tc = TileCoordinates::new(161.0, 856.0, 1600.0, 1200.0, 1024.0);
        assert_eq!(tc.shader_coords, (-0.79875, -0.42666674, 0.48124993, -1.0));
        assert_eq!(tc.texture_coords, (0.0, 0.0, 1.0, 0.3359375));
    }
}

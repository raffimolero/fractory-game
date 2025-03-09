use super::*;

impl FractalViewElement {
    // TODO: refactor to use math instead of transforms
    // pos_to_grid(Vec2) -> TilePos or something
    pub(crate) fn subtree_click_pos(&mut self, click: Vec2, depth: usize) -> Option<TilePos> {
        if depth > self.frac_cam.hover_depth() {
            return None;
        }
        if !in_triangle(click) {
            return None;
        }

        let w = 1.0;
        let side = 2.0;
        let out_r = 3_f32.sqrt() / 3.0 * side;
        let in_r = out_r / 2.0;

        let transforms = [
            flip_xy(),
            shift(0.0, -out_r),
            shift(w, in_r),
            shift(-w, in_r),
        ]
        .map(|t| downscale(2.0) * t)
        .map(|t| t.inverse());

        for (transform, subtile) in transforms.into_iter().zip(SubTile::QUAD.0) {
            let hit_pos = self.subtree_click_pos(
                transform.transform_point3(click.extend(0.0)).truncate(),
                depth + 1,
            );
            if let Some(mut tile_pos) = hit_pos {
                tile_pos.push_outer(subtile);
                return Some(tile_pos);
            }
        }
        Some(TilePos::UNIT)
    }

    pub fn tree_click_pos(&mut self, ctx: &mut Context, pos: Vec2) -> Option<TilePos> {
        let pos = self
            .frac_cam
            .camera
            .inverse()
            .transform_point3(pos.extend(0.0))
            .truncate();

        if !in_triangle(pos) {
            return None;
        }

        self.subtree_click_pos(pos, 0)
    }
}

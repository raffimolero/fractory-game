use super::*;

impl FractalViewElement {
    pub(crate) fn input_edit(
        &mut self,
        button: MouseButton,
        hit_pos: TilePos,
        fractal: &mut Fractal,
        biome: &Biome,
    ) {
        let increment = match button {
            MouseButton::Left => 1,
            MouseButton::Right => biome.leaf_count() - 1,
            _ => {
                debug_assert!(false, "unreachable");
                return;
            }
        };

        let mut tile = fractal.get(hit_pos);
        tile.id += increment;
        tile.id %= biome.leaf_count();
        tile.orient = fractal.library[tile.id].symmetries.into();
        fractal.set(hit_pos, tile);
    }

    pub(crate) fn input_flip(&mut self, hit_pos: TilePos, fractal: &mut Fractal) {
        let tile = fractal.get(hit_pos);
        fractal.set(hit_pos, tile + Transform::FU);
    }

    pub(crate) fn input_act(&mut self, hit_pos: TilePos, activated: &mut ActiveTiles) {
        activated.toggle(hit_pos);
    }

    pub(crate) fn input_rot(
        &mut self,
        button: MouseButton,
        hit_pos: TilePos,
        fractal: &mut Fractal,
    ) {
        let tf = match button {
            MouseButton::Right => Transform::KR,
            MouseButton::Left => Transform::KL,
            _ => {
                debug_assert!(false, "unreachable");
                return;
            }
        };

        let mut tile = fractal.get(hit_pos);
        tile += tf;
        fractal.set(hit_pos, tile);
    }

    pub(crate) fn input_grab(
        &mut self,
        hit_pos: TilePos,
        fractal: &mut Fractal,
        cursor: &mut CursorState,
    ) {
        let CursorState::Free = cursor else { return };
        let tile = fractal.set(hit_pos, Tile::SPACE);
        if tile == Tile::SPACE {
            return;
        }
        *cursor = CursorState::Holding(tile);
    }

    pub(crate) fn input_drop(
        &mut self,
        hit_pos: TilePos,
        fractal: &mut Fractal,
        cursor: &mut CursorState,
    ) {
        let CursorState::Holding(tile) = *cursor else {
            return;
        };
        if fractal.get(hit_pos) != Tile::SPACE {
            return;
        }
        fractal.set(hit_pos, tile);
        *cursor = CursorState::Free;
    }

    /// returns true if mouse input was captured
    pub fn input(
        &mut self,
        ctx: &mut Context,
        res: &mut Resources,
        mouse_focus: bool,
        cursor: &mut CursorState,
        fractory: &mut Fractory,
        cache: &FractoryCache,
    ) -> bool {
        self.frac_cam.input(ctx, res, mouse_focus);

        // if is_key_pressed(KeyCode::Apostrophe) {
        //     dbg!(&fractory.fractal.library);
        // }

        let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        'click: {
            if !mouse_focus {
                break 'click false;
            }

            use MouseButton::{Left as Lmb, Right as Rmb};

            let click = if is_mouse_button_pressed(Lmb) {
                ctx.get_lmb().map(|click| (click, true, Lmb))
            } else if is_mouse_button_released(Lmb) {
                ctx.get_lmb().map(|click| (click, false, Lmb))
            } else if is_mouse_button_pressed(Rmb) {
                ctx.get_rmb().map(|click| (click, true, Rmb))
            } else if is_mouse_button_released(Rmb) {
                ctx.get_rmb().map(|click| (click, false, Rmb))
            } else {
                None
            };

            let Some((click, down, button)) = click else {
                break 'click false;
            };

            let Some(hit_pos) = self.tree_click_pos(ctx, click.pos) else {
                break 'click false;
            };

            match (down, cursor) {
                (false, CursorState::Free) => match (ctrl, shift, button) {
                    (true, true, Lmb) => self.input_rot(button, hit_pos, &mut fractory.fractal),
                    (true, true, Rmb) => self.input_flip(hit_pos, &mut fractory.fractal),
                    (false, true, Lmb) => self.input_act(hit_pos, &mut fractory.activated),
                    _ => {}
                },
                // (false, CursorState::Free) => match (ctrl, shift, button) {
                //     (true, true, _) => {
                //         self.input_edit(button, hit_pos, &mut fractory.fractal, &cache.biome)
                //     }
                //     (true, false, Lmb) => self.input_act(hit_pos, &mut fractory.activated),
                //     (true, false, Rmb) => self.input_flip(hit_pos, &mut fractory.fractal),
                //     (false, true, _) => self.input_rot(button, hit_pos, &mut fractory.fractal),
                //     _ => {}
                // },
                (true, cursor @ CursorState::Free) => match (ctrl, shift, button) {
                    (false, false, Lmb) => self.input_grab(hit_pos, &mut fractory.fractal, cursor),
                    _ => {}
                },
                (true, cursor @ CursorState::Holding { .. }) => match (ctrl, shift, button) {
                    (false, false, Lmb) => self.input_drop(hit_pos, &mut fractory.fractal, cursor),
                    _ => {}
                },
                _ => {}
            }
            true
        }
    }
}

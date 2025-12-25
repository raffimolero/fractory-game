use super::*;

impl FractalViewElement {
    /// returns whether the node continued further down
    ///
    /// ControlFlow::Continue(()) if this is a branch triangle
    ///
    /// ControlFlow::Break(()) if this is a leaf triangle
    pub(crate) fn draw_node(
        &self,
        ctx: &mut Context,
        text_tool: TextToolId,
        fractory: &Fractory,
        cache: &FractoryCache,
        id: usize,
        tile_fill: TileFill,
        pos: Result<TilePos, usize>,
        hovered: bool,
    ) -> ControlFlow<()> {
        let depth = match pos {
            Ok(p) => p.depth(),
            Err(d) => d,
        };

        let should_break = if hovered {
            depth == self.frac_cam.hover_depth()
        } else {
            depth >= self.frac_cam.max_depth()
                || tile_fill.is_leaf() && depth >= self.frac_cam.min_depth()
        };

        let control_flow = if should_break {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        };

        if !DRAW_BRANCHES && control_flow == ControlFlow::Continue(()) {
            return control_flow;
        }

        // FUTURE: add a cursor follower that visually shows the expansion threshold by size
        // maybe solve this once you do bevy tbh
        // shift+scroll zooms the mouse cursor, scroll zooms the camera *and* the cursor

        let is_active = pos.is_ok_and(|p| fractory.activated.contains(p));
        let style = if hovered || is_active {
            let border_color = if !is_active {
                GRAY
            } else if cache
                .biome
                .behavior(cache.fragments.behaviors(), id)
                .is_empty()
            {
                RED
            } else {
                WHITE
            };
            TileStyle::Bordered {
                border_color,
                orient_icon: hovered.then(|| tile_symmetries(fractory, id)),
            }
        } else {
            TileStyle::Plain
        };
        ctx.apply(upscale(self.view_state.scaling()), |ctx| {
            // ctx.apply(shift(0.0, -0.2) * downscale(4.0), |_| {
            //     let text = format!("{pos:#?}");
            //     text_tool(&text);
            // });
            draw_tile(
                ctx,
                text_tool,
                tile_color(fractory, id),
                tile_name(cache.fragments.names(), id),
                style,
            );
        });

        control_flow
    }

    pub(crate) fn draw_subtree(
        &self,
        ctx: &mut Context,
        text_tool: TextToolId,
        fractory: &Fractory,
        cache: &FractoryCache,
        cur_orient: Transform,
        tile: Tile,
        pos: Result<TilePos, usize>,
    ) {
        let mouse = ctx.mouse_pos().unwrap_or(Vec2::ZERO);
        let hovered = in_triangle(mouse);
        let SlotInfo {
            quad,
            fill,
            symmetries: _,
        } = fractory.fractal.library[tile.id];

        let transforms = triangle_transforms();
        let tile_matrix = transform_to_mat4(tile.orient.into());

        ctx.apply(tile_matrix, |ctx| {
            if !ctx.is_onscreen(&TRIANGLE) {
                return;
            }
            match self.draw_node(ctx, text_tool, fractory, cache, tile.id, fill, pos, hovered) {
                ControlFlow::Continue(()) => {}
                ControlFlow::Break(()) => return,
            }
            for ((transform, child), subtile) in
                transforms.into_iter().zip(quad.0).zip(SubTile::QUAD.0)
            {
                let orient = cur_orient - tile.orient.to_transform();

                let pos = match pos {
                    Ok(mut pos) => {
                        pos.push_inner(subtile - orient);
                        (pos.depth <= 30).then_some(pos).ok_or(pos.depth as usize)
                    }
                    Err(d) => Err(d + 1),
                };
                ctx.apply(transform, |ctx| {
                    self.draw_subtree(ctx, text_tool, fractory, cache, orient, child, pos);
                });
            }
        });
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        res: &mut Resources,
        text_tool: TextToolId,
        fractory_meta: &FractoryMeta,
        cache: &FractoryCache,
    ) {
        ctx.apply(self.frac_cam.camera, |ctx| {
            self.draw_subtree(
                ctx,
                text_tool,
                &fractory_meta.fractory,
                cache,
                Transform::KU,
                fractory_meta.fractory.fractal.root,
                Ok(TilePos::UNIT),
            );
        });
        ctx.apply(shift(0.0, -0.8) * downscale(10.0), |ctx| {
            let fractal_cam::FractalCam {
                camera,
                mouse_depth,
                min_bg_depth,
                ..
            } = self.frac_cam;

            // // rectangle around text for clarity
            // ctx.flush();
            // let expected_w = 8.7;
            // let expected_h = 2.2;
            // ctx.queue_polygon(
            //     &[
            //         Vec2::new(-expected_w / 2.0, -expected_h / 2.0),
            //         Vec2::new(expected_w / 2.0, -expected_h / 2.0),
            //         Vec2::new(expected_w / 2.0, expected_h / 2.0),
            //         Vec2::new(-expected_w / 2.0, expected_h / 2.0),
            //     ],
            //     DARKGRAY,
            // );
            ctx.queue_text(
                text_tool,
                format!(
                    "Selection Depth: 2^{:.2}\n\
                    Background Depth: 2^{:.2} * zoom\n\
                    Zoom: 2^{:.2}",
                    mouse_depth,
                    min_bg_depth,
                    self.frac_cam.scale().log2(),
                ),
            );
        });
        ctx.flush();
    }
}

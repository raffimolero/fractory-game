use crate::prelude::*;

pub struct FractoryElement {
    cursor: CursorState,
    fractory_meta: FractoryMeta,
    fractal_view: fractal_view_element::FractalViewElement,
    // inventory_view: InventoryViewElement,
    cache: FractoryCache,
}

impl FractoryElement {
    pub fn new(res: &mut Resources) -> Self {
        let planet_id = PlanetId::from(XYYY);
        let planet = Planet::new_xyyy();
        res.planets.register(planet_id.clone(), Planet::new_xyyy());

        let biome_id = BiomeId::from(XYYY_LANDING_ZONE);
        let biome = Biome::new_xyyy_landing_zone();
        res.biomes.register(
            planet_id.clone(),
            biome_id.clone(),
            Biome::new_xyyy_landing_zone(),
        );

        let mut fractory_meta = FractoryMeta::new(planet_id, biome_id, &planet, &biome);
        init_xyyy_fractory(&mut fractory_meta.fractory, Config::TestGrowFarm);

        let cache = FractoryCache {
            fragments: planet.fragments().to_owned(),
            biome,
        };

        Self {
            cursor: CursorState::Free,
            fractory_meta,
            fractal_view: fractal_view_element::FractalViewElement::new(),
            cache,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, res: &mut Resources, text_tool: TextToolId) {
        self.draw_inventory(ctx, text_tool);
        self.fractal_view
            .draw(ctx, res, text_tool, &self.fractory_meta, &self.cache);

        ctx.apply(shift(0.0, 0.6) * downscale(10.0), |ctx| {
        ctx.queue_text(
            text_tool,
            "Esc: quit\n\
                Tab: toggle shattered view\n\
                Enter: tick\n\
                Camera:\n\
                -> WASD: move | Q/E: rotate | F: flip | (Shift+)Space: zoom (out)in\n\
                -> Click+Drag: move | Scroll: zoom | (Ctrl/Alt)+Scroll: change cursor/background depth\n\
                Shift+LMB/RMB: Rotate tile (no effect on rotational tiles such as X, Y, Rotor)\n\
                Ctrl+LMB: Activate tile | Ctrl+RMB: Flip tile (no effect on reflective tiles)\n\
                Ctrl+Shift+LMB/RMB: Cycle tile\n\
                *Some edits may change other tiles' rotations. This is normal."
                .into(),
        );
    });
        self.draw_cursor(ctx, text_tool);
    }

    pub fn draw_cursor(&mut self, ctx: &mut Context, text_tool: TextToolId) {
        match self.cursor {
            CursorState::Free => {}
            CursorState::Holding { tile_id, orient } => {
                let color = tile_color(&self.fractory_meta.fractory, tile_id);
                let name = tile_name(self.cache.fragments.names(), tile_id);
                // UNIMPLEMENTED: get hit position to draw
                let hit_pos = ctx
                    .mouse_pos()
                    .and_then(|pos| self.fractal_view.tree_click_pos(ctx, pos));

                let mut matrix = orient.to_transform(ctx, self.fractal_view.frac_cam.camera);
                // this ridiculous cols array conversion is due to there being
                // 2 versions of glam, which i am not going to fix at the moment
                if let Some(tile_pos) = hit_pos {
                    matrix = self.fractal_view.frac_cam.camera
                        * shift(0.0, -OUT_R)
                        * tile_pos_to_mat4(tile_pos)
                        * matrix;
                }

                ctx.apply(matrix, |ctx| {
                    draw_tile(
                        ctx,
                        text_tool,
                        color,
                        name,
                        TileStyle::Bordered {
                            border_color: BLUE,
                            with_orient_icon: true,
                        },
                    )
                });
            }
        }
        ctx.flush();
    }

    pub fn draw_inventory(&mut self, ctx: &mut Context, text_tool: TextToolId) {
        let wrap = (self.fractory_meta.fractory.inventory.len() as f32).sqrt() as usize;
        for (idx, (tile_id, count)) in self.fractory_meta.fractory.inventory.iter().enumerate() {
            let x = idx % wrap;
            let y = idx / wrap;
        }
        ctx.flush();
    }

    pub fn input(&mut self, ctx: &mut Context, res: &mut Resources) {
        self.fractal_view.input(
            ctx,
            res,
            &mut self.cursor,
            &mut self.fractory_meta.fractory,
            &self.cache,
        );
    }
}

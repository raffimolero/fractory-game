use super::*;

pub struct InventoryViewState {
    pub enabled: bool,
    pub panel_width: f32,
}

impl InventoryViewState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            panel_width: 1.0 / 3.0,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled ^= true;
    }
}

impl FractoryElement {
    fn inventory_rect(&self, ctx: &mut Context) -> Rect {
        let sw = screen_width();
        let sh = screen_height();
        let w = sw / sh;
        let x = self.inventory_view.panel_width * SCREEN_SPACE_WIDTH;
        Rect::new(w * (1.0 - x), -1.0, x * w, SCREEN_SPACE_HEIGHT)
    }

    pub fn draw_inventory(&mut self, ctx: &mut Context, text_tool: TextToolId) {
        // TODO: what if the inventory was just a pile of user-arranged tiles
        // like, imagine if it wasn't organized and the player had to drag them manually
        // there would of course be a way to align them to a grid
        if !self.inventory_view.enabled {
            return;
        }

        ctx.queue_text(text_tool, "TODO".into());

        'draw_rect: {
            let points = &rect_to_points(self.inventory_rect(ctx));
            ctx.queue_polygon(points, GRAY);
        }

        let wrap = (self.fractory_meta.fractory.inventory.len() as f32).sqrt() as usize;

        for (idx, (&tile_id, &count)) in self.fractory_meta.fractory.inventory.iter().enumerate() {
            let x = idx % wrap;
            let y = idx / wrap;
            let color = tile_color(&self.fractory_meta.fractory, tile_id);
            let name = tile_name(self.cache.fragments.names(), tile_id);
            let sym = tile_symmetries(&self.fractory_meta.fractory, tile_id);
            draw_tile(
                ctx,
                text_tool,
                color,
                name,
                TileStyle::Bordered {
                    border_color: WHITE,
                    orient_icon: Some(sym),
                },
            )
        }
        ctx.flush();
    }

    /// returns true if this captured a mouse input
    pub fn input_inventory(
        &mut self,
        ctx: &mut Context,
        res: &mut Resources,
        mouse_focus: bool,
    ) -> bool {
        if !self.inventory_view.enabled || !mouse_focus {
            return false;
        }
        let captured = ctx
            .mouse_pos()
            .map_or(false, |pos| self.inventory_rect(ctx).contains(pos));
        if captured {
            todo!()
        }
        captured
    }
}

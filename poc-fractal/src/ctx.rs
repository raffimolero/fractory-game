use std::time::{Instant, SystemTime, UNIX_EPOCH};

use super::*;
use ergoquad_2d::macroquad::models::Vertex;

pub type TextToolId = usize;

/// from macroqud gl's default settings
/// max vertices is 10,000 so i have no idea why it's bigger
/// when there are like more indices on average
const MAX_INDICES: usize = 5_000;

struct Batcher {
    filled_meshes: usize,
    meshes: Vec<Mesh>,
    text_tools: Vec<Box<dyn Fn(&str)>>,
    text: Vec<(TextToolId, String, Mat4)>,
    top_z: f32,
}

impl Batcher {
    fn add_polygon_to_mesh(
        mesh: &mut Mesh,
        top_z: &mut f32,
        points: impl ExactSizeIterator<Item = Vec2>,
        color: Color,
    ) {
        let len = points.len();
        let iter = points.map(|p| Vertex {
            position: p.extend(*top_z),
            uv: Vec2::ZERO,
            color,
        });
        mesh.vertices.extend(iter);

        let mut off = mesh.indices.len() as u16;
        for i in 2..len as u16 {
            mesh.indices.push(off);
            mesh.indices.push(off + i - 1);
            mesh.indices.push(off + i);
        }
        *top_z -= 1.0 / MAX_INDICES as f32;
    }

    fn next_mesh<'a>(
        filled: &mut usize,
        meshes: &'a mut Vec<Mesh>,
        top_z: &mut f32,
        len: usize,
    ) -> &'a mut Mesh {
        if meshes
            .get_mut(*filled)
            .is_some_and(|m| m.indices.len() + len * 3 < MAX_INDICES)
        {
            return &mut meshes[*filled];
        }
        *filled = filled.wrapping_add(1);
        *top_z = 0.0;
        if *filled < meshes.len() {
            return &mut meshes[*filled];
        }
        meshes.push(Mesh {
            vertices: vec![],
            indices: vec![],
            texture: None,
        });
        return &mut meshes[*filled];
    }

    fn queue_polygon(&mut self, points: impl ExactSizeIterator<Item = Vec2>, color: Color) {
        let mesh = Self::next_mesh(
            &mut self.filled_meshes,
            &mut self.meshes,
            &mut self.top_z,
            points.len(),
        );
        Self::add_polygon_to_mesh(mesh, &mut self.top_z, points, color)
    }

    fn register_text_tool(&mut self, text_tool: Box<dyn Fn(&str)>) -> TextToolId {
        self.text_tools.push(text_tool);
        self.text_tools.len() - 1
    }

    /// text will always be drawn on top of the mesh. flush in between if necessary.
    fn queue_text(&mut self, tool_id: TextToolId, text: String, matrix: Mat4) {
        self.text.push((tool_id, text, matrix))
    }

    fn flush(&mut self, base_matrix: Mat4) {
        use ergoquad_2d::bread_n_butter::apply;
        apply(base_matrix, || {
            for mesh in &mut self.meshes {
                draw_mesh(mesh);
                mesh.vertices.clear();
                mesh.indices.clear();
            }
            self.filled_meshes = usize::MAX;
            for (tool, text, matrix) in self.text.drain(..) {
                apply(matrix, || {
                    self.text_tools[tool](&text);
                });
            }
        });
        self.top_z = 0.0;
    }
}

impl std::fmt::Debug for Batcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Batcher {{ mesh: .., top_z: {} }}", self.top_z)
    }
}

impl Default for Batcher {
    fn default() -> Self {
        Self {
            filled_meshes: usize::MAX,
            meshes: vec![],
            text_tools: vec![],
            text: vec![],
            top_z: 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Context {
    mouse: Option<Vec2>,
    last_lmb: Option<(Vec2, Instant)>,
    last_rmb: Option<(Vec2, Instant)>,
    matrix: Mat4,
    inv_matrix: Mat4,
    batcher: Batcher,
}

impl Context {
    pub fn apply<T>(&mut self, matrix: Mat4, f: impl FnOnce(&mut Self) -> T) -> T {
        let orig = (self.matrix, self.inv_matrix);
        self.matrix = self.matrix * matrix;
        self.inv_matrix = self.matrix.inverse();
        // HACK: inlined ergoquad apply
        let out = {
            let gl = unsafe { get_internal_gl().quad_gl };
            gl.push_model_matrix(matrix);
            let out = (|| f(self))();
            gl.pop_model_matrix();
            out
        };
        (self.matrix, self.inv_matrix) = orig;
        out
    }

    pub fn is_onscreen(&self, points: &[Vec2]) -> bool {
        points
            .iter()
            .map(|p| self.unproject(*p))
            .map(|p| Rect::new(p.x, p.y, 0.0, 0.0))
            .reduce(|a, b| a.combine_with(b))
            .unwrap()
            .overlaps(&Rect::new(-1.0, -1.0, 2.0, 2.0))
    }

    pub fn draw_canvas(&mut self, matrix: Mat4, paint: impl FnOnce(&mut Self)) {
        unimplemented!("reset model matrix, make canvas with computed size");
    }

    pub fn queue_polygon(&mut self, points: &[Vec2], color: Color) {
        // self.project(p) inlined to avoid double borrow
        let project = |p: &Vec2| self.matrix.transform_point3(p.extend(0.0)).truncate();
        let points = points.iter().map(project);
        self.batcher.queue_polygon(points, color)
    }

    pub fn register_text_tool(&mut self, text_tool: Box<dyn Fn(&str)>) -> TextToolId {
        self.batcher.register_text_tool(text_tool)
    }

    pub fn queue_text(&mut self, tool_id: TextToolId, text: String) {
        self.batcher.queue_text(tool_id, text, self.matrix)
    }

    pub fn flush(&mut self) {
        self.batcher.flush(self.inv_matrix)
    }

    pub fn matrix(&self) -> Mat4 {
        self.matrix
    }

    pub fn inv_matrix(&self) -> Mat4 {
        self.inv_matrix
    }

    pub fn project(&self, point: Vec2) -> Vec2 {
        self.inv_matrix
            .transform_point3(point.extend(0.0))
            .truncate()
    }

    fn unproject(&self, point: Vec2) -> Vec2 {
        self.matrix.transform_point3(point.extend(0.0)).truncate()
    }

    pub fn mouse_pos(&self) -> Option<Vec2> {
        self.mouse.map(|pos| self.project(pos))
    }

    pub fn lmb_pos(&self) -> Option<(Vec2, Instant)> {
        self.last_lmb.map(|(pos, time)| (self.project(pos), time))
    }

    pub fn rmb_pos(&self) -> Option<(Vec2, Instant)> {
        self.last_rmb.map(|(pos, time)| (self.project(pos), time))
    }

    pub fn update(&mut self) {
        self.mouse.replace(mouse_position_local());
        let now = Instant::now();
        if is_mouse_button_pressed(MouseButton::Left) {
            self.last_lmb = self.mouse.map(|pos| (pos, now));
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            self.last_rmb = self.mouse.map(|pos| (pos, now));
        }
    }
}
pub struct Click {
    pub pos: Vec2,
    pub held: bool,
}
impl Context {
    fn get_click(&self, button: MouseButton) -> Option<Click> {
        const SCREEN_WIDTH: f32 = 2.0;
        const LEASH_RANGE: f32 = SCREEN_WIDTH / 4.0;
        const HOLD_DURATION: Duration = Duration::from_secs(1);

        let (down, lmb_time) = match button {
            MouseButton::Right => self.rmb_pos(),
            MouseButton::Left => self.lmb_pos(),
            _ => return None,
        }?;
        let up = self.mouse_pos()?;

        let leash_sq = LEASH_RANGE * LEASH_RANGE;
        let in_range = (down - up).length_squared() < leash_sq;

        let hold_time = Instant::now() - lmb_time;
        let held = hold_time >= HOLD_DURATION;

        in_range.then(|| Click { pos: down, held })
    }

    pub fn get_lmb(&self) -> Option<Click> {
        self.get_click(MouseButton::Left)
    }

    pub fn get_rmb(&self) -> Option<Click> {
        self.get_click(MouseButton::Right)
    }
}

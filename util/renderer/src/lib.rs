
pub trait RenderTarget {
    type Vertex: RenderVertex;
    fn draw(&self, vertices: impl IntoIterator<Item = Self::Vertex>, indices: impl IntoIterator<Item = u32>);
}

pub trait RenderVertex: Sized {
    const ASSERT: () = {
        assert!(
            std::mem::size_of::<Self>() % std::mem::size_of::<f32>() == 0,
            "Vertex must be sequence of floating points"
        );
    };

    const VERTEX_LAYOUT: &'static [usize];

    fn to_raw_data(&self) -> [f32; std::mem::size_of::<Self>() / 4];
}

impl RenderVertex for u8 {
    const VERTEX_LAYOUT: &'static [usize] = &[];
}

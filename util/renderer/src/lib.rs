#![feature(generic_const_exprs)]

pub trait RenderTarget {
    type Vertex: RenderVertex;
    fn draw(
        &self,
        vertices: impl IntoIterator<Item = Self::Vertex>,
        indices: impl IntoIterator<Item = u32>,
    );
}

pub trait RenderVertex: Sized + Copy {
    const F32_WIDTH: usize = {
        assert!(
            std::mem::size_of::<Self>() % std::mem::size_of::<f32>() == 0,
            "Vertex must be sequence of floating points"
        );
        std::mem::size_of::<Self>() / 4
    };

    const VERTEX_LAYOUT: &'static [usize];

    fn to_raw_data(&self) -> [f32; Self::F32_WIDTH] {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl RenderVertex for u8 {
    const VERTEX_LAYOUT: &'static [usize] = &[];
}

// #[derive(Clone, Copy)]
// struct ColoredBox {
//     tl_corner: [f32; 2],
//     color: [f32; 4],
//     br_corner: [f32; 2],
// }

// impl RenderVertex for ColoredBox {
//     const VERTEX_LAYOUT: &'static [usize] = &[2, 4, 2];
// }

// fn do_thing<T: RenderVertex>(thing: T)
// where
//     [(); T::F32_WIDTH]:, // ???
// {
//     thing.to_raw_data();
// }

// #[test]
// fn test() {
//     do_thing([0_f32; 5]);
//     // do_thing(5_u8);
// }

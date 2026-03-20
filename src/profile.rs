use bevy::prelude::*;
pub trait ElementProfile {
    /// Cut of profile
    fn cut(&self,  base: &Vec3, tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3>;
    /// construction of a profile element from a near and far cut
    fn build(&self, near: &Vec<Vec3>, far: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>, uvs: &mut Vec<[f32;2]>);
    fn normal(tangent: &Vec3, bnormal: &Vec3)-> Vec3 {
          -tangent.normalize().cross(bnormal.normalize()).normalize()
    }
}
/// Flat track profile

pub struct EpFlat{pub half_width: f32}
impl ElementProfile for EpFlat {
    fn cut(&self, base: &Vec3, _tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3> {
        vec![
            base - bnormal * self.half_width, base  +  bnormal * self.half_width,
        ]
    }

    // ---

    fn build(&self, near: &Vec<Vec3>, far: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>, uvs: &mut Vec<[f32;2]> ) {
        let j =  verts.len() as u32;
        verts.extend(vec![
            near[0], near[1], far[0], far[1],
            ].iter().map(Vec3::to_array)
        );
        idxs.extend_from_slice(&[
            j, j + 1, j + 2,
            j + 2, j + 1, j + 3,
        ]);

        uvs.extend_from_slice(&[
            [1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0., 0.],
        ]);
    }
}

// -----------------------------------
/// Box-like track profile
pub struct EpBox {pub half_width: f32, pub half_height: f32}
impl ElementProfile for EpBox {
    fn cut(&self,  base: &Vec3, tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3> {
        let normal  = Self::normal(tangent, bnormal);
        vec![
            -normal * self.half_height - bnormal * self.half_width,
            -normal * self.half_height + bnormal * self.half_width,
            normal * self.half_height + bnormal * self.half_width,
            normal * self.half_height - bnormal * self.half_width,
        ]. iter().map(| p |  base + p ).collect::<Vec<_>>()
    }

    // ---

    fn build(&self, near: &Vec<Vec3>, far: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>, uvs: &mut Vec<[f32;2]>) {
        let j =  verts.len() as u32;
        verts.extend(
            vec![
                near[3], near[2], far[3], far[2],  // top
                near[0], near[1], far[0], far[1],   // bottom
                near[1], far[1], near[2], far[2],   // right
                far[0], near[0], far[3], near[3]   // left
            ].iter().map(Vec3::to_array)
        );
        idxs.extend_from_slice(&[
            j, j + 1, j + 2,
            j + 2, j + 1, j + 3,

            j + 5, j + 4, j + 6,
            j + 6, j + 7, j + 5,

            j + 8, j + 9, j + 10,
            j + 9, j + 11, j + 10,

            j + 14, j + 12, j + 13,
            j + 14, j + 13, j + 15,
        ]);

        uvs.extend_from_slice(&[
            [1.0, 0.5], [0.0, 0.5], [1.0, 0.0], [0., 0.], // top
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // bottom
            [1.0, 1.0], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right
            [1.0, 1.0], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left
        ]);
    }
}

// -------------------------------
/// Square Channel track profile
pub struct EpSquareChannel {pub half_width: f32, pub height: f32, pub depth: f32, pub border_width: f32 }
impl ElementProfile for EpSquareChannel {
    fn cut(&self,  base: &Vec3, tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3> {
        let normal = Self::normal(tangent, bnormal);

        vec![
            -bnormal * self.half_width,
            bnormal * self.half_width,
            normal * self.height + bnormal * self.half_width ,
            normal * self.height + bnormal * (self.half_width - self.border_width),
            normal * (self.height - self.depth) + bnormal * (self.half_width - self.border_width),
            normal * (self.height - self.depth) - bnormal * (self.half_width - self.border_width),
            normal * self.height - bnormal * (self.half_width -self. border_width),
            normal * self.height - bnormal * self.half_width ,
        ].iter().map(| p | base + p ).collect::<Vec<_>>()

    }

    // ---

    fn build(&self, near: &Vec<Vec3>, far: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>, uvs: &mut Vec<[f32;2]>) {
        let j =  verts.len() as u32;
        verts.extend(
            vec![
                near[0], near[1], far[0], far[1],  //bottom
                near[5], near[4], far[5], far[4], // top
                near[3], near[2], far[3], far[2], // rigth border
                near[7], near[6], far[7], far[6], // left border
                far[4], near[4], far[3], near[3], // rigth inner
                near[5], far[5], near[6], far[6], // left inner
                near[1], far[1], near[2], far[2], // rigth outer
                far[0], near[0], far[7], near[7], // left outer

            ].iter().map(Vec3::to_array)
        );
        idxs.extend_from_slice(&[
            j, j + 2, j + 1,
            j + 2, j + 3, j + 1,

            j + 6, j + 4, j + 5,
            j + 5, j + 7, j + 6,

            j + 8, j + 9, j + 10,
            j + 9, j + 11, j + 10,

            j + 14, j + 12, j + 13,
            j + 14, j + 13, j + 15,

            j + 18, j + 16, j + 17,
            j + 18, j + 17, j + 19,

            j + 21, j + 22, j + 20,
            j + 21, j + 23, j + 22,

            j + 25, j + 26, j + 24,
            j + 25, j + 27, j + 26,

            j + 29, j + 30, j + 28,
            j + 29, j + 31, j + 30,
        ]);

        uvs.extend_from_slice(&[
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // bottom
            [1.0, 0.5], [0.0, 0.5], [1.0, 0.0], [0., 0.], // top
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right border
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left border
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right inner
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left inner
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // right outer
            [1.0, 0.6], [0.0, 1.0], [1.0, 0.6], [0., 0.6], // left outer

        ]);

    }

}

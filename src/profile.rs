use bevy::prelude::*;
pub trait ElementProfile {
    fn cut(&self,  base: &Vec3, tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3>;
    fn build(&self, prev: &Vec<Vec3>, current: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>);
    fn normal(tangent: &Vec3, bnormal: &Vec3)-> Vec3 {
          -tangent.normalize().cross(bnormal.normalize()).normalize()
    }
}

pub struct EpFlat{pub half_width: f32}
impl ElementProfile for EpFlat {
    fn cut(&self, base: &Vec3, _tangent: &Vec3, bnormal: &Vec3) -> Vec<Vec3> {
        vec![
            base - bnormal * self.half_width, base  +  bnormal * self.half_width,
        ]
    }

    // ---

    fn build(&self, prev: &Vec<Vec3>, current: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let j =  verts.len() as u32;
        verts.extend(vec![
            prev[0], prev[1], current[0], current[1],
            ].iter().map(Vec3::to_array)
        );
        idxs.extend_from_slice(&[
            j, j + 1, j + 2,
            j + 2, j + 1, j + 3,
        ]);
    }
}

// -----------------------------------

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

    fn build(&self, prev: &Vec<Vec3>, current: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let j =  verts.len() as u32;
        verts.extend(
            vec![
                prev[3], prev[2], current[3], current[2],  // top
                prev[0], prev[1], current[0], current[1],   // bottom
                prev[1], current[1], prev[2], current[2],   // right
                current[0], prev[0], current[3], prev[3]   // left
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
    }
}

// -------------------------------

pub struct EpSquareChannel {pub half_width: f32, pub height: f32, pub depth: f32, pub border_width: f32}
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

    fn build(&self, prev: &Vec<Vec3>, current: &Vec<Vec3>, verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let j =  verts.len() as u32;
        verts.extend(
            vec![
                prev[0], prev[1], current[0], current[1],  //bottom
                prev[5], prev[4], current[5], current[4], // top
                prev[3], prev[2], current[3], current[2], // rigth border
                prev[7], prev[6], current[7], current[6], // left border
                current[4], prev[4], current[3], prev[3], // rigth inner
                prev[5], current[5], prev[6], current[6], // left inner
                prev[1], current[1], prev[2], current[2], // rigth outer
                current[0], prev[0], current[7], prev[7], // left outer

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
    }

}

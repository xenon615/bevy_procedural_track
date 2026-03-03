use bevy::prelude::*;

// ---

pub trait ElementProfile {
    fn build(&self, current: &(Vec3, Vec3), next: &(Vec3, Vec3),  verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>);
    fn shoulders(current: &(Vec3, Vec3), next: &(Vec3, Vec3), half_width: f32) -> ((Vec3, Vec3), (Vec3, Vec3)) {
        (
            ( current.0 - current.1 * half_width, current.0 + current.1 * half_width),
            ( next.0 - next.1 * half_width, next.0 + next.1 * half_width)
        )
    }
}

#[allow(dead_code)]
pub struct EpFlat { pub half_width: f32 }
impl ElementProfile for EpFlat {
    fn build(&self, current: &(Vec3, Vec3), next: &(Vec3, Vec3),  verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let (current, next) = Self::shoulders(current, next, self.half_width);
        let j = verts.len() as u32;
        verts.extend_from_slice(&[
            current.0.to_array(), current.1.to_array(),  next.0.to_array(), next.1.to_array(),
        ]);
        idxs.extend_from_slice(&[
            j, j + 2, j + 1,
            j + 2, j + 3, j + 1
        ]);
    }
}

#[allow(dead_code)]
pub struct EpBox { pub half_width: f32, pub half_height: f32 }
impl ElementProfile for EpBox {
    fn build(&self, current: &(Vec3, Vec3), next: &(Vec3, Vec3),  verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let (current, next) = Self::shoulders(current, next, self.half_width);
        let j =  verts.len() as u32;
        let norm = (current.1 - current.0).normalize().cross((current.0 - next.0).normalize()).normalize();
        let top = vec![current.0, current.1, next.0, next.1]
            .iter()
            .map(| e | (e + norm * self.half_height).to_array())
            .collect::<Vec<_>>();
        let bottom = vec![current.0, current.1, next.0, next.1]
            .iter()
            .map(| e | (e - norm * self.half_height).to_array())
            .collect::<Vec<_>>();
        let right = vec![current.1 - norm * self.half_height, next.1 - norm * self.half_height , current.1 + norm * self.half_height, next.1 + norm * self.half_height]
            .iter()
            .map(|e| e.to_array())
            .collect::<Vec<_>>();
        let left = vec![current.0 - norm * self.half_height, next.0 - norm * self.half_height , current.0 + norm * self.half_height, next.0 + norm * self.half_height]
            .iter()
            .map(|e| e.to_array())
            .collect::<Vec<_>>()
        ;

        verts.extend_from_slice(&top);
        verts.extend_from_slice(&bottom);
        verts.extend_from_slice(&right);
        verts.extend_from_slice(&left);

        idxs.extend_from_slice(&[
            j, j + 2, j + 1,
            j + 2, j + 3, j + 1,

            j + 5, j + 6, j + 4,
            j + 6, j + 5, j + 7,

            j + 8, j + 10, j + 9,
            j + 9, j + 10, j + 11,

            j + 14, j + 12, j + 13,
            j + 14, j + 13, j + 15,
        ]);
    }
}

#[allow(dead_code)]
pub struct EpSquareChannel { pub half_width: f32, pub height: f32, pub depth: f32, pub border_width: f32}
impl ElementProfile for EpSquareChannel {
    fn build(&self, current: &(Vec3, Vec3), next: &(Vec3, Vec3),  verts: &mut Vec<[f32; 3]>, idxs: &mut Vec<u32>) {
        let (current, next) = Self::shoulders(current, next, self.half_width);
        let j = verts.len() as u32;
        let across = (current.1 - current.0).normalize();
        let along = (next.0 - current.0).normalize();
        let up = -across.cross(along).normalize();


        verts.extend(vec![
            // inner left
            current.0 + across * self.border_width + up * (self.height - self.depth),
            next.0 + across * self.border_width + up * (self.height - self.depth),
            current.0 + across * self.border_width + up * self.height,
            next.0 + across * self.border_width + up * self.height,
            // inner right
            current.1 - across * self.border_width + up * (self.height - self.depth),
            next.1 - across * self.border_width + up * (self.height - self.depth),
            current.1 - across * self.border_width + up * self.height,
            next.1 - across * self.border_width + up * self.height,

            // middle
            current.0 + across * self.border_width + up * (self.height - self.depth),
            current.1 - across * self.border_width + up * (self.height - self.depth),
            next.0 + across * self.border_width + up * (self.height - self.depth),
            next.1 - across * self.border_width + up * (self.height - self.depth),

             // r -border
            current.1 + up * self.height - across * self.border_width,
            current.1 + up * self.height,
            next.1 + up * self.height - across * self.border_width,
            next.1 + up * self.height,

             // l -border
            current.0 + up * self.height,
            current.0 + up * self.height + across * self.border_width,
            next.0 + up * self.height,
            next.0 + up * self.height + across * self.border_width,

            // right
            current.1,
            next.1,
            current.1 + up * self.height,
            next.1 + up * self.height,

            // left
            current.0,
            next.0,
            current.0 + up * self.height,
            next.0 +  up * self.height,

            // bottom
            current.0,
            current.1,
            next.0,
            next.1,
        ]
        .iter()
        .map(Vec3::to_array)
        );
        // println!("{:?}", vs);

        idxs.extend_from_slice(&[
            j, j + 2, j + 1,
            j + 2, j + 3, j + 1,

            j + 6, j + 4, j + 5,
            j + 5, j + 7, j + 6,

            j + 8, j + 10, j + 9,
            j + 9, j + 10, j + 11,

            j + 14, j + 13, j + 12,
            j + 14, j + 15, j + 13,

            j + 18, j + 17, j + 16,
            j + 18, j + 19, j + 17,

            j + 21, j + 20, j + 22,
            j + 21, j + 22, j + 23,

            j + 25, j + 26, j + 24,
            j + 25, j + 27, j + 26,

            j + 29, j + 28, j + 30,
            j + 29, j + 30, j + 31,
        ]);

    }
}

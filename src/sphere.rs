use vertix::prelude::*;

/*fn project_to_unit_sphere(vertices: Vec<Vertex>)
{
  for vertex in vertices {
    let p = vertex.position;
    auto n = norm(p);
    mesh.position(v) = (1.0 / n) * p;
  }
}*/

pub fn icosahedron() -> (Vec<Vertex>, Vec<u32>){
    let phi = (1.0 + 5.0_f32.sqrt()) * 0.5; // golden ratio
    let a = 1.0;
    let b = 1.0 / phi;
    let c = 0.;
    let vertices = vec![
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [c, b, -a],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [b, a, c],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [-b, a, c],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [c, b, a],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [c, -b, a],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [-a, c, b],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [c, -b, -a],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [a, c, -b],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [a, c, b],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [-a, c, -b],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [b, -a, c],
        },
        Vertex {
            tex_coords: [0.1, 0.1],
            position: [-b, -a, c],
        },
    ];

    let indices = vec![2, 1, 0, 1, 2, 3, 5, 4, 3, 4, 8, 3, 7, 6, 0, 6, 9, 0, 11, 10, 4, 10, 11, 6, 9, 5, 2, 5, 9, 11, 8, 7, 1, 7, 8, 10, 2, 5, 3, 8, 1, 3, 9, 2, 0, 1, 7, 0, 11, 9, 6, 7, 10, 6, 5, 11, 4, 10, 8, 4];
    return (vertices,indices);
}

extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::ops;
use std::convert::From;
//use std::num::Float;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SCALE: usize = 1;

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);
#[derive(Clone, Copy)]
struct Point{
    x: f32,
    y: f32, 
}

impl Point{
    fn new(x: f32, y: f32) -> Point{
        Point {
            x: x,
            y: y
        }
    }
}

struct Vec3d(f32, f32, f32);
struct Triangle(Vec3d, Vec3d, Vec3d);
struct Mesh{
    tris: Vec<Triangle>,
}

struct Mat4x4{
    m: [[f32; 4]; 4],
}

impl Mat4x4{
    fn new() -> Mat4x4{
        Mat4x4{
            m: [[0f32; 4]; 4] 
        }
    }
}

impl Mesh{
    fn new(tris: Vec<Triangle>) -> Mesh{
        Mesh{
            tris: tris
        }
    }
}

impl ops::Add<Color> for u32 {

    type Output = u32;

    fn add(self, _rhs: Color) -> u32 {
        let mut red: u32 = _rhs.0 as u32;
        let green: u32 = _rhs.1 as u32;
        let mut blue: u32 = _rhs.2 as u32;

        red = red << 16;
        blue = blue << 8;

        let result: u32 = red + blue + green;

        return result;
    }
}

impl From<Color> for u32 {
    fn from(item: Color) -> Self{
        0 + item
    }
}

fn multiply_matrix_vector(i: &Vec3d, o: &mut Vec3d, m: &Mat4x4){
    o.0 = i.0 * m.m[0][0] + i.1 * m.m[1][0] + i.2 * m.m[2][0] + m.m[3][0]; 
    o.1 = i.0 * m.m[0][1] + i.1 * m.m[1][1] + i.2 * m.m[2][1] + m.m[3][1];
    o.2 = i.0 * m.m[0][2] + i.1 * m.m[1][2] + i.2 * m.m[2][2] + m.m[3][2]; 
    let w = i.0 * m.m[0][3] + i.1 * m.m[1][3] + i.2 * m.m[2][3] + m.m[3][3]; 

    if w != 0.0{
        o.0 /= w;
        o.1 /= w;
        o.2 /= w; 
    }
}

fn plot(x: f32, y: f32, brightness: f32, buffer: &mut Vec<u32>){
    let x = x * (SCALE as f32);
    let y = y * (SCALE as f32);
    let mut points = [[0f32; SCALE]; SCALE];

    for y1 in 0..SCALE{
        let mut point = ((y + (y1 as f32)) - 1.0) * (WIDTH as f32) + x - 1.0;
        for x1 in 0..SCALE{
            if !(point >= (WIDTH * HEIGHT) as f32){
                points[x1][y1] = point;
            }
            point += 1.0;
        }
    }

    let c = (255.0 * brightness) as u8;
    for temp in &points{  
        for val in temp{
            buffer[*val as usize] = Color(c, c, c).into();
        }
    }
}

fn ipart(i : f32) -> f32{
    i.trunc()
}

fn round(i : f32) -> f32{
    ipart(i + 0.5) as f32
}

fn fpart(i : f32) -> f32{
    i - i.trunc() as f32
}

fn rfpart(i : f32) -> f32{
    return 1.0 - fpart(i);
}

fn draw_line(p1: Point, p2: Point, buff: &mut Vec<u32>){
    let mut x0 = p1.x;
    let mut y0 = p1.y;
    let mut x1 = p2.x;
    let mut y1 = p2.y;

    let dif1: f32 = y1-y0;
    let dif2: f32 = x1-x0;

    let steep: bool = dif1.abs() > dif2.abs(); 

    if steep {
        let temp = x0;
        x0 = y0;
        y0 = temp;

        let temp = x1;
        x1 = y1;
        y1 = temp;
    }
    if x0 > x1 {
        let temp = x0;
        x0 = x1;
        x1 = temp;

        let temp = y0;
        y0 = y1;
        y1 = temp;
    }

    let dx: f32 = x1 - x0;
    let dy: f32 = y1 - y0;
    let mut grad: f32 = dy/dx;

    if dx == 0.0 {
        grad = 1.0;
    }

    let xend = round(x0);
    let yend = (y0 as f32) + grad*((xend - x0) as f32);
    let xgap = rfpart(x0 as f32 + 0.5);
    let xpxl1 = xend;
    let ypxl1 = ipart(yend);

    if steep{
        plot(ypxl1, xpxl1, rfpart(yend) * xgap, buff);
        plot(ypxl1+1.0, xpxl1, fpart(yend) * xgap, buff);
    } else {
        plot(xpxl1, ypxl1, rfpart(yend) * xgap, buff);
        plot(xpxl1, ypxl1 + 1.0, fpart(yend) * xgap, buff);
    }

    let mut intery = yend + grad;
    let xend = round(x1);
    let yend = y1 + grad * (xend - x1);
    let xgap = fpart(x1 + 0.5);
    let xpxl2 = xend;
    let ypxl2 = ipart(yend);

    if steep {
        plot(ypxl2, xpxl2, rfpart(yend) * xgap, buff);
        plot(ypxl2 + 1.0, xpxl2, fpart(yend) * xgap, buff);
    } else {
        plot(xpxl2, ypxl2, rfpart(yend) * xgap, buff);
        plot(xpxl2, ypxl2 + 1.0, fpart(yend) * xgap, buff);
    }

    if steep{
        for x in (xpxl1 as usize + 1)..(xpxl2 as usize){
            plot(ipart(intery), x as f32, rfpart(intery), buff);
            plot(ipart(intery) + 1.0, x as f32, fpart(intery), buff);
            intery = intery + grad;
        }
    } else {
        for x in (xpxl1 as usize + 1)..(xpxl2 as usize){
            plot(x as f32, ipart(intery), rfpart(intery), buff);
            plot(x as f32, ipart(intery) + 1.0, fpart(intery), buff);
            intery = intery + grad;
        }
    }
}

fn draw_triangle(p1: Point, p2: Point, p3: Point, buff: &mut Vec<u32>){
    draw_line(p1, p2, buff);
    draw_line(p2, p3, buff);
    draw_line(p3, p1, buff);
}


fn main() {
    let cube: Vec<Triangle> = vec![
        //SOUTH
        Triangle(Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 1.0, 0.0), Vec3d(1.0, 1.0, 0.0)),
        Triangle(Vec3d(0.0, 0.0, 0.0), Vec3d(1.0, 1.0, 0.0), Vec3d(1.0, 0.0, 0.0)),
        //EAST
        Triangle(Vec3d(1.0, 0.0, 0.0), Vec3d(1.0, 1.0, 0.0), Vec3d(1.0, 1.0, 1.0)),
        Triangle(Vec3d(1.0, 0.0, 0.0), Vec3d(1.0, 1.0, 1.0), Vec3d(1.0, 0.0, 1.0)),
        //NORTH
        Triangle(Vec3d(1.0, 0.0, 1.0), Vec3d(1.0, 1.0, 1.0), Vec3d(0.0, 1.0, 1.0)),
        Triangle(Vec3d(1.0, 0.0, 1.0), Vec3d(0.0, 1.0, 1.0), Vec3d(0.0, 0.0, 1.0)),
        //WEST
        Triangle(Vec3d(0.0, 0.0, 1.0), Vec3d(0.0, 1.0, 1.0), Vec3d(0.0, 1.0, 0.0)),
        Triangle(Vec3d(0.0, 0.0, 1.0), Vec3d(0.0, 1.0, 0.0), Vec3d(0.0, 0.0, 0.0)),
        //TOP
        Triangle(Vec3d(0.0, 1.0, 0.0), Vec3d(0.0, 1.0, 1.0), Vec3d(1.0, 1.0, 1.0)),
        Triangle(Vec3d(0.0, 1.0, 0.0), Vec3d(1.0, 1.0, 1.0), Vec3d(1.0, 1.0, 0.0)),
        //BOTTOM
        Triangle(Vec3d(1.0, 0.0, 1.0), Vec3d(0.0, 0.0, 1.0), Vec3d(0.0, 0.0, 0.0)),
        Triangle(Vec3d(1.0, 0.0, 1.0), Vec3d(0.0, 0.0, 0.0), Vec3d(1.0, 0.0, 0.0))
    ];

    //Projection Matrix
    let near = 0.1;
    let far = 1000.0;
    let fov = 90.0;
    let aspect_ratio = (HEIGHT as f32)/(WIDTH as f32);
    let fov_rad = 1.0 / (((fov*0.5 / 180.0 * 3.14159) as f32).tan());

    let mesh_cube = Mesh::new(cube);
    let mut mat_proj = Mat4x4::new();
    let v_camera = Vec3d(0.0, 0.0, 0.0);

    mat_proj.m[0][0] = aspect_ratio * fov_rad;
    mat_proj.m[1][1] = fov_rad;
    mat_proj.m[2][2] = far / (far - near);
    mat_proj.m[3][2] = (-far * near) / (far - near);
    mat_proj.m[2][3] = 1.0;
    mat_proj.m[3][3] = 0.0;

    let mut window = Window::new(
        "Cube Test! - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut theta: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {    

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        // max 125 bc 1 unit = 4x4 px
        //let p1 = Point::new(0, 0);
        //let p2 = Point::new(124, 300);

        let mut mat_rot_z = Mat4x4::new();
        let mut mat_rot_zx = Mat4x4::new();

        theta += 0.05;

        mat_rot_z.m[0][0] = theta.cos();
        mat_rot_z.m[0][1] = theta.sin();
        mat_rot_z.m[1][0] = -(theta.sin());
        mat_rot_z.m[1][1] = theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        mat_rot_zx.m[0][0] = 1.0;
        mat_rot_zx.m[1][1] = (theta * 0.5).cos();
        mat_rot_zx.m[1][2] = (theta * 0.5).sin();
        mat_rot_zx.m[2][1] = -((theta * 0.5).sin());
        mat_rot_zx.m[2][2] = (theta * 0.5).cos();
        mat_rot_zx.m[3][3] = 1.0;
       
        for tri in &mesh_cube.tris{
            let mut tri_projected = Triangle(Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0));
            let mut tri_rotated_z = Triangle(Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0));
            let mut tri_rotated_zx = Triangle(Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0), Vec3d(0.0, 0.0, 0.0));


            multiply_matrix_vector(&tri.0, &mut tri_rotated_z.0, &mat_rot_z);
            multiply_matrix_vector(&tri.1, &mut tri_rotated_z.1, &mat_rot_z);
            multiply_matrix_vector(&tri.2, &mut tri_rotated_z.2, &mat_rot_z);

            multiply_matrix_vector(&tri_rotated_z.0, &mut tri_rotated_zx.0, &mat_rot_zx);
            multiply_matrix_vector(&tri_rotated_z.1, &mut tri_rotated_zx.1, &mat_rot_zx);
            multiply_matrix_vector(&tri_rotated_z.2, &mut tri_rotated_zx.2, &mat_rot_zx);

            let mut tri_translated = Triangle(Vec3d(tri_rotated_zx.0.0, tri_rotated_zx.0.1, tri_rotated_zx.0.2),
                                    Vec3d(tri_rotated_zx.1.0, tri_rotated_zx.1.1, tri_rotated_zx.1.2), 
                                    Vec3d(tri_rotated_zx.2.0, tri_rotated_zx.2.1, tri_rotated_zx.2.2));
            tri_translated.0.2 = tri_rotated_zx.0.2 + 3.0;
            tri_translated.1.2 = tri_rotated_zx.1.2 + 3.0;
            tri_translated.2.2 = tri_rotated_zx.2.2 + 3.0;

            let mut normal= Vec3d(0.0, 0.0, 0.0);
            let mut line1 = Vec3d(0.0, 0.0, 0.0);
            let mut line2 = Vec3d(0.0, 0.0, 0.0);

            line1.0 = tri_translated.1.0 - tri_translated.0.0;
            line1.1 = tri_translated.1.1 - tri_translated.0.1;
            line1.2 = tri_translated.1.2 - tri_translated.0.2;

            line2.0 = tri_translated.2.0 - tri_translated.0.0;
            line2.1 = tri_translated.2.1 - tri_translated.0.1;
            line2.2 = tri_translated.2.2 - tri_translated.0.2;

            normal.0 = line1.1 * line2.2 - line1.2 * line2.1;
            normal.1 = line1.2 * line2.0 - line1.0 * line2.2;
            normal.2 = line1.0 * line2.1 - line1.1 * line2.0;

            let length = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
            normal.0 /= length;
            normal.1 /= length;
            normal.2 /= length;

            if (normal.0 * (tri_translated.0.0 - v_camera.0) +
                normal.1 * (tri_translated.0.1 - v_camera.1) +
                normal.2 * (tri_translated.0.2 - v_camera.2) < 0.0) {
                multiply_matrix_vector(&tri_translated.0, &mut tri_projected.0, &mat_proj);
                multiply_matrix_vector(&tri_translated.1, &mut tri_projected.1, &mat_proj);
                multiply_matrix_vector(&tri_translated.2, &mut tri_projected.2, &mat_proj);

                tri_projected.0.0 += 1.0;
                tri_projected.0.1 += 1.0;
                tri_projected.1.0 += 1.0;
                tri_projected.1.1 += 1.0;
                tri_projected.2.0 += 1.0;
                tri_projected.2.1 += 1.0;

                tri_projected.0.0 *= 0.5 * (WIDTH as f32)/(SCALE as f32);
                tri_projected.0.1 *= 0.5 * (WIDTH as f32)/(SCALE as f32);
                tri_projected.1.0 *= 0.5 * (WIDTH as f32)/(SCALE as f32);
                tri_projected.1.1 *= 0.5 * (WIDTH as f32)/(SCALE as f32);
                tri_projected.2.0 *= 0.5 * (WIDTH as f32)/(SCALE as f32);
                tri_projected.2.1 *= 0.5 * (WIDTH as f32)/(SCALE as f32);

                draw_triangle(Point::new(tri_projected.0.0, tri_projected.0.1),
                                Point::new(tri_projected.1.0, tri_projected.1.1),
                                Point::new(tri_projected.2.0, tri_projected.2.1), &mut buffer);
            }
        }
       
        

        
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
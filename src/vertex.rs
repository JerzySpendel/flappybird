use glium;
use glium::backend::Facade;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    position: [f32; 3],
}

#[derive(Copy, Clone, Debug)]
pub struct UVPoint {
    uv: [f32; 2],
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { position: [x, y, 1.] }
    }
    pub fn standard_rectangle() -> [Point; 4] {
        [
            Point::new(-1., 1.),
            Point::new(-1., -1.),
            Point::new(1., -1.),
            Point::new(1., 1.),
        ]
    }
    pub fn standard_rectangle_buffer(display: &dyn Facade) -> glium::VertexBuffer<Point> {
        glium::VertexBuffer::new(display, &Point::standard_rectangle()).unwrap()
    }
}

impl UVPoint {
    pub fn new(x: f32, y: f32) -> UVPoint {
        UVPoint { uv: [x, y] }
    }
    pub fn standard_rectangle() -> [UVPoint; 4] {
        [
            UVPoint::new(0., 1.),
            UVPoint::new(0., 0.),
            UVPoint::new(1., 0.),
            UVPoint::new(1., 1.),
        ]
    }
    pub fn standard_rectangle_buffer(display: &dyn Facade) -> glium::VertexBuffer<UVPoint> {
        glium::VertexBuffer::new(display, &UVPoint::standard_rectangle()).unwrap()
    }
}

glium::implement_vertex!(Point, position);
glium::implement_vertex!(UVPoint, uv);


use rtree::point::Point;

#[derive(Debug, Clone)]
pub struct RectSpan {
    pub pt1: Point,
    pub pt2: Point
}

#[derive(Debug)]
pub enum Rect {
    Empty,
    Span(RectSpan)
}

impl Rect {
  pub fn empty() -> Rect {
      Rect::Empty
  }

  pub fn new(pt1: Point, pt2: Point) -> Rect {
    Rect::Span(RectSpan {pt1: pt1, pt2:pt2})
  }

  pub fn union(&self, other: &Rect) -> Rect {
    match (self,other) {
      (&Rect::Empty,&Rect::Empty) => Rect::Empty,
      (&Rect::Span(ref s),&Rect::Empty) => Rect::Span(s.clone()),
      (&Rect::Empty,&Rect::Span(ref s)) => Rect::Span(s.clone()),
      (&Rect::Span(ref s1),&Rect::Span(ref s2)) => Rect::new(
        Point::new(if s1.pt1.x < s2.pt1.x {s1.pt1.x} else {s2.pt1.x},if s1.pt1.y < s2.pt1.y {s1.pt1.y} else {s2.pt1.y}),
        Point::new(if s1.pt2.x > s2.pt2.x {s1.pt2.x} else {s2.pt2.x},if s1.pt2.y > s2.pt2.y {s1.pt2.y} else {s2.pt2.y})
      ),
    }
  }
}

pub trait Rectable {
  fn rect(&self) -> Rect;
}
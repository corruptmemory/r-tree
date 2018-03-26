
mod rtree;
use rtree::rect::Rect;

fn main() {
  let r1 = Rect::empty();
  let r2 = Rect::empty();
  let r3 = r1.union(&r2);
  println!("Rect: {:?}",r3);
}

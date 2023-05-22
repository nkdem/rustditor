use std::io::Write;

pub trait View {
    fn render(&self, out: &mut impl Write);
}
use std::io::Write;

pub trait View {
    type Output;
    fn render(&self, out: &mut impl Write) -> Self::Output;
}
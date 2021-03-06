use ffi::size::Size;
use measures::Measures;
use std::fmt;

pub struct Context<'a, 'm> {
    text: &'a str,
    measurer: &'m Measures,
}

impl<'a, 'm> Context<'a, 'm> {
    pub fn new(text: &'a str, measures: &'m Measures) -> Context<'a, 'm> {
        Context {
            text: text,
            measurer: measures,
        }
    }

    pub fn measure(&self) -> Size {
        self.measurer.measure(self.text)
    }
}

impl<'a, 'm> fmt::Debug for Context<'a, 'm> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context {{ text: {} }}", self.text)
    }
}

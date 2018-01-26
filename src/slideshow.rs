use slide::Slide;

pub(crate) struct Slideshow<'font> {
    pub(crate) current_slide: usize,
    pub(crate) slides: Vec<Slide<'font>>,
}

impl<'font> Slideshow<'font> {
    pub(crate) fn new() -> Self {
        Slideshow {
            current_slide: 0,
            slides: Vec::new(),
        }
    }

    pub(crate) fn add(&mut self, slide: Slide<'font>) {
        self.slides.push(slide);
    }

    pub(crate) fn len(&self) -> usize {
        self.slides.len()
    }
}

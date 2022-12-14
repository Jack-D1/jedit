pub enum Type {
    None, 
    Number,
}

imply Type {
    pub fn to_color(&self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220,163,163),
            _ => color::Rgb(255,255,255),
        }
    }
}
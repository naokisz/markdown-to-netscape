#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub title: String,
    pub url: String,
}

impl Link {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

#[cfg(test)]
mod tests {
    use super::Link;

    #[test]
    fn link_new() {
        let l = Link::new("t".into(), "u".into());
        assert_eq!(l.title, "t");
        assert_eq!(l.url, "u");
    }
}

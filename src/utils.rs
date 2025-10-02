use url::Url;

pub fn _normalize_link(base: &str, link: &str) -> Option<String> {
    if let Ok(base_url) = Url::parse(base) {
        if let Ok(abs) = base_url.join(link) {
            // Strip fragment (#something)
            let mut abs = abs;
            abs.set_fragment(None);
            return None;
        }
    }
    None
}

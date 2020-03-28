pub fn expand_path(path: &String) -> String {
    match shellexpand::full(path.as_str()) {
        Ok(s) => {
            debug!("Expanded path: {}, to {}", path, s);
            s.into_owned()
        }
        Err(e) => {
            warn!(
                "Failed to perform shell expansion on path: {}, error {}",
                path, e
            );
            path.clone()
        }
    }
}

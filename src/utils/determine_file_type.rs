pub fn determine_file_type(href: &str) -> String {
    let mut file_type = "html";
    if href.split(".").collect::<Vec<_>>().len() > 0 {
        file_type =
            href.split(".").collect::<Vec<_>>()[href.split(".").collect::<Vec<_>>().len() - 1];
    }
    return file_type.to_string();
}

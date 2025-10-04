use std::collections::HashMap;

/// Converts text into a slug (safe identifier string).
pub fn slugify(s: &str) -> String {
    let acute_to_plain: HashMap<char, char> = HashMap::from([
        ('á', 'a'), ('é', 'e'), ('í', 'i'), ('ó', 'o'), ('ú', 'u'), ('ý', 'y'),
        ('Á', 'A'), ('É', 'E'), ('Í', 'I'), ('Ó', 'O'), ('Ú', 'U'), ('Ý', 'Y'),
    ]);

    s.chars()
        // Replace accented letters
        .map(|c| *acute_to_plain.get(&c).unwrap_or(&c))
        // Convert to lowercase
        .map(|c| c.to_ascii_lowercase())
        // Replace spaces with underscores
        .map(|c| if c == ' ' { '_' } else { c })
        // Keep only ASCII letters, digits, and underscores
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}

#[derive(Debug, Clone)]
pub struct BashrcLine {
    pub name: String,
    pub count: usize,
}

impl BashrcLine {
    pub fn new(name: String, count: usize) -> Self {
        BashrcLine { name, count }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        if !line.starts_with("# archer(") {
            return None;
        }

        let inner = &line["# archer(".len()..line.len() - 1];

        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return None;
        }

        let name = parts[0].to_string();
        let count = parts[1].parse::<usize>().ok()?;

        Some(BashrcLine { name, count })
    }

    pub fn to_identifier(&self) -> String {
        format!("# archer({}, {})", self.name, self.count)
    }
}

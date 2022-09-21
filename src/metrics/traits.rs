pub trait Metrics: Clone + Default {
    fn add_metrics(&mut self, other: &Self);
    fn normalize_by(&mut self, n: u32);
    fn format_to_string(&self) -> String;
}

pub trait Series {
    fn add_layer(&mut self, other: &Self);
    fn normalize_by(&mut self, n: u32);
    fn format_to_file(&self, header: String) -> String;
}

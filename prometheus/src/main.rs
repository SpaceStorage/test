mod metrics;

fn main() {
    let mut metrics_tree = metrics::prometheus::Prometheus::new();

    metrics_tree.access.with_label_values(&["myproj", "select"]).inc();
    metrics_tree.response_time.with_label_values(&["myproj", "select", "0.5"]).set(0.3);
    let mut metrics_str = metrics_tree.get_metrics();
    println!("{}", String::from_utf8(metrics_str).unwrap());
}

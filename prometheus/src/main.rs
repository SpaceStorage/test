mod metrics;

fn main() {
    let metricsTree = metrics::prometheus::Prometheus::new();
    metricsTree::init();
    ////metrics::prometheus::prometheus_print(r);

    //let mut metrics_tree = metrics::prometheus::prometheus_get_metrics(r.clone());
    //println!("{}", String::from_utf8(metrics_tree).unwrap());
    //metrics_tree = metrics::prometheus::prometheus_get_metrics(r);
    //println!("{}", String::from_utf8(metrics_tree).unwrap());
}

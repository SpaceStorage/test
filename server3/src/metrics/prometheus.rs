use prometheus::{Encoder, Opts, Registry, TextEncoder, CounterVec, GaugeVec};

pub struct Prometheus {
    r: prometheus::Registry,
    pub access: prometheus::CounterVec,
    pub access_received_bytes: prometheus::CounterVec,
    pub response_time: prometheus::GaugeVec,
}

impl Prometheus {
    pub fn new() -> Self {
        let access_opts = Opts::new("spacestorage_access", "access queries");
        let access_received_bytes_opts = Opts::new("spacestorage_received_bytes", "received bytes");
        let response_time_opts = Opts::new("spacestorage_response_time", "response time");
        let metric_tree = Prometheus {
            r: Registry::new(),
            access: CounterVec::new(access_opts, &["namespace", "project", "operation"]).unwrap(),
            access_received_bytes: CounterVec::new(access_received_bytes_opts, &["namespace", "project", "operation"]).unwrap(),
            response_time: GaugeVec::new(response_time_opts, &["project", "operation", "quantile"]).unwrap(),
        };

        metric_tree.r.register(Box::new(metric_tree.access.clone())).unwrap();
        metric_tree.r.register(Box::new(metric_tree.access_received_bytes.clone())).unwrap();
        metric_tree.r.register(Box::new(metric_tree.response_time.clone())).unwrap();

        return metric_tree;
    }

    pub fn get_metrics(&mut self) -> std::vec::Vec::<u8> {
        let mut buffer = Vec::<u8>::new();
        let encoder = TextEncoder::new();
        let metric_families = self.r.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        return buffer;
    }
}

//use std::time::Duration;
//use std::thread;

use prometheus::{Encoder, Opts, Registry, TextEncoder, CounterVec, GaugeVec};

pub struct Prometheus {
    r: prometheus::Registry,
    pub access: prometheus::CounterVec,
    pub response_time: prometheus::GaugeVec,
}

impl Prometheus {
    pub fn new() -> Self {
        let access_opts = Opts::new("spacestorage_access", "access queries");
        let response_time_opts = Opts::new("spacestorage_response_time", "response time");
        let metric_tree = Prometheus {
            r: Registry::new(),
            access: CounterVec::new(access_opts, &["project", "operation"]).unwrap(),
            response_time: GaugeVec::new(response_time_opts, &["project", "operation", "quantile"]).unwrap(),
        };

        metric_tree.r.register(Box::new(metric_tree.access.clone())).unwrap();
        metric_tree.r.register(Box::new(metric_tree.response_time.clone())).unwrap();

        return metric_tree;
    }

    ////pub fn init(&self) -> prometheus::Registry {
    //pub fn init(&mut self) {
    //    //thread::spawn(move || {
    //    //    for _ in 0..10 {
    //    //        thread::sleep(Duration::from_millis(500));
    //    //        c2.inc();
    //    //        cv2.with_label_values(&["3", "4"]).inc();
    //    //        g2.inc();
    //    //        gv2.with_label_values(&["3", "4"]).inc();
    //    //    }
    //    //});
    //
    //    //thread::spawn(move || {
    //    //    for _ in 0..5 {
    //    //        thread::sleep(Duration::from_secs(1));
    //    //        counter.inc();
    //    //        counter_vec.with_label_values(&["3", "4"]).inc();
    //    //        gauge.dec();
    //    //        gauge_vec.with_label_values(&["3", "4"]).set(42.0);
    //    //    }
    //    //});
    //}

    pub fn get_metrics(&mut self) -> std::vec::Vec::<u8> {
        let mut buffer = Vec::<u8>::new();
        let encoder = TextEncoder::new();
        let metric_families = self.r.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        //println!("{}", String::from_utf8(buffer.clone()).unwrap());
        //buffer.clear();
        return buffer;
    }
}

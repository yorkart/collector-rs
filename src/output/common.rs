use core;

pub struct PackageTypeMap<'a> {
    map : Vec<&'a str>,
}

impl <'a> PackageTypeMap<'a> {
    pub fn new() -> PackageTypeMap<'a> {
        let mut package_type_vec = Vec::with_capacity(128);
        (0..128).for_each(|_| package_type_vec.push(""));

        package_type_vec[core::AGENT_INFO] = "apm-agent-info";
        package_type_vec[core::AGENT_INFO_V2] = "apm-agent-info";

        package_type_vec[core::METADATA_API] = "apm-metadata";
        package_type_vec[core::METADATA_API_V2] = "apm-metadata";

        package_type_vec[core::AGENT_STAT] = "apm-stat";
        package_type_vec[core::AGENT_STAT_V2] = "apm-stat";

        package_type_vec[core::TRACE_ORIGIN] = "apm-trace";
        package_type_vec[core::TRACE_ORIGIN_V2] = "apm-trace";

        PackageTypeMap{
            map: package_type_vec,
        }
    }

    pub fn get_topic(&self, package_type: usize) ->&str {
        if package_type > 127 {
            return "rust-demo"
        }
        self.map[package_type]
    }
}

//pub fn get_package_type() {
//    let mut package_type_vec = Vec::new();
//
//    package_type_vec[AGENT_INFO] = "apm-agent-info";
//    package_type_vec[AGENT_INFO_V2] = "apm-agent-info";
//
//    package_type_vec[METADATA_API] = "apm-metadata";
//    package_type_vec[METADATA_API_V2] = "apm-metadata";
//
//    package_type_vec[AGENT_STAT] = "apm-stat";
//    package_type_vec[AGENT_STAT_V2] = "apm-stat";
//
//    package_type_vec[TRACE_ORIGIN] = "apm-trace";
//    package_type_vec[TRACE_ORIGIN_V2] = "apm-trace";
//
//    unsafe {
//        PACKAGE_TYPE = Some(package_type_vec);
//    }
////    package_type_vec.as_ref()
//
////    let package_type: &'static [String; 127];
////
////    package_type[AGENT_INFO] = "apm-agent-info".to_owned();
////    package_type[AGENT_INFO_V2] = "apm-agent-info".to_owned();
////
////    package_type[METADATA_API] = "apm-metadata".to_owned();
////    package_type[METADATA_API_V2] = "apm-metadata".to_owned();
////
////    package_type[AGENT_STAT] = "apm-stat".to_owned();
////    package_type[AGENT_STAT_V2] = "apm-stat".to_owned();
////
////    package_type[TRACE_ORIGIN] = "apm-trace".to_owned();
////    package_type[TRACE_ORIGIN_V2] = "apm-trace".to_owned();
////
////    package_type
//}
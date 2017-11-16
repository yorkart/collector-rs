
use time;

pub fn get_mills(time_spec: time::Timespec) -> i64{
    time_spec.sec + time_spec.nsec as i64 / (1000 * 1000)
}
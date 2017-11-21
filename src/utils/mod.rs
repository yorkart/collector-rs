
mod file;

use time;

pub fn get_mills(time_spec: time::Timespec) -> i64{
    time_spec.sec * 1000 + time_spec.nsec as i64 / 1000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_mills_test() {
        let t = time::get_time();
        println!("{}", get_mills(t));
    }
}
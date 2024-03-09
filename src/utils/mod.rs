use chrono::NaiveTime;

pub mod api;
pub mod cookie;

pub trait ToNaiveTime {
    fn to_navie_time(self) -> NaiveTime;
}

impl ToNaiveTime for String {
    fn to_navie_time(self) -> NaiveTime {
        let components: Vec<&str> = self.split(":").collect();
        if components.len() != 2 && components.len() != 3 {
            panic!("Format is something like 03:33, 17:23 or 13:02:33")
        }
        let hour = components[0].parse::<u32>().unwrap();
        let min = components[1].parse::<u32>().unwrap();
        let sec: u32 = if components.len() == 3 {
            components[2].parse::<u32>().unwrap()
        } else {
            0
        };
        return NaiveTime::from_hms_opt(hour, min, sec).unwrap().to_owned();
    }
}

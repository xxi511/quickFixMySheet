use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeSheets {
    pub employee_sheets: Vec<Sheet>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub id: i32,
    pub cycle_start_date: String,
    pub locked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attendance {
    pub attendance: Vec<AttendanceData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceData {
    pub date: String,
    note: AttendanceNote,
    paid_timeoff_seconds: i32,
    timeoff_description: String,
}

impl AttendanceData {
    pub fn needs_clock_in(&self) -> bool {
        match &self.note.description {
            Some(v) => {
                if self.timeoff_description.is_empty() && !v.is_empty() {
                    // National holiday 
                    return false;
                }
            },
            _ => (),
        }
        return self.paid_timeoff_seconds == 0;
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceNote {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub start: String,
    pub end: String,
    pub offset: i32,
    //[{"id":30668679,"start":"2024-02-29T09:25","end":"2024-02-29T18:00","offset":-480}]
}

impl Entry {
    pub fn new(start: String, end: String, offset: i32) -> Self {
        Self { start: start, end: end, offset: offset }
    }

    pub fn to_array(self) -> Vec<Entry> {
        let data = vec![self];
        return data;
    }
}
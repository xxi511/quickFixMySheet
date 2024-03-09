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

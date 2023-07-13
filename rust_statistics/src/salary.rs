use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salaryinusd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
}

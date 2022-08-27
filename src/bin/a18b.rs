// Topic: Result & the question mark operator
#![allow(dead_code)]
#![allow(unused_must_use)]
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum EmployeeRole {
    MaintenanceCrew,
    MarketingDept,
    Managers,
    LineSupervisor,
    KichenStaff,
    AssTechnicians,
}

#[derive(Debug)]
enum EmployeeStatus{
    Employed,
    Terminated,
}

#[derive(Debug)]
struct Employee{
    role: EmployeeRole,
    status: EmployeeStatus,
}

impl  Employee {
    fn check_status (self) -> Result<Employee, String>{
        match self.role {
            EmployeeRole::MaintenanceCrew | EmployeeRole::Managers | EmployeeRole::MarketingDept => {
                match self.status {
                    EmployeeStatus::Employed => Ok(self),
                    EmployeeStatus::Terminated => Err("statys terminated".to_owned())
                }
            },
            _ => Err("Role not allowed".to_owned())
        }
    }
}

fn get_status () -> Result<(), String> {

    let erick = Employee{
        role: EmployeeRole::Managers,
        status: EmployeeStatus::Employed,
    };

    let erick_status = erick.check_status()?;
    println!("{:?}", erick_status);
    Ok(())
} 

fn main() {
    match get_status() {
        Ok(()) => println!("access granted"),
        Err(message) => println!("access denied : {}", message),
    }
}

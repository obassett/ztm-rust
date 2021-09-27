// Topic: Result & the question mark operator
//
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

#[derive(PartialEq)]
enum EmployeePostion {
    MaintenanceCrew,
    Marketing,
    Manager,
    LineSuperviosr,
    KitchenStaff,
    AssemblyTechnician,
}

struct Employee {
    name: String,
    position: EmployeePostion,
    is_employed: bool,
}

fn access_allowed(employee: &Employee) -> Result<(), String> {
    let allowed_roles = vec![
        EmployeePostion::MaintenanceCrew,
        EmployeePostion::Marketing,
        EmployeePostion::Manager,
    ];

    if !employee.is_employed {
        return Err("Access Denied - No Longer Employed Here".to_owned());
    }

    if !allowed_roles.contains(&employee.position) {
        return Err("Access Denied - Employee is unauthorised".to_owned());
    }
    Ok(())
}

fn print_access(employee: &Employee) -> Result<(), String> {
    access_allowed(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let richard = Employee {
        name: "Richard".to_owned(),
        position: EmployeePostion::MaintenanceCrew,
        is_employed: true,
    };

    if let Err(e) = print_access(&richard) {
        println!("access denied: {:?}", e)
    }
    // match print_access(&richard) {
    //     Err(e) => println!("access denied: {:?}", e),
    //     _ => (),
    // }
}

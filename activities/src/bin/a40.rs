// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}
#[derive(Debug)]
struct Rental {
    vin: String,
    vehicle_type: Vehicle,
    status: VehicleStatus,
}

#[derive(Debug)]
struct Corporate(Rc<RefCell<Vec<Rental>>>);
#[derive(Debug)]
struct StoreFront(Rc<RefCell<Vec<Rental>>>);

fn new_rental_car(vin: String, vehicle_type: Vehicle, status: VehicleStatus) -> Rental {
    Rental {
        vin,
        vehicle_type,
        status,
    }
}

fn main() {

    // let rentals = Rc::new(RefCell::new(vec![]));
    // let corporate = Corporate(Rc::clone(&rentals));
    // let branch = StoreFront(Rc::clone(&rentals));

    // let rental = new_rental_car( "ABC123".to_owned(), Vehicle::Truck, VehicleStatus::Unavailable);
    // let rental2 = new_rental_car( "XYZ321".to_owned(), Vehicle::Car, VehicleStatus::Unavailable);

    // rentals.borrow_mut().push(rental);
    // rentals.borrow_mut().push(rental2);

    // let my_car = corporate
}
#[cfg(test)]
mod test {
    use super::*;

    fn update_state() {
        let vehicles = vec![
            Rental {
                status: VehicleStatus::Available,
                vehicle_type: Vehicle::Car,
                vin: "XYZ123".to_owned(),
            },
            Rental {
                status: VehicleStatus::Maintenance,
                vehicle_type: Vehicle::Truck,
                vin: "123ABC".to_owned(),
            },
        ];
        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = storefront.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, VehicleStatus::Available);
                car.status = VehicleStatus::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, VehicleStatus::Rented);
                car.status = VehicleStatus::Available;
            }
        }

        let rentals = storefront.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, VehicleStatus::Available);
        }
    }
}

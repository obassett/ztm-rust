// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
    id: String,
    state: State,
}

impl<State> Luggage<State> {
    fn scan<NextState>(self, state: NextState) -> Luggage<NextState> {
        Luggage {
            id: self.id,
            state: state,
        }
    }
}

struct CheckIn;

impl Luggage<CheckIn> {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            state: CheckIn,
        }
    }

    fn load_bag(self) -> Luggage<OnLoading> {
        self.scan(OnLoading)
    }
}

struct OnLoading;

impl Luggage<OnLoading> {
    fn offload_bag(self) -> Luggage<Offloading> {
        self.scan(Offloading)
    }
}
struct Offloading;

impl Luggage<Offloading> {
    fn baggage_claim(self) -> Luggage<AwaitingPickup> {
        self.scan(AwaitingPickup)
    }
}
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn picked_up(self) -> Luggage<EndCustody> {
        self.scan(EndCustody)
    }
}

struct EndCustody;

fn main() {
    let my_bag = Luggage::new("AWESOMEBAG");

    let bye_bag = my_bag.load_bag().offload_bag().baggage_claim().picked_up();
}

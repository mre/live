extern crate rand;
extern crate rayon;

use std::collections::BTreeSet;
use rand::Rng;
use rayon::prelude::*;

#[derive(PartialOrd, Ord, Clone, Debug, Hash)]
struct City {
    name: String,
    prices: Vec<Option<u64>>,
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name
    }
}
impl Eq for City {}

impl City {
    fn new(name: String, prices: Vec<Option<u64>>) -> City {
        City { name, prices }
    }
}

type Stay = (String, u64);
type Trip = Vec<Stay>;
type Trips = Vec<Trip>;

fn get_trips_for_city(start: &City, cities: &BTreeSet<City>, day: u64) -> Trips {
    let mut trip = Trip::new();
    let price = start.prices[day as usize];
    match price {
        Some(p) => trip.push((start.clone().name, p)),
        None => return vec![],
    };

    let mut remaining_cities = cities.clone();
    remaining_cities.remove(start);

    if remaining_cities.is_empty() {
        return vec![trip];
    } else {
        let mut trips = Trips::new();
        for city in &remaining_cities {
            let remaining_trips = get_trips_for_city(city, &remaining_cities, day + 1);
            for remaining_trip in remaining_trips {
                let mut full_trip = trip.clone();
                full_trip.extend(remaining_trip);
                trips.push(full_trip);
            }
        }
        return trips;
    }
}

fn get_trips(cities: &BTreeSet<City>) -> Trips {
    cities
        .par_iter()
        .flat_map(|city| get_trips_for_city(city, &cities, 0))
        .collect()
}

fn generate_cities(names: Vec<String>) -> BTreeSet<City> {
    let mut cities = BTreeSet::new();
    let num_prices = names.len();
    for name in names {
        let mut prices = Vec::new();
        for _ in 0..num_prices {
            let no_free_room = rand::thread_rng().gen_range(0, 5);
            if no_free_room == 0 {
                prices.push(None);
            } else {
                prices.push(Some(rand::thread_rng().gen_range(50, 300)));
            }
        }
        cities.insert(City::new(name, prices));
    }
    cities
}

fn cost(trip: &Trip) -> u64 {
    let mut total = 0;
    for &(ref _city, price) in trip {
        total += price;
    }
    total
}

fn cheapest_trip(trips: Trips) -> Option<Trip> {
    let mut cheapest_trip = None;
    let mut min_cost = std::u64::MAX;
    for trip in trips {
        let cost = cost(&trip);
        if cost < min_cost {
            min_cost = cost;
            cheapest_trip = Some(trip);
        }
    }
    cheapest_trip
}

fn main() {
    let cities: BTreeSet<City> = generate_cities(vec![
        "New York".into(),
        "Duesseldorf".into(),
        "London".into(),
        "Paris".into(),
        "Istanbul".into(),
        "Shanghai".into(),
        "Sao Paolo".into(),
        "Kopenhagen".into(),
        "Munich".into(),
        "Tokio".into(),
        "Venice".into(),
        "Rome".into(),
        // "Stockholm".into(),
    ]);
    //println!("{:#?}", cities);
    println!("{:#?}", cheapest_trip(get_trips(&cities)));
}

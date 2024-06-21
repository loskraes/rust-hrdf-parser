use chrono::NaiveDateTime;
use rustc_hash::FxHashSet;

use crate::{models::Journey, storage::DataStorage};

#[derive(Debug, Clone)]
pub struct RouteSection {
    journey_id: Option<i32>,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    arrival_at: NaiveDateTime,
    duration: Option<i16>,
}

impl RouteSection {
    pub fn new(
        journey_id: Option<i32>,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        arrival_at: NaiveDateTime,
        duration: Option<i16>,
    ) -> Self {
        Self {
            journey_id,
            departure_stop_id,
            arrival_stop_id,
            arrival_at,
            duration,
        }
    }

    // Getters/Setters

    pub fn journey_id(&self) -> Option<i32> {
        self.journey_id
    }

    pub fn departure_stop_id(&self) -> i32 {
        self.departure_stop_id
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.arrival_stop_id
    }

    pub fn set_arrival_stop_id(&mut self, value: i32) {
        self.arrival_stop_id = value;
    }

    pub fn arrival_at(&self) -> NaiveDateTime {
        self.arrival_at
    }

    pub fn set_arrival_at(&mut self, value: NaiveDateTime) {
        self.arrival_at = value;
    }

    pub fn duration(&self) -> Option<i16> {
        self.duration
    }

    // Functions

    pub fn journey<'a>(&'a self, data_storage: &'a DataStorage) -> Option<&Journey> {
        self.journey_id.map(|id| data_storage.journeys().find(id))
    }
}

#[derive(Debug, Clone)]
pub struct Route {
    sections: Vec<RouteSection>,
    visited_stops: FxHashSet<i32>,
}

impl Route {
    pub fn new(sections: Vec<RouteSection>, visited_stops: FxHashSet<i32>) -> Self {
        Self {
            sections,
            visited_stops,
        }
    }

    // Getters/Setters

    pub fn sections(&self) -> &Vec<RouteSection> {
        &self.sections
    }

    pub fn visited_stops(&self) -> &FxHashSet<i32> {
        &self.visited_stops
    }

    // Functions

    pub fn last_section(&self) -> &RouteSection {
        // A route always contains at least one section.
        self.sections.last().unwrap()
    }

    pub fn last_section_mut(&mut self) -> &mut RouteSection {
        // A route always contains at least one section.
        self.sections.last_mut().unwrap()
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.last_section().arrival_stop_id()
    }

    pub fn arrival_at(&self) -> NaiveDateTime {
        self.last_section().arrival_at()
    }

    pub fn has_visited_any_stops(&self, stops: &FxHashSet<i32>) -> bool {
        !self.visited_stops.is_disjoint(stops)
    }

    pub fn sections_having_journey(&self) -> Vec<&RouteSection> {
        self.sections
            .iter()
            .filter(|section| section.journey_id().is_some())
            .collect()
    }

    pub fn count_connections(&self) -> usize {
        self.sections_having_journey().len()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RoutingAlgorithmMode {
    SolveFromDepartureStopToArrivalStop,
    SolveFromDepartureStopToReachableArrivalStops,
}

pub struct RoutingAlgorithmArgs {
    mode: RoutingAlgorithmMode,
    arrival_stop_id: Option<i32>,
    time_limit: Option<NaiveDateTime>,
}

impl RoutingAlgorithmArgs {
    pub fn new(
        mode: RoutingAlgorithmMode,
        arrival_stop_id: Option<i32>,
        time_limit: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            mode,
            arrival_stop_id,
            time_limit,
        }
    }

    pub fn solve_from_departure_stop_to_arrival_stop(arrival_stop_id: i32) -> Self {
        Self::new(
            RoutingAlgorithmMode::SolveFromDepartureStopToArrivalStop,
            Some(arrival_stop_id),
            None,
        )
    }

    pub fn create_solve_one_to_many(time_limit: NaiveDateTime) -> Self {
        Self::new(
            RoutingAlgorithmMode::SolveFromDepartureStopToReachableArrivalStops,
            None,
            Some(time_limit),
        )
    }

    // Getters/Setters

    pub fn mode(&self) -> RoutingAlgorithmMode {
        self.mode
    }

    /// Do not call this function if you are not sure that arrival_stop_id is not None.
    pub fn arrival_stop_id(&self) -> i32 {
        self.arrival_stop_id.unwrap()
    }

    /// Do not call this function if you are not sure that time_limit is not None.
    pub fn time_limit(&self) -> NaiveDateTime {
        self.time_limit.unwrap()
    }
}

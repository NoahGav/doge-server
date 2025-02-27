use std::fmt::Display;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[derive(Debug, Clone, Copy)]
pub enum When {
    EveryDay,                 // Every day
    EveryWeekday(Weekday),    // Every Wednesday
    SpecificDay(u32),         // 1st, 2nd, ..., 31st
    LastDayOfMonth,           // Last day of any month
    EveryTwoWeeks(NaiveDate), // Repeats every two weeks starting from a given date
    FederalPaycheck(u32),     // Adjusted payday for federal checks
}

impl When {
    pub fn matches(&self, date: NaiveDate) -> bool {
        match self {
            When::EveryDay => true,
            When::EveryWeekday(weekday) => date.weekday() == *weekday,
            When::SpecificDay(day) => date.day() == *day,
            When::LastDayOfMonth => {
                let next_day = date + Duration::days(1);
                next_day.month() != date.month() // If next day is in a different month, it's the last day
            }
            When::EveryTwoWeeks(start_date) => {
                let days_since_start = (date - *start_date).num_days();
                days_since_start >= 0 && days_since_start % 14 == 0
            }
            When::FederalPaycheck(day) => {
                let paycheck_date =
                    Self::calculate_federal_paycheck(date.year(), date.month(), *day, date);
                date == paycheck_date
            }
        }
    }

    /// Computes the actual payday based on the federal paycheck rule
    fn calculate_federal_paycheck(
        year: i32,
        month: u32,
        target_day: u32,
        current_date: NaiveDate,
    ) -> NaiveDate {
        // Get the next payday
        let mut payday = NaiveDate::from_ymd_opt(year, month, target_day).unwrap();

        // If the calculated payday is not after the current date, move to the next month
        if payday <= current_date {
            if month == 12 {
                payday = NaiveDate::from_ymd_opt(year + 1, 1, target_day).unwrap();
            } else {
                payday = NaiveDate::from_ymd_opt(year, month + 1, target_day).unwrap();
            }
        }

        // Move backwards to the first business day (not Sat/Sun)
        while payday.weekday() == Weekday::Sat || payday.weekday() == Weekday::Sun {
            payday -= Duration::days(1);
        }

        // Move back two additional business days
        let mut count = 0;
        while count < 2 {
            payday -= Duration::days(1);
            if payday.weekday() != Weekday::Sat && payday.weekday() != Weekday::Sun {
                count += 1;
            }
        }

        payday
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Money(i32);

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0 as f32 / 1000.0)
    }
}

impl Money {
    pub fn dollars(dollars: f32) -> Self {
        Self((dollars * 1000.0) as i32)
    }

    pub fn add(&mut self, other: Money) {
        self.0 += other.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MoneyFlow {
    pub amount: Money,
    pub when: When,
}

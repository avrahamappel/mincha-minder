# Mincha Minder

Don't forget Mincha again!

Enter the time:

- [ ] Fixed Time (time)
- [ ] Baefore / After sunset (time)

Recurs daily Sunday through Thursday.

Ability to override single days

Add travel time / notification time

## Tech

- ical (https://docs.rs/icalendar/latest/icalendar/)
    - Sunset data: https://crates.io/crates/sunrise
    - Generate 1 year of events
- caldav server
    - See https://en.wikipedia.org/wiki/CalDAV
- rocket.rs
- user accounts

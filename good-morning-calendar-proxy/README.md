# Good Morning Calendar Proxy

ESP has little memory, `ical` crate does not support
streaming events, so an alternative solution is needed.

Serving a single endpoint, which takes an ical url (POST),
fetches it's events and returns the events happening today.
(Watch server time zone.)

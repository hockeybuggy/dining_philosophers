# Dining philosophers

This is effectively a exact copy of the code in the excellent [rust book's "Dining Philosophers"][DINING_PHILOSOPHERS] example with two changes.

1. `std::time::sleep_ms` has been deprecated since the version of the book with
   this example was released. Instead this uses `std::time::Duration::from_millis`.

2. Added an extra sleep between grabbing forks. Without it I couldn't replicate
   a deadlock.

[DINING_PHILOSOPHERS]: http://doc.rust-lang.org/1.2.0/book/dining-philosophers.html

# Hachiya

Hachiya is an attempt to provide a mechanism for natively modifying a Bevy
application. Meaning, as an alternative to incorporating a scripting engine, one
may "hook" their game into source-available and/or pre-built Rust libraries.
This crate is a work-in-progress (and quite nascent at that). API changes and
general instability should therefore be expected throughout the course of its
development.

> [!WARNING]
> This crate is not ready for use. A minimum viable product should hopefully be
> available in the coming weeks once the `Registrar` API has been ironed-out.
> Ideally, hot-reloading would also be implemented.


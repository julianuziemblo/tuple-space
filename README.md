# Tuple Space
An implementation of a custom tuple space protocol - a logical space for sharing data between programs, written in Rust. This project is <u>still under developement</u>, so many things may change.

The implementation consists of:
- `tuple_space`: the tuple space API. Contains useful functions for making and sending tuples between the participants.
- `server`: the middleware of the tuple space. Stores, maintains the tuple space, and performs operations commisioned by clients.
- `client`: example client

# System specification
All of the tuple space's features have been thoroughly described in [this article](https://github.com/julianuziemblo/tuple-space/files/15044712/Julian_Uziemblo_Przestrzen_krotek_Linda_-_realizacja_projektu_Warszawa_2024.pdf) (in Polish). 


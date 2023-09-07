# Stasis

State channels for layer 2s.

## What?

This is an implementation of Stasis in Rust for arbitrum stylus. Stasis is a generic state channel protocol for off chain communication of settlements between distrusting parties.

## Lite-spec

Stasis is designed to lock up user funds and release them when a settlement is triggered. For this we first need a funding transaction:

### Funding transaction

T

# Stasis

State channels for layer 2s.

## What?

This is an implementation of Stasis in Rust for arbitrum stylus. Stasis is a generic state channel protocol for off chain communication of settlements between distrusting parties.

## Lite-spec

Stasis is designed to lock up user funds and release them when a settlement is triggered. For this we first need a funding transaction:

### Funding transaction

The initial funding transaction contains the addresses of the parties, token amounts, token addresses and signature made by both parties to confirm the opening of a channel. After a channel is opened, the parties can start communicating via off chain messages, known as commitment transactions.

### Commitment transactions

Contrary to the Lightning Network, commitment transactions do not represent transactions that can be broadcast to the network and do not include a time lock. A commitment transaction includes the following data:
```
addresses and balances of counterparties:
- balance0(u256)
- balance1 (u256)
commitment tx nonce:
- nonce (u32)
signatures of the specified parameters hashed together:
- sig0 (bytes32[2])
- sig1 (bytes32[2])
```

### Settlement

Stasis is fully trustless. Channels can be settled in 2 modes: cooperative and unilateral.

#### Cooperative close

A cooperative close means that both are cooperating during settlement. To initate a cooperative close, the parties must sign a commitment transaction with a nonce of u32::MAX. The commitment tx is then submited on chain and final balances are settled.

#### Unilateral close

In case a party is not responsive, the other party can initiate a unilateral close. The user would take the commitment transaction with the highest nonce and submit it to the chain. The commitment is then recorded along with a timestamp.  

Due to the potential for fraud, we add a 7 day grace period to allow the other party to respond. If the other party can respond with a higher nonce transaction, the party submitting the unilateral close loses all of their funds which are then transfered to the party submitting the justice transaction.

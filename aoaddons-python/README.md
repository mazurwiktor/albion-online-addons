pyaoaddons
==========

# API reference

## subscribe

Args: Callable[dict]

Registers callback which is going to be called on each game event

## initialize

Initialize logging and packet sniffing.

After invocation library starts to listen to game packets.

> **Note**: logs are placed in `aoaddons.log` file

Returns `InitializationResult`


## Available game events

- MainPlayerAppeared
- PlayerAppeared
- PlayerAppeared
- DamageDone
- HealthReceived
- ZoneChange
- EnterCombat
- LeaveCombat
- UpdateFame
- UpdateItems
- UpdateParty
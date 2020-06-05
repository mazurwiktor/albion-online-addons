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

## item_category_mapping

Dictionary containing human readable category names for coded items.

E.g 
```python
{
    "T6_POTION_HEAL": "potion"
}
```

## localization_mapping

Dictionary containing human readable names for coded items.

E.g 
```python
{
    "T8_ARTEFACT_2H_CURSEDSTAFF_MORGANA": "Elder's Bloodforged Catalyst"
}
```

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
# Kingdom Kards

Kingdom Kards is a card game that can be played with a standard deck of playing 
cards. You can find a full list of rules in [Rules.md](Rules.md)

## Usage
When `kingdom-kards` is first started, the user will be prompted to either
connect "Host a Game" or "Join a game".

```
Starting Kingdom Kards...

1. Host a game
2. Join a game

Choose an option: 
```

If the user chooses to host a game, then a locally hosted server will 
started with a join code. This join code can be used by other players on the 
same network to connect to the locally hosted server. 

```
1. Host a game
2. Connect to game

Choose an option [1 or 2]: 1
Starting server with join code: 1234
```

For players on the same network to join, they must simply start up the 
`kingdom-kards` application and choose to start a game instead of host one. 
Afterwards, they will be prompted to enter a join code. 

```
1. Host a game
2. Connect to a game

Choose an option: 2
```

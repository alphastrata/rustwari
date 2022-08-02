```
                         __   ^
|-| ! |\/| /\ \ /\ / /\  |_!  |
! ! | |  !/  \ \  / /  \ |\   !
                         | \    -気象衛星
rustwari

```

## Todo:

- [] Implement a waitgroup kinda thing to rate limit the number of concurrent downloads happening (not everyone can use the ulimit hack)
- [] This app has been going for some time now, it maybe contains a few dozens of versions worth of 'idiomatic rust' 
- [] The user_config functionality needs some work, could contain more things like: Lenght of the pauses, Screen dimensions
- [] Screen dimensions could be used to do less work (fetch tiles at lower resolutions for example).
- [] Can more of those for{for{}} loops be turned into lovely rusty .iter()s?
- [] Can some of these async functions be sync?
- [] It makes good sense for the scraping functionality to by async, but not really for the concatenation to be so, maybe change that?
- [] There's still a bit of .unwrap() or it's buddies going on. Is there a nicer way to deal with options than this ...ok()? crap?
- [] Sometimes, the app fails and you get an entire disc worth of "no image found" tiles, use dsssim or something to work out when this has happened and revert to previously known good timestamp's entry. (maybe even check the entry is good BEFORE setting it?)
- [] Installing apparently for windows really sucks, can you bundle the open cv .dlls?
- [] Implement a silent mode -q (like cargo :p) which can surpress stdout and make this less intrusive to run in terminals you're planing on continuing to use.
- [] 
- []
- []


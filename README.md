## webgrep: the simplest way to surf the web
webgrep is the fastest way to read the web right from your terminal, with all the features of regular grep.
### how to use:
download it with cargo, then use it like any other unix command:
#### flags:
- `-u/--url` base url to search from
- `-s/--search` OPTIONAL, search query (REGEX supported)
- `-r/--recursive` OPTIONAL, iterate through/search all links on the base url for a set depth (see below), provide a value for depth (how many branches of the link tree to explore)
- `-p/--pathcontains` OPTIONAL, plaintext string that must be in a path for it to be explored (useful to prevent going to going wandering)
- `-o/--samehost` OPTIONAL, only explore webpages on the same host (ie same domain)

### bugs/contributing
please report any bugs you encounter on the 'issues' page. we welcome any contributions, just make a pr!

### coming soon!
- support for adding a delay in between requests
- tests
- more advanced tree creation method
- more!

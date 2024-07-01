## webgrep: the simplest way to surf the web
### how to use:
webgrep is currently still in early development, but once we get to a 'finished-enough' point we will publish it on crates.io. until then, you can build it yourself by cloning the repo.
#### flags:
- `-u/--url` base url to search from
- `-s/--search` search query (REGEX supported)
- `-r/--recursive` iterate through/search all links on the base url for a set depth (see below)
- `-d/--depth` integer, defaults to 1, how many times to recurse
- `-i/--interval` how long to wait before making requests to webpages (defaults to 1 second)

### bugs/contributing
please report any bugs you encounter on the 'issues' page. we welcome any contributions, just make a pr!

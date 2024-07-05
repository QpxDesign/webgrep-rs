## webgrep: the simplest way to surf the web
webgrep is the fastest way to read the web right from your terminal, with all the features of regular grep. webgrep supports full browser webpage rendering using headless-chrome, as well as more basic http request/html parsing. it can even read pdfs. webgrep can render javascript, search links trees recursively, read webpages with regex, and more. (note i know the order for source and search is reversed from original grep, but its so much better trust me)
### how to use:
download it with cargo, then use it like any other unix command:
`cargo install webgrep`
`webgrep <search (url)> <search or regex expression>`
recommended: use [headless-chrome](https://github.com/rust-headless-chrome/rust-headless-chrome) (see how below), since many sites only work with javascript/other browser-required features.
to use headless-chrome with webgrep, either use the `-c/--chrome` flag, or set the following enviromental variable: (note, using chrome is much slower [~2sec/request vs .25sec/request])
`export WEBGREP_ALWAYS_USE_CHROME=1`
#### flags:
- `-r/--recursive` OPTIONAL, iterate through/search all links on the base url for a set depth (see below), provide a value for depth (how many branches of the link tree to explore)
- `-p/--pathcontains` OPTIONAL, plaintext string that must be in a path for it to be explored (useful to prevent recursive search from going too far astray)
- `-o/--samehost` OPTIONAL, only explore webpages on the same host (ie same domain)
- `-c/--chrome` OPTIONAL, use chrome instead of just basic http request (needed for sites with js)
### Examples:
recursively search through a college course catalog for a keyword (case insensitive):

`webgrep https://https://oxy.smartcatalogiq.com/en/2021-2022/catalog/course-descriptions/ "(?i)ethical" --samehost -p course-descriptions`

read a react.js webpage using headless chrome:

`webgrep https://quinnpatwardhan.com/aboutme frisb -c` (or, with env variable set): `webgrep https://quinnpatwardhan.com/aboutme frisb`

### bugs/contributing
please report any bugs you encounter on the 'issues' page. we welcome any contributions, just make a pr!

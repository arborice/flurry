> init procedure
> -
> - missing dotfiles write fn call on first run
> - missing shell completions *(**panics** w/ subcommands from clap bug, will have to process app by app)*

<br/>

> review unsafe dep usage
> -
> - only used after valid encoding established <del> percent-encoding uses **unsafe** for *what*?
> - termion uses **unsafe** *probably for good reason* ... other backends available for tui, although not as well maintained, maybe worth review
> - <del/> which uses **libc**, but is very mature and covering edge cases to replace may not be worth it
> - REMOVED <del/> colored has 2 deps, both **unsafe**. Not necessary

<br/>

> refactor to condense?
> -
> - codebase is highly readable and expandable, but a bit sparse
> - make tui optional?
> - make media shuffler optional
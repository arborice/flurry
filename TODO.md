> shrink deps tree
> -
> - anyhow is nice, but a crate-wide error type could achieve the same convenience
> - REMOVED <del/> once_cell isn't needed. webbrowser aliases do not need to be static
> - REMOVED <del/> strum is similarly nice, but usage has been cut to impl'ing AsRef<str> - not needed

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
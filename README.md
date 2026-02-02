# ralcapi

another extremely useless backend server that does simple arithmetic (the other one is [calcapi](https://github.com/faretek1/calcapi))

e.g. [`https://ra.faretek.dev/add/1/2`](https://ra.faretek.dev/add/1/2) gives `1 + 2 = 3` as a response.

The response isn't even json. it's just a string with the inputs and the output in the format `{a} {op} {b} = {result}`

so yeah. this is my first actual project in rust which is cool i guess. its quite useless though

It works with floats and ints. `1 / 0` gives `inf` because of how go and IEEE 754 works.

## available ops

`add`: `+`
`sub`: `-`
`mul`: `*`
`div`: `/`

this one is async unlike the golang one which is cool i guess

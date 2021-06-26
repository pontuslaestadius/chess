let input = require("./chessdotcom_openings.json")

console.log(JSON.stringify(input.map(x => {
    delete x.id
    delete x.u
    delete x.c
    delete x.f
    return x
}), null, 4))

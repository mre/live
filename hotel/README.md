You and your friends have an awesome plan: visit 10 cities in 10 days.

For each city you found a cool hotel on trivago and
you've noted down the hotel price per night. (Let's ignore all other costs right now.)
This is the data you collected (in JSON format):

```
[
    {
        "name": "Duesseldorf",
        "prices": [
            "192",
            "190",
            "205",
            "",
            "151",
            "78",
            "74",
            "211",
            "189",
            ""
        ]
    },
    {
        "name": "Istanbul",
        "prices": [
            "",
            "94",
            "217",
            "",
            "245",
            "188",
            "175",
            "116",
            "207",
            "231"
        ]
    },
    {
        "name": "Kopenhagen",
        "prices": [
            "61",
            "",
            "200",
            "",
            "172",
            "213",
            "",
            "117",
            "247",
            "130"
        ]
    },
    {
        "name": "London",
        "prices": [
            "99",
            "64",
            "",
            "229",
            "124",
            "150",
            "284",
            "140",
            "237",
            ""
        ]
    },
    {
        "name": "Munich",
        "prices": [
            "91",
            "248",
            "219",
            "252",
            "163",
            "87",
            "92",
            "156",
            "218",
            "298"
        ]
    },
    {
        "name": "New York",
        "prices": [
            "",
            "",
            "165",
            "",
            "109",
            "122",
            "169",
            "84",
            "",
            "141"
        ]
    },
    {
        "name": "Paris",
        "prices": [
            "281",
            "217",
            "174",
            "216",
            "62",
            "149",
            "146",
            "",
            "188",
            ""
        ]
    },
    {
        "name": "Sao Paolo",
        "prices": [
            "159",
            "",
            "178",
            "270",
            "198",
            "269",
            "111",
            "",
            "151",
            "290"
        ]
    },
    {
        "name": "Shanghai",
        "prices": [
            "68",
            "119",
            "233",
            "131",
            "",
            "176",
            "54",
            "",
            "",
            "88"
        ]
    },
    {
        "name": "Tokio",
        "prices": [
            "",
            "73",
            "70",
            "250",
            "158",
            "125",
            "227",
            "177",
            "56",
            "232"
        ]
    }
]
```

The JSON document consists of a list of city objects.
Each city object has a "name" field, which contains the name of the city (e.g. "New York").
Also each object contains a list of prices, ordered by day of travel, so the first entry in the list would be the price for the first night,
the second entry would be the price for the second night and so on.
If the entry is empty, then for this night the hotel is considered unavailable.

The task is to find the cheapest trip based on the given JSON data when you stay exactly one night in each hotel.

Participate by filling out the following information and get the chance to win a *fabulous* prize.

Total cost for cheapest trip: ____________  
Last city you will visit on the trip: ____________  
Your name: __________  

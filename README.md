# Or Probability Calculator

A quick CLI project to dabble in Rust & calculate multiple probabilities. 

## Example

Suppose you have a coin with 50% odds of landing heads. If you observe 2 coin flips, the probability that at least one of them is heads is their sum minus the product of both occurring: `0.5 + 0.5 - (0.5 * 0.5)`.

We can write this as: `P(A or B) = P(A) + P(B) - (P(A) * P(B))`

But what if we want to calculate the odds of getting at least 1 heads for 3 flips? We could get this recursively if we already know `P(A or B)`:

`P(A or B or C) = P(A or B) + P(C) - (P(A or B) - P(C))`

We can use this pattern to get probabilities for any number of events with the same individual probabilities.

This CLI takes arguments for the probability of each event (`p`) and the number of events that will be observed (`c`). It recursively figures out the total probability of at least one of the `c` events occurring:

```
fn calculate_probability(p: f32, c: u32) -> f32 {
    match c {
        0 => 0.0,
        1 => p,
        2 => p + p - (p * p),
        _ => {
            let prev = calculate_probability(p, c - 1);
            prev + p - (prev * p)
        },
    }
}
```


## How to use

```
USAGE:
    cargo run -- [OPTIONS]

OPTIONS:
    -p, --probability <PROBABILITY>    Probability of each event occurring
    -c, --count <COUNT>                Number of times each event could happen
```

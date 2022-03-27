# Brasso
Me trying Rust out.

#### What it is like to write.
Great.

#### Early thought on performance.
I have only written a solution to one simple problem, the luck be a landlord game high score. This is important because maybe this is a bad use case for Rust.

Now we have caveatted our words. Rust is slow. And it looks like it is very susceptible to small changes ruining the optimisation. I don't know if C would share this fragility, but I suspect not.  

Specifically I had a while loop, and I wanted it to be a Do While loop, so I had something like. 

```
function();
while some_test_fun() {
    function();
}
```

Changing the code to the below added 25% to the run time of the loop. That's bad. My guess is that it stopped inlining the `function` function.

```
a_different_fun();
while some_test_fun() {
    function();
}
```
A cast to usize like the below added 5% to 15% to a for loop with a couple of array indexing. 

```
if i > 0 && i < width && j > 0 && j < height {
    result *= array[grid[j as usize][i as usize]psome_idex];
}
```

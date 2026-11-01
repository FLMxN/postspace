<div align="center">

# Postspace

Welcome to politics, ***sibling***.

</div>

**Postspace** is a terminal quiz that sketches your political profile through 80 shuffled questions.

Answer with yes, no, or neutral; your answers are grouped across five axes: **Market**, **Liberty**, **Progressivism**, **Transhumanism**, and **Democracy**.

At the end, **Postspace** prints your scores, highlights ideological labels that match your profile, and lists the three closest figures by euclidean vector similarity. It is a playful comparison tool, not a scientific assessment or a definitive label.

Run with `cargo run`. Each result also generates a 3:4 PNG card in `cards/`.

To skip the quiz and use your own coordinates, pass five values in **Market**, **Liberty**, **Progressivism**, **Transhumanism**, **Democracy** order. Each value must be between `-1` and `1`:

```sh
cargo run -- --vector 0.25 -0.50 0.10 0.75 0.40
```

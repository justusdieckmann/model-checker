# LTL Model Checker

This is a small project in which one can create Kripke-Structures and check them with LTL formulas.

The current release can be viewed at https://justusdieckmann.github.io/model-checker/.

The UI is just enough to get the job done, since the focus lies on the model checking, which is in a separate crate in the `lib` subdirectory.

### Explanation

The LTL Syntax consists of Until `U`, Next `X`, Release `R`, Weak Until `W`, Future `F`, Generally `G`, And `&`,  Or `|`, Not `!`, True `1`, False `0`, parenthesis and atomic propositions, which begin with a lower-case letter.

Kripke structure states can be created with a click, connected with a drag (only visible after mouse released), deleted with `del` and made a starting state with a double click.

When a state is selected (blue), its atomic propositions can be set via the text field, separated by `,`.
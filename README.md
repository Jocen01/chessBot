# Chessbot

## Play against the bot
To play against the bot you can either play it on lichess [here](https://lichess.org/@/Alpha-Beta-Bot) Currently is isn't live all the time in wich case there is an .exe version wich implements the uci protocol in _engine_versions_. This version can be plugged in to most chess GUIs without any modifications.


## Techniques
### Movegeneration
Initially the movegeneration generate all psudo leagal moves then played them to see which were actually leagal. To speed up the generation everything is now based on bitboards and the movegeneration first generetes all pinns and if its check or double check to see what pices are alowed to move. The pinned pices can then only move along the pin-ray. To speed up the movegeneration for siding pices [magic bitboards](https://www.chessprogramming.org/Magic_Bitboards) are utalized. By multiplying the blockers by a _magic_ number it can then be bitshifted to index in an array. With the current generation it speeds up the generation by approximately 4%. 

### Evaluation
The current evaluation is very simple and mostly relies on the [pesto](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function) position tables. It includes position scores and pice values for both middlegame and endgame. To get a bit better pawn evaluation bonus scores are added to pastpawns, rooks on open files and deduction for doubbled pawns.

### Search
The search uses basic alpha-beta pruning algoritm. To further increece the pruning iterative deepening along with window search is utalized. To take care of transpositions a [transposition table](https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm) is used and doubbel up to keep track of the [principal variation](https://www.chessprogramming.org/Principal_Variation_Search). This allows for faster conversion of alpha and beta. Furthermore, nullwindow search is used to prune earlier. For moveordering only [MVV-LVA](https://www.chessprogramming.org/MVV-LVA) (Most Valuable Victim - Least Valuable Aggressor) is used. This have a huge impact on the number of nodes searched in each layer. In some situations this can increece the depth by 5 ply or more. To reduce the impact of the horizon problem a [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search) is used to reduce the likelyhood of a capture just beyond the horizon. 
# Samson Chess Engine

Step 1:
Get legal move generation, PGN/FEN parsing and a reasonable on disk storage system for games. Base this on stockfish data structures.

Step 2: 
Integrate into lakinwecker/delila

Step 3:
Continue porting Stockfish and Scoutfish ideas/algorithms.


Step 4:
Adapt to focus on teaching humans

This engine is not going to be focused on having the highest possible
elo. Rather, it will be focused on helping to train players. The intent is to
develop efficient identification and explanation of tactical and positional
motifs. So that we can easily process a large set of games quickly, identifying
various tactical opportunities as well as to map up each of those tactical
opportunities to the appropriate motifs.


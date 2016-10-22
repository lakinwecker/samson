# LoneWolf Chess Engine

This engine is not going to be focused on having the highest possible
elo. Rather, it will be focused on helping to train players. The intent is to
develop efficient identification and explanation of tactical and positional
motifs. So that we can easily process a large set of games quickly, identifying
various tactical opportunities as well as to map up each of those tactical
opportunities to the appropriate motifs.

The intent is to integrate this into the LoneWolf Chess Training program
which will help train players via this same concept.

# Process / Goals
1. Port python-chess into rust.
2. Include full PGN parsing and game move validation / generation.
3. Include a simple positional evaluation function
4. Implement detection of motifs in positions.


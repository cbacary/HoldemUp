# How the algorithm works
The algorithm actually has some inspiration from a minimiax search.

In essence, the function `calculate_odds` is given a depth (`cards_to_play`),
and a deck. The algorithm starts at the first card in the deck, plays it on 
the board, removes it from the deck, and recalls `calculate_odds` passing
in a new depth and a modified deck. This process is continued until a depth of 0
is reached.

Once a depth of zero is reached the current hand is calculated, and passed back to the 
prior node. The prior node (depth=1) sums up the chances of each hand of the child node
occuring and adds it to a hashmap where the hand is the key, and the value is the chance.
Once this node has completed all cards in the deck it is returned to depth=2 where this occurs 
all the way back up to root node.

# A better solution
This solution isn't too bad. 
Calculating 3 cards to play with 2 cards in player's deck the result is something like
(50 * 49 * 48) operations. Unfortunately, with the use of the semi-odd and unorthadox hashmap
usage and slow hand callculation what may seem to be few operations for a computer is slowed 
down dramatically. A faster way to do this is described here 
https://www.pokerstrategy.com/strategy/various-poker/texas-holdem-probabilities/
It seems once you calculate the number of outs for a given hand, it becomes 
relatively trivial to calculate the actual odds of that hand occuring, 
but the calculation used seems to change quite a bit depending on what you are calculating.
My solution seemed to be the quckest for development. If more time were to be invested into this 
project, this option would be explored, but only after increasing the efficiency of my get
`get_hand` function, and possibly the removal of the hashmap because hashmaps while fast for retrieval
are very slow when it comes to something like this.
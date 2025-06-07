class Suit(enumerate):
    Clubs
    Hearts
    Spades
    Diamonds

class Enhancement(enumerate):
    Bonus
    Mult
    Wild
    Glass
    Steel
    Stone
    Gold
    Lucky

class Edition(enumerate):
    Foil
    Holographic
    Polychrome
    Negative
class Seal(enumerate):
    Red
    Blue
    Purple
    Gold
    
    
class State:
    def __init__(self):
        self.round = 0
        self.ante = 0
        self.money = 4
        self.hands_played = 0
        self.cards_discarded = 0
        self.cards_played = 0
        self.rounds_skipped = 0
        self.cards_added = 0
        self.tarots_used = 0

        self.mult = 0
        self.chips = 0
        self.score = 0

        self.jokers = []
        self.deck:list[Card] = [] # fi
        self.hand: list[Card] = []
        self.boss_blind = None

        self.consumables = []

    def play_hand(self, hand: list[Card]):
        self.set_base(hand)
        self.apply_boss(hand)
        self.on_play(hand)
        for card in hand:
            card.score(self)
        held = self.remaining_held(hand)
        for card in held:
            card.on_hold(self)
        for joker in self.jokers:
            joker.independent(state, hand)
        self.apply_consumable_effects()
        self.end_score()
        
    def set_base(self, hand):
        pass

    def remaining_held(self) -> list[Card]:
        pass            

    def on_card_score(self, card: Card):
        for joker in self.jokers:
            joker.on_card_score(self, card)
    def on_card_held(self, card):
        for joker in self.jokers:
            joker.on_card_held(card)
        

class Card:    
    def __init__(self, rank: int, suit: Suit, enhancement: Enhancement = None, edition: Edition = None):
        self.rank = rank
        self.suit = suit
        self.base_chips = rank if 2 <= rank <=10 else [10, 10, 10, 11][rank - 11]
        self.enhancement = enhancement
        self.edition = edition
        self.seal = None


    def score(self, state: State):
        state.chips += self.base_chips
        self.apply_score_enhancement(state)
        self.apply_score_seal(state)
        self.apply_score_edition(state)
        state.on_card_score(self)
        

    def on_hold(self, state):
        self.apply_held_enhancement(state)
        state.on_card_held(self)


        
        
class OnCardScore:
    def apply(self, card: Card, state):
        return card.on_card_score(state)

def apply_effect(state, effect):
    
def play(state):
    pass
    
def action():
    pass
def game_loop(state):
    action = action()
    if action == "play":
        play(state)
        score()
    else:
        discard()
    draw()
    return state



        

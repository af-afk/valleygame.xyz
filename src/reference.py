#!/usr/bin/env python3

from hypothesis import given
from hypothesis.strategies import integers

class GameStarted(Exception): pass
class GameOver(Exception): pass

def merge_record(a, b):
    return {k: a.get(k) + b.get(k) for k in a.keys() | b.keys()}

class Game:
    """
    The current round the game is on.
    """
    round = 0

    """
    Simple knowledge of who has the right to participate in this
    game.
    """
    players = {}

    """
    The predictions made by users at each step of the game for
    five rounds. The key for each round is their unique id. Stored
    privately from each player's perspective.
    """
    player_predictions = []

    """
    The amount that every user invested to play the game.
    """
    fixed_buyin = 5

    """
    A fixed size circular buffer for messages. Maybe 200 items long.
    """
    messages = []

    """
    Points awarded for each bip based on how close the user is to
    the mean (1 bip = 0.1%).
    """
    bip_points = 100

    def __init__(self, rounds=5):
        self.player_predictions = [{} for _ in range(rounds)]

    def calc_points(self, avg, p):
        return (abs(avg - p) * (self.bip_points / avg))

    def buyin(self, user):
        """
        Buy in an amount, only possible if the game is the first round.
        """
        if self.round > 0: raise GameStarted()
        self.players[user] = True

    def predict(self, user, value):
        """
        Lock in the prediction for this round. Bump the round if we hit
        the limit of the participants right now.
        """
        if self.round == len(self.player_predictions): raise GameOver()
        self.player_predictions[self.round][user] = value
        if len(self.player_predictions[self.round]) == len(self.players):
            self.round += 1

    def proximity_points(self):
        """
        Get the points for each user at the current round.
        """
        acc = {}
        for predictions in self.player_predictions[:self.round]:
            mean = sum(predictions.values()) / len(predictions)
            for k, p in predictions.items():
                acc[k] = acc.get(k, 0) + self.calc_points(mean, p)
        return acc

    def early_points(self):
        """
        Get the points the user is owed for how close they were at the
        beginning to the end consensus.
        """
        # Short circuit if the game isn't over yet.
        if self.round < len(self.player_predictions):
            return {k: 0 for k in self.players.keys()}
        first_predictions = self.player_predictions[0]
        final_predictions = self.player_predictions[self.round-1].values()
        m = sum(final_predictions) / len(final_predictions)
        return {k: self.calc_points(m, first_predictions[k]) for k in self.players.keys()}

    def points(self):
        """
        Get the points for each user at the current round.
        """
        return merge_record(self.proximity_points(), self.early_points())

    def allocate(self):
        """
        Return the funds each user should receive as a result of the game.
        """
        available = self.fixed_buyin * len(self.players)
        p = self.points()
        all_points = sum(p.values())
        return {k: available * (v / all_points) for k, v in p.items()}

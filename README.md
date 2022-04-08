# 128H-Final
### Group 22 ###

Members: Warren Li (warrenl2), Alan Yao (ay15), David Yates (djyates2), Armin Rafieyan (arafi4)

Project Description:
This repository implements the classical game of battleship using Rust.

The battleship game will involve a player playing against the computer. Each side will get 5 ships to place either horizontally or vertically on a 10 by 10 board. Each turn they can choose one square to attack. Additionally, if the square they hit is occupied by a ship they get an additional turn. If all squares a ship occupies is attacked, it is considered to be sunk. The first side to sink all of the enemy’s ships will be declared the winner of the game. The computer will randomly attack unless a player’s ship is hit, then it will try to hit near the location of where the ship was hit to try and sink it. If the ship is sunk, the computer will go back to random movements. The player will input their own moves.

System Overview:
Our game will follow an object-oriented design- we will represent different attributes of the game as structs (Game, Board, Battleship, Player, etc.) and enums. The positions of the board can be represented as enums that show whether the space is occupied or not (empty, occupied, missed, or hit), and a 2d vector of these enums can be used to make the entire board. Most likely the enums and structs will be the first components of the program to be created. Then, we will work on the computer and how it plays, making sure it randomly targets until hitting a ship, then hitting near the ship to pursue a sink. Around the same time we will be implementing user inputs which will attack the computer’s board.

Each action of the game will be executed by calling its corresponding function (i.e. a function that fires a missile at a certain space).

At the end, if time allows, we would like to implement a server component that’ll allow two players to oppose each other in the game. However, this may prove to be too difficult, so we are only keeping it as a potential extra to introduce.

Possible Challenges:
Implementing Graphical Interface
Computer Decision Algorithm
Creating a multiplayer game mode (if we have time)

References:
https://en.wikipedia.org/wiki/Battleship_(game)

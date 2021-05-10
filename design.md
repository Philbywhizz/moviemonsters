# Movie Monsters

___"Simcity meets Godzilla"___

A game set in the early days of movies and cinema - where cheesy B-grade horror and disaster movies ruled.

This particular movie is about a monster and a city, however instead of playing as the _good guy_, you get to play the monster.

How would you play your monster? Do you cause lots of destruction and chaos, or are you just wanting a bite to eat? It isn't going to be easy to make friends when you have the whole city against you!

## Introduction

This design document is more of a 'brain dump' of ideas, and not an exact outline on what the game will become. For exact details for each version of the game see the [scope](scope.md) file

## Targeted platforms
Movie Monsters is currently being written using the rust programming language.  This allows the game to be targeted to multiple platforms. It is currently being developed on the Mac, it can be exported easily for the other platforms such as Windows and Linux.  Mobile devices are currently out of scope.

## Visual Style
2D, top down in a similar style to the old PC game of Grand Theft Auto. The theme should be based on a time when these old b-grade horror and disaster movies were made.

## Audio Style
The audio should be in the style of b-grade horror movies or films with cheesy sound effects.

# Starting Out

## Game start – Main Menu – Character Selection/Creation
The main menu should look like you have just walked into the front of a cinema, with a ticket booth, movie posters, popcorn for sale, etc. Each part of the scene represents a different menu option. There should also be a 'now showing' display for the current running configured movie.

### Ticket Booth
Ticket booth - Here you can determine the parameters of the Movie. There are 3 selections with the now showing: Monster type, City Location and Theme type. Each selection can be changed to set the configuration of the movie. Once you exit the ticket booth, the 'Now showing' displays should reflect the type of movie that is on display

### Watch Movie
Enter the cinema - Starts a new game. The monster type, city and theme of the game is determined by the 'now showing' posters.  A mock movie is shown to introduce the current configured game to the player. (Think like a star wars opening credit scene where the story intro is described). It also describes how the game should be played and the possible end/win conditions. Once this is watched (or skipped) the game will start.

### Box Office
Trophy cabinet - Lists high scores and achievements that have been done.

### Managers Office
Game Settings - allows you to set general game options and settings such as UI, difficulty, etc.

### Quit
This points to the exit and quits the game.

# UI

## In-Game HUD & Menus
TODO:

## Graphic sizes restrictions and guidelines (in pixels):
These values are just a rough guide and may change during the course during development. It is mainly used to indicate the relative size of objects in the game.
- Monsters = 64x64
- Scenery/tile = 128x128
- Vehicles = 32x24
- Actors = 8x8
- Props = 4x4

# Gameplay

## Mechanics
TODO:

## Controls
Standard W,A,S,D controls to move the monster. <SPACE> to use the monster's attack. <CTRL> will use the monster's special attack or command.

## Modes
TODO:

# Game Object Descriptions

## The Monster (you)
The Monster is the player character. There are different types of monster, each with their own abilities, both positive and negative. This is selected at the ticket booth before the game begins.

### Giant Robot:
The robot is several stories in height, and has a laser weapon that can vaporize anything.
- Laser beam for range attack
- Does not require food.
- Very strong but doesn't heal to damage is permanent.

### Blob:
The blob has no form or shape. The more it absorbs, the bigger and slower it becomes.
- Destroys anything it touches
- No range attack
- Slow in moving, gets slower as it gets bigger
- Requires food for energy
- Water reduces the size

### Dinosaur
The dinosaur is a large lizard creature. It is slow and heavy
- Bite attack. Rear attack with sweeping tail and damages things on the side when turning.
- No range attack
- Requires food for energy

### Wasp
The wasp is a super giant flying monster
- Reverse attach sting
- Can fly over buildings and water
- Requires food for energy

### Ant
The ant is a very large monster, very strong.
- Biting attack
- Range attack lifts up items to throw
- Leaves a trail of pheromones
- Requires food for energy

### Spider
The spider is a very fast and nimble monster. It is also very large.
- Bite attack. Stab attack (with multiple legs)
- Range attach by shooting web to immobilize.
- Requires food for energy.
- Can climb over buildings.

## Actors
Actors are the people in the game (NPCs). They wander around the movie set. Depending on the type of actor they would react differently to the player's monster.

### Civilian:
The civilian actor is just your every day person going about their day. They will avoid your monster as much as possible.

### Emergency:
The emergency actor will try and protect civilians by herding them to safety (Police), healing or helping injured civilians (Ambulance) or put our fires (Fire). They will avoid your monster as much as possible, however will try and help civilians first.

### Army:
The army actors will actively try and attack your monster using what resources are available (such as vehicles and weapons). They shoot bullets, rockets and other ammo at the monster to try and kill you.

### Scientists:
The Scientist actors will actively try and attack your monster using a special attack. Their attacks are quite powerful and long lasting and can affect your abilities rather than just raw damage. (The idea here is that they have researched a way to stop you)

## Props
These are items that have a limited lifetime in the game.
### Bullet
Simple projectiles from guns and artillery
### Grenade
slow moving projectiles with limited range with an explosion radius
### Rocket
Fast moving projectiles with a large explosion radius - causes the most damage.

## Sets
These things are fixed in place and don't move around. They can be destroyed either by the monster or the actors fighting the monster.
### Building
Just a building.
### Landmark
A special building that has significant points associated with it in the destruction themed game.
### Park
A little bit of green space never hurt anyone. Tends to burn down and spreads fire easily though.
### Power
These buildings can control sectors - take these out and it can hinder Actors from doing things.
### Road
Destroy a road and it makes it hard for vehicles to travel on.
### Water
A water tile. Canal, Rivers or Harbor

## Vehicles
Vehicles allow actors to quickly move around the set. They can drive around normally, trying to escape the monster, or actively getting into position to attack the monster. If a vehicle crashes or becomes immobile then the occupants can escape it and either attack or run away (depending on the actor type)
### Cars
Just your very day car. Multiple colors, shapes and sizes. Contains 1-2 actors. Taxies also fall into this category. Trucks can also be treated like a 'car'.
### Bus
A bus can contain 10-20 actors.
### Emergency
This falls under the Police/Fire/Ambulance category.
### Military
Military vehicles consists of Jeeps, Tanks, and mobile artillery. They tend to fire back at the monster and are positioned accordingly.
### Science
Special vehicle, the science van. This can contain a special attach.

## Aircraft
Just like vehicles, except these are faster, and are not restricted to the ground terrain.
### Civilian Helicopter
More of a nuisance. Most of the civilian types are news reporters. Harmless (may contains food)
### Military Aircraft
These flying machines (helicopters and fast aircraft) can attack the monster, and hard.
### Science Helicopter
Watch out for this one. Attacks from these can be quite deadly.

## Themes
Themes define the type of game that is to be created. Each theme allocates different points to different things. It also helps create the storyline and the end goal (win condition).
### Revenge
You need to destroy or complete a certain target to win the game.
### Destruction
Cause as much destruction as possible - go for the landmarks for extra points.
### Search
You are looking for someone or something. When you find it you win the game.
### Escape
You need to escape the movie set - You win the game by simply leaving the map - don't think the actors will make it easy for you though.
### Lunch
The monster is simply hungry. Most of the points are favored over eating.
### Survival
You get points the longer you stay alive. The game is over when your monster is defeated.

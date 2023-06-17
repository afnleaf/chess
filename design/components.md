# Rust Chess - Components

## Chess backend
### bitboards
	using the 12x64 bit integer board for speed
### gamestate
- possible moves
- piece moves
	- pawn
	- knight
	- bishop
	- rook
	- queen
	- king
- legal moves
- castling rights
- en passant
- pawn moves up
- pawn promotion
- drawing
- stalemate
- checkmate
- whose turn is it
### api
- give our bitboard implementation an interface that is used to access the gamestate variables
### extendable/modable
- chess960
- anything else?
- more board size not really possible unless 128 bit integers or some shit like avx-512

## Chess visualization
### game loop
takes the backend and implements the gamestate along with some type of visualization
- command line game
- output to web brower console
- simple wasm visualization	
### wasm frontend for web
- design of pieces
- board
- user interface design
- make it more like a video game than using the document object model
- look into webGL?
	
## Artificial Intelligence  
### negamax with alpha beta pruning
- already done with python
### neural network
- how to do this over wasm
- weights
- how does it do forward propagation over the web browser
- how do we train it
### coach/move evaluator
- the monetizable aspect
- review the game
- different ai levels
	
## Devops
### dev environment
docker container?
GitHub
issues - need to breakdown everything

### branching conventions
- Branches are created for issues
- Naming convention: "iss#-issueName"
### code review conventions
- Change-based
- Occurs on merge-request
- At least 1 person performs review
- Any relevant communication should occur within the ticket for documentation purposes
- Some questions to ask:
	- Is this code readable?
	- Does the design of the code fit the existing code?
	- Is it easy to understand?
	- Have tests been written?
	- Is it properly commented?
	- See further possible questions: https://sengnotes.socs.uoguelph.ca/development:codereview
- Reviewer leaves a comment/questions on the changes
- The developer resolves all the comments

### commit conventions
#### 1) Specify the type of commit <br>
- feat: The new feature you're adding to a particular application <br>
- fix: A bug fix<br>
- style: Feature and updates related to styling<br>
- refactor: Refactoring a specific section of the codebase<br>
- test: Everything related to testing<br>
- docs: Everything related to documentation<br>
- chore: Regular code maintenance.[ You can also use emojis to represent commit types]<br>

#### 2) Capitalize the subject line and each paragraph

#### 3) Do not end the subject line with a period

#### 4) Use the imperative mood in the subject line
Imperative mood just means “spoken or written as if giving a command or instruction”. A few examples:

- Clean your room
- Close the door
- Take out the trash

A properly formed Git commit subject line should always be able to complete the following sentence:<br> 
If applied, this commit will _your subject line here_

For example:

- If applied, this commit will _refactor subsystem X for readability_
- If applied, this commit will _update getting started documentation_
- If applied, this commit will _remove deprecated methods_

Sources:
- https://cbea.ms/git-commit/
- https://www.freecodecamp.org/news/writing-good-commit-messages-a-practical-guide/

### merge template
- [] All merge requests have gone through the code review process
- [] All issues have their own branch
- [] Commits are not allowed to be squashed
- [] The code passes all unit tests
- [] The code passes the linter
- [] If the new code is based on a previous issue, the link to the related issue is in the merge request


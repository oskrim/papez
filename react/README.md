# ReAct: Synergizing reasoning and acting in language models

Sample output (`text-davinci-003`):
```
Question: In what year was the football club for which Reuven Niemeijer plays founded?
Thought: I need to search Reuven Niemeijer, find the football club that he plays for, then find the year it was founded.
Action: Search[Reuven Niemeijer]
Observation: Reuven Niemeijer (born 27 March 1995) is a Dutch professional footballer who plays as a midfielder for Italian Serie C club Brescia
Thought: Reuven Niemeijer plays for Brescia, so I need to find when Brescia was founded.
Action: Search[Brescia]
Observation: Brescia (Italian pronunciation: [ˈbreʃʃa] (listen), locally [ˈbreːʃa]; Lombard: Brèsa [ˈbrɛsɔ, ˈbrɛhɔ, ˈbrɛsa]; Latin: Brixia; Venetian: Bressa) is a city and comune in the region of Lombardy, in Italy
Thought: Brescia has a long history, but does not mention when it was founded. I need to look up the founding of Brescia.
Action: Lookup[founding of Brescia]
```

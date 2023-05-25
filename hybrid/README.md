# An implementation of some of the ideas in "Out-of-Domain Semantics to the Rescue! Zero-Shot Hybrid Retrieval Models"

Computation of the embeddings (slow, needed only once, cached on subsequent runs)
```
COMPUTE=1 python3 main.py
```
after that
```
python3 main.py
```
works

## Model choice

By default, reciprocal rank fusion (RRF) of a neural and lexical model is used. To run only neural, use
```
python3 main.py --model neural
```
or to run only lexical, use
```
python3 main.py --model neural
```

## Results

### Passages

Currently with commit `4c377f2`, results are as follows. Mean average precision (MAP) is used for evaluation

First 1% of dataset
```
[neural]  MAP: 0.643283397888661
[both]    MAP: 0.38575320877952407
[lexical] MAP: 0.37455950482266237
[random]  MAP: 0.3033065618591935
```

First 5% of dataset
```
[neural]  MAP: 0.6611335781273242
[both]    MAP: 0.42146234087012063
[lexical] MAP: 0.4041820352084331
[random]  MAP: 0.28606920943669834
```

`[random]` denotes a model that assigns a relevancy score randomly from the unit interval for each passage

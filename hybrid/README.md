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

Currently with commit `4c377f2`, results are as follows. Mean reciprocal rank (MRR) is used for evaluation
```
[neural]  MRR: 0.643283397888661
[lexical] MRR: 0.37455950482266237
[both]    MRR: 0.38575320877952407
